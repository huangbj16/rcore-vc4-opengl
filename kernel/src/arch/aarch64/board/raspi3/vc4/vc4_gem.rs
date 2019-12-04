use crate::drivers::gpu::gpu_device::*;
use alloc::vec::Vec;
use alloc::sync::Arc;
use spin::Mutex;
use rcore_fs::vfs::*;
use crate::memory::copy_from_user;
use super::vc4_bo::{roundUp, roundDown, VC4_BO_TYPE_BCL};
use super::vc4_validate::*;

struct drm_vc4_submit_rcl_surface {
	hindex: u32, /* Handle index, or ~0 if not present. */
	offset: u32, /* Offset to start of buffer. */
	/*
	 * Bits for either render config (color_write) or load/store packet.
	 * Bits should all be 0 for MSAA load/stores.
	 */
	bits: u16,
	flags: u16,
}

#[repr(C)]
struct vc4_shader_state {
	addr: u32,
	/* Maximum vertex index referenced by any primitive using this
	 * shader state.
	 */
	max_index: u32,
}

#[repr(C)]
pub struct drm_vc4_submit_cl {
	/* Pointer to the binner command list.
	 *
	 * This is the first set of commands executed, which runs the
	 * coordinate shader to determine where primitives land on the screen,
	 * then writes out the state updates and draw calls necessary per tile
	 * to the tile allocation BO.
	 */
	bin_cl: usize,

	/* Pointer to the shader records.
	 *
	 * Shader records are the structures read by the hardware that contain
	 * pointers to uniforms, shaders, and vertex attributes.  The
	 * reference to the shader record has enough information to determine
	 * how many pointers are necessary (fixed number for shaders/uniforms,
	 * and an attribute count), so those BO indices into bo_handles are
	 * just stored as __u32s before each shader record passed in.
	 */
	shader_rec: u64,

	/* Pointer to uniform data and texture handles for the textures
	 * referenced by the shader.
	 *
	 * For each shader state record, there is a set of uniform data in the
	 * order referenced by the record (FS, VS, then CS).  Each set of
	 * uniform data has a __u32 index into bo_handles per texture
	 * sample operation, in the order the QPU_W_TMUn_S writes appear in
	 * the program.  Following the texture BO handle indices is the actual
	 * uniform data.
	 *
	 * The individual uniform state blocks don't have sizes passed in,
	 * because the kernel has to determine the sizes anyway during shader
	 * code validation.
	 */
	uniforms: u64,
	bo_handles: usize,

	/* Size in bytes of the binner command list. */
	bin_cl_size: u32,
	/* Size in bytes of the set of shader records. */
	shader_rec_size: u32,
	/* Number of shader records.
	 *
	 * This could just be computed from the contents of shader_records and
	 * the address bits of references to them from the bin CL, but it
	 * keeps the kernel from having to resize some allocations it makes.
	 */
	shader_rec_count: u32,
	/* Size in bytes of the uniform state. */
	uniforms_size: u32,

	/* Number of BO handles passed in (size is that times 4). */
	bo_handle_count: u32,

	/* RCL setup: */
	width: u16,
	height: u16,
	min_x_tile: u8,
	min_y_tile: u8,
	max_x_tile: u8,
	max_y_tile: u8,
	color_read: drm_vc4_submit_rcl_surface,
	color_write: drm_vc4_submit_rcl_surface,
	zs_read: drm_vc4_submit_rcl_surface,
	zs_write: drm_vc4_submit_rcl_surface,
	msaa_color_write: drm_vc4_submit_rcl_surface,
	msaa_zs_write: drm_vc4_submit_rcl_surface,
	clear_color : [u32;2],
	clear_z: u32,
	clear_s: u8,

	//__u32 pad:24;

	flags: u32,

	/* Returned value of the seqno of this render job (for the
	 * wait ioctl).
	 */
	seqno: u64,
}

pub struct vc4_exec_info<'a> {
	/* Kernel-space copy of the ioctl arguments */
	args: &'a mut drm_vc4_submit_cl,//struct drm_vc4_submit_cl *

	/* This is the array of BOs that were looked up at the start of exec.
	 * Command validation will use indices into this array.
	 */
	bo: Vec<Arc<Mutex<gpu_bo>>>,//struct vc4_bo **
	bo_count: u32,

	/* List of other BOs used in the job that need to be released
	 * once the job is complete.
	 */
	// TODO
	//unref_list: list_entry_t,
	unref_list: Vec<Arc<Mutex<gpu_bo>>>,

	/* Current unvalidated indices into @bo loaded by the non-hardware
	 * VC4_PACKET_GEM_HANDLES.
	 */
	bo_index: [u32; 2],

	/* This is the BO where we store the validated command lists, shader
	 * records, and uniforms.
	 */
	exec_bo: Option<Arc<Mutex<gpu_bo>>>,//struct vc4_bo *

	shader_state: usize,//struct vc4_shader_state *

	/** How many shader states the user declared they were using. */
	shader_state_size: u32,
	/** How many shader state records the validator has seen. */
	shader_state_count: u32,

	bin_tiles_x: u8,
	bin_tiles_y: u8,
	/* Physical address of the start of the tile alloc array
	 * (where each tile's binned CL will start)
	 */
	tile_alloc_offset: u32,

	/**
	 * Computed addresses pointing into exec_bo where we start the
	 * bin thread (ct0) and render thread (ct1).
	 */
	ct0ca: u32,
	ct0ea: u32,
	ct1ca: u32,
	ct1ea: u32,

	/* Pointer to the unvalidated bin CL (if present). */
	bin_u: usize,

	/* Pointers to the shader recs.  These paddr gets incremented as CL
	 * packets are relocated in validate_gl_shader_state, and the vaddrs
	 * (u and v) get incremented and size decremented as the shader recs
	 * themselves are validated.
	 */
	shader_rec_u: usize,
	shader_rec_v: usize,
	shader_rec_p: u32,
	shader_rec_size: u32,
}

impl GpuDevice {
	pub fn vc4_cl_lookup_bos(&self, exec:&mut vc4_exec_info) -> Result<()>
	{
		exec.bo_count = exec.args.bo_handle_count;

		if exec.bo_count == 0 {
			return Ok(())
		}

		// TODO check correctness
		// exec.fb_bo = self.fb_bo;
		// //??? 2-D array pointer, unsolved.
		// // exec.bo = (struct vc4_bo **)kmalloc(exec->bo_count *
		// // 				     sizeof(struct vc4_bo *));

		let mut handles: Vec<u32> = Vec::new();

		let vbegin = exec.args.bo_handles;
		for i in 0..exec.bo_count {
			let vaddr = vbegin + (i * 4) as usize;
			if let Some(handle) = copy_from_user(vaddr as *const u32) {
				handles.push(handle);
			} else {
				return Err(FsError::InvalidParam)
			}
		}

		for i in 0..exec.bo_count as usize {
			if let Some(bo) = self.vc4_lookup_bo(handles[i]) {
				exec.bo.push(bo.clone());
			} else {
				return Err(FsError::InvalidParam)
			}
		}
		Ok(())
	}

	// Layout
	// -----------
	// bin_cl
	// -----------
	// shader rec size
	// -----------
	// uniforms size
	// -----------
	// shader rec
	// -----------
	pub fn vc4_get_bcl(&mut self, exec: &mut vc4_exec_info) -> Result<()>
	{
		// struct drm_vc4_submit_cl *args = exec->args;

		let bin_offset: u32 = 0;
		let shader_rec_offset: u32 = roundUp(bin_offset + exec.args.bin_cl_size, 16);
		let uniforms_offset = shader_rec_offset + exec.args.shader_rec_size as u32;
		let exec_size = uniforms_offset + exec.args.uniforms_size;
		let temp_size = exec_size + (core::mem::size_of::<vc4_shader_state>() as u32 * exec.args.shader_rec_count);

		if (shader_rec_offset < exec.args.bin_cl_size || uniforms_offset < shader_rec_offset || exec_size < uniforms_offset || temp_size < exec_size) {
			println!("VC4: overflow in exec arguments\n");
			return Err(FsError::InvalidParam)
		}

		//??? allocate a piece of memory for some reason,
		// such operation may not be suitable in rust
		// temp = (void *)kmalloc(temp_size);
		// if (!temp) {
		// 	print!("vc4: Failed to allocate storage for copying "
		// 		"in bin/render CLs.\n");
		// 	ret = -E_NOMEM;
		// 	goto fail;
		// }

		// exec.shader_rec_u = shader_rec_offset;
		// exec.shader_state = exec_size;
		// exec.shader_state_size = exec.args.shader_rec_count;

		// let mut temp: Vec<u8> = Vec::with_capacity(temp_size);

		// let baddr: usize = exec.args.bin_cl;
		// for i in 0..exec.args.bin_cl_size {
		// 	let vaddr = baddr + i;
		// 	if let Some(b) = copy_from_user(vaddr as *const u8) {
		// 		temp[bin_offset + i] = b;
		// 	} else {
		// 		println!("VC4: copt from user error");
		// 		return Err(FsError::InvalidParam)
		// 	}
		// }

		// baddr = exec.args.shader_rec;
		// for i in 0..exec.args.shader_rec_size {
		// 	let vaddr = baddr + i;
		// 	if let Some(b) = copy_from_user(vaddr as *const u8) {
		// 		temp[exec->shader_rec_u + i] = b;
		// 	} else {
		// 		Err(FsError::EFAULT);
		// 	}
		// }

		if let Some(bo) = self.vc4_bo_create(exec_size, VC4_BO_TYPE_BCL) {
			exec.exec_bo =Some(bo.clone());
			let bo_entry = bo.lock();
			exec.ct0ca = bo_entry.paddr as u32 + bin_offset;

			exec.shader_rec_v = bo_entry.vaddr + shader_rec_offset as usize;
			exec.shader_rec_p = bo_entry.paddr + shader_rec_offset;
			exec.shader_rec_size = exec.args.shader_rec_size;
		} else {
			print!("vc4: Couldn't allocate BO for binning\n");
			return Err(FsError::InvalidParam)
		}


		// TODO
		// list_add_before(&exec->unref_list, &exec->exec_bo->unref_head);

		let mut temp: Vec<u8> = Vec::with_capacity(exec.args.bin_cl_size as usize);
		let baddr: usize = exec.args.bin_cl;
		for i in 0..exec.args.bin_cl_size as usize {
			let vaddr = baddr + i;
			if let Some(b) = copy_from_user(vaddr as *const u8) {
				temp[i] = b;
			} else {
				println!("VC4: copy from user error");
				return Err(FsError::InvalidParam)
			}
		}

		self.vc4_validate_bin_cl(exec, &temp)?;
		//vc4_validate_shader_recs(exec)?;

		//TODO
		//list_add_before(&exec.unref_list, &exec.exec_bo.unref_head);
		Ok(())
	}

	pub fn vc4_submit_cl_ioctl(&mut self, data: usize) -> Result<()>
	{
		let args = unsafe { &mut *(data as *mut drm_vc4_submit_cl) };

		let mut exec = vc4_exec_info {
			args: args,
			bo: Vec::new(),
			bo_count: 0,
			unref_list: Vec::new(),

			bo_index: [0, 0],
			
			exec_bo: None,//struct vc4_bo *
			shader_state: 0,//struct vc4_shader_state *
			/** How many shader states the user declared they were using. */
			shader_state_size: 0,
			/** How many shader state records the validator has seen. */
			shader_state_count: 0,

			bin_tiles_x: 0,
			bin_tiles_y: 0,
			tile_alloc_offset: 0,
			ct0ca: 0,
			ct0ea: 0,
			ct1ca: 0,
			ct1ea: 0,
			bin_u: 0,
			shader_rec_u: 0,
			shader_rec_v: 0,
			shader_rec_p: 0,
			shader_rec_size: 0,
		};

		self.vc4_cl_lookup_bos(&mut exec)?;
		// TODO this
		// if ret != 0 {
		// 	vc4_complete_exec(dev, &exec)?;
		// 	ret
		// }

		if exec.args.bin_cl_size != 0 {
			self.vc4_get_bcl(&mut exec)?;
			//TODO clean
			// if ret != 0 {
			// 	vc4_complete_exec(dev, &exec);
			// 	ret
			// }
		} else {
			exec.ct0ca = 0;
			exec.ct0ea = 0;
		}

		// vc4_get_rcl(&exec)?;

		// //TODO for clean
		// //vc4_complete_exec(&exec)?;

		// /* Clear this out of the struct we'll be putting in the queue,
		//  * since it's part of our stack.
		//  */
		// exec.args = Option<None>;
		// vc4_queue_submit(dev, &exec);
		// //TODO for clean
		// //vc4_complete_exec(dev, &exec);
		Ok(())	
	}
}