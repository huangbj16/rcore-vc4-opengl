//#ifndef VC4_DRV_H
//#define VC4_DRV_H

//#include <arm.h>
//#include <dev.h>
//#include <mmu.h>
//#include <list.h>
//#include <types.h>

//#include "vc4_regs.h"
use vc4_regs::*;
use rcore_memory::PAGE_SIZE;
use std::mem;
use crate::syscall::SysError::*;
use vc4_drm::*;

enum vc4_kernel_bo_type {
	VC4_BO_TYPE_FB,
	VC4_BO_TYPE_V3D,
	VC4_BO_TYPE_BIN,
	VC4_BO_TYPE_RCL,
	VC4_BO_TYPE_BCL,
}

struct vc4_bo {
	size: usize,
	handle: u32,
	paddr: u32,
	vaddr: usize,//void*
	bo_type: vc4_kernel_bo_type,
	/* List entry for the BO's position in either
	 * vc4_exec_info->unref_list or vc4_dev->bo_cache.time_list
	 */

	 //TODO
	unref_head: list_entry_t,
}

struct vc4_dev {
	dev: usize,//struct device *

	/* The memory used for storing binner tile alloc, tile state,
	 * and overflow memory allocations.  This is freed when V3D
	 * powers down.
	 */
	//bin_bo: usize,//struct vc4_bo *
	bin_bo: Vec<Arc<Mutex<vc4_bo>>>;

	/* Size of blocks allocated within bin_bo. */
	bin_alloc_size: u32,

	/* Special bo for framebuffer, does not need to be freed. */
	fb_bo: usize,//struct vc4_bo *

	handle_bo_map: BTreeMap<u32, Arc<Mutex<vc4_bo>>>,//struct vc4_bo *
}

pub const VC4_DEV_BUFSIZE: u32 = (2 * PAGE_SIZE - mem::size_of::<vc4_dev>());
pub const VC4_DEV_BO_NENTRY: u32 = (VC4_DEV_BUFSIZE / mem::size_of::<vc4_bo>());

pub fn to_vc4_dev(dev: &device) -> vc4_dev
{
	return dev.driver_data as vc4_dev;
}

/**
	 * This tracks the per-shader-record state (packet 64) that
	 * determines the length of the shader record and the offset
	 * it's expected to be found at.  It gets read in from the
	 * command lists.
	 */
struct vc4_shader_state {
	addr: u32,
	/* Maximum vertex index referenced by any primitive using this
     * shader state.
     */
	max_index: u32,
}

struct vc4_exec_info {
	/* Kernel-space copy of the ioctl arguments */
	args: usize,//struct drm_vc4_submit_cl *

	/* This is the array of BOs that were looked up at the start of exec.
	 * Command validation will use indices into this array.
	 */
	bo: Vec<Arc<Mutex<vc4_bo>>>,//struct vc4_bo **
	fb_bo: usize,//struct vc4_bo *
	bo_count: u32,

	/* List of other BOs used in the job that need to be released
	 * once the job is complete.
	 */
	// TODO
	//unref_list: list_entry_t,

	/* Current unvalidated indices into @bo loaded by the non-hardware
	 * VC4_PACKET_GEM_HANDLES.
	 */
	bo_index: [u32; 2],

	/* This is the BO where we store the validated command lists, shader
	 * records, and uniforms.
	 */
	exec_bo: Arc<Mutex<vc4_bo>>,//struct vc4_bo *

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

impl vc4_exec_info {
	fn new(arg: usize) -> vc4_exec_info {
		vc4_exec_info {
			args: arg,
			bo: Vec::new(),
			fb_bo: 0,//struct vc4_bo *
			bo_count: 0,

			bo_index: [0, 0],
			
			exec_bo: 0,//struct vc4_bo *
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
		}
	}
}

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize }
    }
}

macro_rules! le2bo {
	($le: tt, $member: ident) => {
		$le - offset_of!(vc4_bo, $member)
	}
}

pub fn V3D_READ(offset: u32) -> u32{
	inw(V3D_BASE + offset)
}
pub fn V3D_WRITE(offset: u32, val:u32){
	outw(V3D_BASE + offset, val)
}

/*
int dev_init_vc4();

int vc4_create_bo_ioctl(struct device *dev, void *data);
int vc4_mmap_bo_ioctl(struct device *dev, void *data);
int vc4_free_bo_ioctl(struct device *dev, void *data);
int vc4_submit_cl_ioctl(struct device *dev, void *data);

struct vc4_bo *vc4_bo_create(struct device *dev, size_t size,
			     enum vc4_kernel_bo_type type);
struct vc4_bo *vc4_lookup_bo(struct device *dev, uint32_t handle);
void vc4_bo_destroy(struct device *dev, struct vc4_bo *bo);

/* vc4_validate.c */
int vc4_validate_bin_cl(struct device *dev, void *validated, void *unvalidated,
			struct vc4_exec_info *exec);

int vc4_validate_shader_recs(struct device *dev, struct vc4_exec_info *exec);

struct vc4_bo *vc4_use_bo(struct vc4_exec_info *exec, uint32_t hindex);
int vc4_get_rcl(struct device *dev, struct vc4_exec_info *exec);
*/

//#endif // VC4_DRV_H

/* from vc4_drv.c */

//#include <dev.h>
//#include <inode.h>
//#include <error.h>

use vc4_drm::*;
use crate::drivers::gpu::fb::{self, ColorDepth, ColorFormat, FramebufferInfo, FramebufferResult};
//#include "bcm2708_fb.h"
//#include "mailbox_property.h"

pub fn bo_map_init(bo: &mut vc4_bo)
{
//	memset(bo, 0, sizeof(struct vc4_bo) * VC4_DEV_BO_NENTRY);
	let length = mem::size_of::<vc4_bo>() * VC4_DEV_BO_NENTRY;
	for i in 0..length {
		let bo_offset = bo + i;
		unsafe {*bo_offset = 0 as u8};
	}
}

pub fn vc4_allocate_bin_bo(dev: &mut device) -> i32{//struct device *
	let mut vc4 = to_vc4_dev(dev);
	let &mut size : u32 = 512 * 1024;
	let bo = vc4_bo_create(dev, size, VC4_BO_TYPE_BIN);
	if bo.is_none() {
		error!("vc4_allocate_bin_bo: ERROR_NO_MEMORY");
		ENOMEM
	}

	vc4.bin_bo = bo;
	vc4.bin_alloc_size = size;

	return 0;
}

pub fn vc4_bind_fb_bo(dev: &mut device) -> i32{
	let mut vc4 = to_vc4_dev(dev);

	let fb = get_fb_info();
	if fb.is_none() {
		error!("vc4_allocate_bin_bo: ERROR_NO_DEV");
		ENODEV
	}

	let mut bo = vc4.handle_bo_map[fb.handle] as &mut vc4_bo;
	bo.size = fb.screen_size;
	bo.handle = fb.handle;
	bo.paddr = fb.fb_bus_address;
	bo.vaddr = fb.screen_base;
	bo.bo_type = VC4_BO_TYPE_FB;
	list_init(bo.unref_head);

	vc4.fb_bo = bo as usize;

	return 0;
}

pub fn vc4_probe(mut dev: &mut device) -> i32{
	let mut vc4 = vc4_dev;
	let mut ret = 0;

	assert!((VC4_DEV_BO_NENTRY as i32) > 128);
//	static_assert((int)VC4_DEV_BO_NENTRY > 128);
//	vc4 = (struct vc4_dev *)kmalloc(sizeof(struct vc4_dev) +
//					VC4_DEV_BUFSIZE); ??? can't understand why need to malloc a memory larger than struct needed.
	if vc4.is_none
		ENOMEM

	// The blob now has this nice handy call which powers up the v3d pipeline.
	if (ret = mbox_qpu_enable(1)) != 0 {
		print!("VC4: cannot enable qpu.\n");
		print!("VideoCore IV GPU failed to initialize.\n");
		ret
	}

	if V3D_READ(V3D_IDENT0) != V3D_EXPECTED_IDENT0 {
		print!("VC4: V3D_IDENT0 read 0x{%08x} instead of 0x{%08x}\n",
			V3D_READ(V3D_IDENT0), V3D_EXPECTED_IDENT0);
		print!("VideoCore IV GPU failed to initialize.\n");
		EINVAL
	}

	vc4.dev = dev;
	vc4.handle_bo_map = vc4.bin_bo;
	dev.driver_data = vc4;

	bo_map_init(vc4.handle_bo_map);

	if fb_check() && (ret = vc4_bind_fb_bo(dev)) != 0 {
		print!("VC4: cannot bind framebuffer bo.\n");
		print!("VideoCore IV GPU failed to initialize.\n");
		ret
	}
	if (ret = vc4_allocate_bin_bo(dev))!=0 {
		print!("VC4: cannot alloc bin bo.\n");
		print!("VideoCore IV GPU failed to initialize.\n");
		ret
	}

	print!("VideoCore IV GPU initialized.\n");
	ret

//	goto out;
/* goto statement cannot be used in rust. */
//fail:
//	kfree(vc4);
//	kprintf("VideoCore IV GPU failed to initialize.\n");
//out:
//	return ret;
}

impl vc4_dev {
	fn io_control(&self, op: u32, data: usize) -> Result<()> {
		match op {
			DRM_IOCTL_VC4_SUBMIT_CL => {
				vc4_submit_cl_ioctl(data);
			}
			DRM_IOCTL_VC4_CREATE_BO => {
				vc4_create_bo_ioctl(data);
			}
			DRM_IOCTL_VC4_MMAP_BO => {
				vc4_mmap_bo_ioctl(data);
			}
			DRM_IOCTL_VC4_FREE_BO => {
				vc4_free_bo_ioctl(data);
			}
			_ => {
				Err(FsError::NotSupported)
			}
		}
	}
}

pub fn vc4_device_init(mut dev: &mut device) -> i32
{
	dev = device;

	let mut ret;
	if (ret = vc4_probe(dev)) != 0 {
		return ret;
	}

	dev.d_blocks = 0;
	dev.d_blocksize = 1;
	dev.d_open = vc4_open;
	dev.d_close = vc4_close;
	dev.d_io = NULL_VOP_INVAL;
	dev.d_ioctl = vc4_ioctl;
	dev.d_mmap = NULL_VOP_INVAL;

	ret
}

pub fn dev_init_vc4() -> i32
{
	let mut node;
	let mut ret;
	if (node = dev_create_inode()).is_none() {
		ret = E_NODEV;
		print!("vc4: dev_create_node failed: {%e}\n", ret);
		ret
	}

	if (ret = vc4_device_init(vop_info(node, device))) != 0 {
		print!("vc4: vc4_device_init failed: {%e}\n", ret);
		dev_kill_inode(&mut node);
		ret
	}
	if (ret = vfs_add_dev("gpu0", node, 0)) != 0 {
		print!("vc4: vfs_add_dev failed: {%e}\n", ret);
		dev_kill_inode(&mut node);
		ret
	}

	0
}



