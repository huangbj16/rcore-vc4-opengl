use crate::drivers::gpu::gpu_device::*;
use super::vc4_gem::vc4_exec_info;
use alloc::vec::Vec;
use rcore_fs::vfs::*;
use alloc::collections::BTreeMap;
use lazy_static::lazy_static;
use super::vc4_packet::*;
use core::mem::transmute;
use spin::Mutex;
use alloc::sync::Arc;


pub struct Cmd_info {
	len: u32,
	func: Option<fn(exec: &mut vc4_exec_info, validated: usize, untrusted: &[u8], bin_paddr: u32) -> Result<()>>,
}

pub fn vc4_use_bo(exec: &mut vc4_exec_info, hindex: u32) -> Option<Arc<Mutex<gpu_bo>>>
{
	if (hindex >= exec.bo_count) {
		println!("vc4: BO index {} greater than BO count {}\n", hindex, exec.bo_count);
		None
	}

	Some(exec.bo[hindex].clone())
}

pub fn vc4_set_field(value: u8, shift: u8, mask: u8) {
	let fieldvar: u8 = value << shift;
	fieldvar & mask
}

pub fn vc4_use_handle(exec: &mut vc4_exec_info, gem_handles_packet_index: u32) -> Option<Arc<Mutex<gpu_bo>>>
{
	vc4_use_bo(&exec, exec.bo_index[gem_handles_packet_index])
}

pub fn validate_indexed_prim_list(exec: &mut vc4_exec_info, validated: usize, untrusted: &[u8], bin_paddr: u32) -> Result<()>
{
	if (untrusted.len() < 13) {
		return Err(FsError::InvalidParam)
	}
	let length: u32 = transmute(&untrusted[1..5]);
	let offset: u32 = transmute(&untrusted[5..9]);
	let max_index: u32 = transmute(&untrusted[9..13]);
	let index_size: u32;
	if (untrusted[0] >> 4) == 1 {
		index_size = 2;
	} else {
		index_size = 1;
	}

	/* Check overflow condition */
	if (exec.shader_state_count == 0) {
		print!("vc4: shader state must precede primitives\n");
		return Err(FsError::InvalidParam)
	}
	let &mut shader_state = exec.shader_state[exec.shader_state_count - 1];

	if (max_index > shader_state.max_index) {
		shader_state.max_index = max_index;
	}

	if let Some(ib) = vc4_use_handle(&exec, 0) {
		let ib_entry = ib.lock();
		if (offset > ib_entry.size || ((ib_entry.size - offset) / index_size) < length) {
			print!("vc4: IB access overflow ({} + {}*{} > {})\n", offset, length, index_size, ib_entry.size);
			return Err(FsError::InvalidParam)
		}
		unsafe { *((validated + 14) as *mut u32) = ib_entry.paddr + offset; }
		//put_unaligned_32(validated + 5, ib.paddr + offset);
		Ok(())
	} else {
		return Err(FsError::InvalidParam)
	}
}

pub fn validate_gl_array_primitive(exec: &mut vc4_exec_info, validated: usize, untrusted: &[u8], bin_paddr: u32) -> Result<()>
{
	let length: u32 = transmute(&untrusted[1..5]);
	let base_index: u32 = transmute(&untrusted[5..9]);

	/* Check overflow condition */
	if (exec.shader_state_count == 0) {
		println!("vc4: shader state must precede primitives");
		return Err(FsError::InvalidParam)
	}

	if (length + base_index < length) {
		println!("vc4: primitive vertex count overflow");
		return Err(FsError::InvalidParam)
	}
	let max_index = length + base_index - 1;
	let &mut shader_state = exec.shader_state[exec.shader_state_count - 1];

	if (max_index > shader_state.max_index) {
		shader_state.max_index = max_index;
	}
	Ok(())
}

pub fn validate_nv_shader_state(exec: &mut vc4_exec_info, validated: usize, untrusted: &[u8], bin_paddr: u32) -> Result<()>
{
	let i = exec.shader_state_count;
	exec.shader_state_count += 1;

	if (i >= exec.shader_state_size) {
		println!("vc4: More requests for shader states than declared");
		return Err(FsError::InvalidParam)
	}

	exec.shader_state[i].addr = transmute(&untrusted[0..4]);
	exec.shader_state[i].max_index = 0;

	unsafe { *(validated as *mut u32) = exec.shader_rec_p + exec.shader_state[i].addr; }
	//put_unaligned_32(&validated, (exec.shader_rec_p + exec.shader_state[i].addr));

	exec.shader_rec_p += 16;

	Ok(())
}

pub fn validate_tile_binning_config(exec: &mut vc4_exec_info, validated: usize, untrusted: &[u8], bin_paddr: u32) -> Result<()>
{
	let tile_count: u32;

	//???怎么只取指定的8个bits?
	exec.bin_tiles_x = transmute(&untrusted[12..13]);
	exec.bin_tiles_y = transmute(&untrusted[13..14]);
	tile_count = exec.bin_tiles_x * exec.bin_tiles_y;
	let flags: u8 = transmute(&untrusted[14..15]);

	/* The tile state data array is 48 bytes per tile, and we put it at
	 * the start of a BO containing both it and the tile alloc.
	 */
	let tile_state_size: u32 = 48 * tile_count;

	unsafe {
		*((validated + 14) as *mut u8) =
			((flags & (!(VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_MASK | VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_MASK))) |
			 VC4_BIN_CONFIG_AUTO_INIT_TSDA |
			 vc4_set_field(VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_32,
				       VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_SHIFT, VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_MASK) |
			 vc4_set_field(VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_128,
				       VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_SHIFT, VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_MASK));
	}

	/* Since the tile alloc array will follow us, align. */
	exec.tile_alloc_offset = bin_addr + roundUp(tile_state_size, PAGE_SIZE);

	/* tile alloc address. */
	unsafe { *(validated as *mut u32) = exec.tile_alloc_offset; }
	unsafe { *((validated + 4) as *mut u32) = bin_paddr + vc4.bin_alloc_size -  exec.tile_alloc_offset; }
	unsafe { *((validated + 4) as *mut u32) = bin_paddr; }

	Ok(())
}

pub fn validate_gem_handles(exec: &mut vc4_exec_info, validated: usize, untrusted: &[u8], bin_paddr: u32)  -> Result<()>
{
	// memcpy(exec.bo_index, untrusted, sizeof(exec.bo_index));
	for i in 0..exec.bo_index.len() {
		exec.bo_index[i] = transmute(&untrusted[(i * 4)..(i * 4 + 4)]);
	}
	Ok(())
}

lazy_static! {
	pub static ref CMD_INFO: BTreeMap<u8, Cmd_info> = {
		let mut m = BTreeMap::new();
		m.insert(vc4_packet::VC4_PACKET_HALT as u8, Cmd_info {len: VC4_PACKET_HALT_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_NOP as u8, Cmd_info {len: VC4_PACKET_NOP_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_FLUSH as u8, Cmd_info {len: VC4_PACKET_FLUSH_SIZE, func: None}); // validate_flush
		m.insert(vc4_packet::VC4_PACKET_FLUSH_ALL as u8, Cmd_info {len: VC4_PACKET_FLUSH_ALL_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_START_TILE_BINNING as u8, Cmd_info {len: VC4_PACKET_START_TILE_BINNING_SIZE, func: None}); // validate_start_tile_binning
		m.insert(vc4_packet::VC4_PACKET_INCREMENT_SEMAPHORE as u8, Cmd_info {len: VC4_PACKET_INCREMENT_SEMAPHORE_SIZE, func: None}); // validate_increment_semaphore

		m.insert(vc4_packet::VC4_PACKET_GL_INDEXED_PRIMITIVE as u8, Cmd_info {len: VC4_PACKET_GL_INDEXED_PRIMITIVE_SIZE, func: validate_indexed_prim_list});
		m.insert(vc4_packet::VC4_PACKET_GL_ARRAY_PRIMITIVE as u8, Cmd_info {len: VC4_PACKET_GL_ARRAY_PRIMITIVE_SIZE, func: validate_gl_array_primitive});

		m.insert(vc4_packet::VC4_PACKET_PRIMITIVE_LIST_FORMAT as u8, Cmd_info {len: VC4_PACKET_PRIMITIVE_LIST_FORMAT_SIZE, func: None});

		m.insert(vc4_packet::VC4_PACKET_GL_SHADER_STATE as u8, Cmd_info {len: VC4_PACKET_GL_SHADER_STATE_SIZE, func: None}); // validate_gl_shader_state
		m.insert(vc4_packet::VC4_PACKET_NV_SHADER_STATE as u8, Cmd_info {len: VC4_PACKET_NV_SHADER_STATE_SIZE, func: validate_nv_shader_state});

		m.insert(vc4_packet::VC4_PACKET_CONFIGURATION_BITS as u8, Cmd_info {len: VC4_PACKET_CONFIGURATION_BITS_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_FLAT_SHADE_FLAGS as u8, Cmd_info {len: VC4_PACKET_FLAT_SHADE_FLAGS_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_POINT_SIZE as u8, Cmd_info {len: VC4_PACKET_POINT_SIZE_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_LINE_WIDTH as u8, Cmd_info {len: VC4_PACKET_LINE_WIDTH_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_RHT_X_BOUNDARY as u8, Cmd_info {len: VC4_PACKET_RHT_X_BOUNDARY_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_DEPTH_OFFSET as u8, Cmd_info {len: VC4_PACKET_DEPTH_OFFSET_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_CLIP_WINDOW as u8, Cmd_info {len: VC4_PACKET_CLIP_WINDOW_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_VIEWPORT_OFFSET as u8, Cmd_info {len: VC4_PACKET_VIEWPORT_OFFSET_SIZE, func: None});
		m.insert(vc4_packet::VC4_PACKET_CLIPPER_XY_SCALING as u8, Cmd_info {len: VC4_PACKET_CLIPPER_XY_SCALING_SIZE, func: None});
		/* Note: The docs say this was also 105, but it was 106 in the
		 * initial userland code drop.
		 */
		m.insert(vc4_packet::VC4_PACKET_CLIPPER_Z_SCALING as u8, Cmd_info {len: VC4_PACKET_CLIPPER_Z_SCALING_SIZE, func: None});

		m.insert(vc4_packet::VC4_PACKET_TILE_BINNING_MODE_CONFIG as u8, Cmd_info {len: VC4_PACKET_TILE_BINNING_MODE_CONFIG_SIZE, func: validate_tile_binning_config});

		m.insert(vc4_packet::VC4_PACKET_GEM_HANDLES as u8, Cmd_info {len: VC4_PACKET_GEM_HANDLES_SIZE, func: validate_gem_handles});
		m
	};
}

impl GpuDevice {
	pub fn vc4_validate_bin_cl(&self, exec: &mut vc4_exec_info, bin_start_addr: usize, src: & Vec<u8>) -> Result<()>
	{
		let len = exec.args.bin_cl_size;
		let mut dst_offset = 0;
		let mut src_offset = 0;

		let bin_paddr: u32 = 0;
		if let Some(bin_bo) = self.bin_bo {
			let bo_entry = bin_bo.lock();
			bin_paddr = bin_bo.paddr;
		} else {
			return Err(FsError::InvalidParam)
		}

		while src_offset < len {
			let mut dst_pkt = bin_start_addr + dst_offset;

			let cmd: u8 = src[src_offset as usize];

			// if cmd >= CMD_INFO.len() {
			// 	println!("vc4: 0x{#08x}: packet {} out of bounds", src_offset, cmd);
			// 	return Err(FsError::InvalidParam)
			// }

			if let Some(info) = CMD_INFO.get(&cmd) {
				// if (info.name.isempty()) {
				// 	println!("vc4: 0x{08x}: packet {} invalid", src_offset, cmd);
				// 	return Err(FsError::InvalidParam)
				// }

				if (src_offset + info.len as u32 > len) {
					println!("vc4: 0x{:08x}: packet {} ({}) length 0x{:08x} exceeds bounds (0x{:08x})",
						src_offset, cmd, info.name, info.len,
						src_offset + len);
					return Err(FsError::InvalidParam)
				}

				if (cmd != vc4_packet::VC4_PACKET_GEM_HANDLES as u8) {
					for i in 0..info.len as usize {
						unsafe { *((dst_pkt + i) as *mut u8) = src[src_offset as usize + i]; }
					}
					//memcpy(dst_pkt, src_pkt, info.len);
				}

				// TODO
				if let Some(func) = info.func {
					func(exec, dst_pkt + 1, &src[(src_offset + 1)..], bin_paddr);
				}

				src_offset += info.len;
				/* GEM handle loading doesn't produce HW packets. */
				if (cmd != vc4_packet::VC4_PACKET_GEM_HANDLES as u8) {
					dst_offset += info.len as usize;
				}

				/* When the CL hits halt, it'll stop reading anything else. */
				if (cmd == vc4_packet::VC4_PACKET_HALT as u8) {
					break;
				}
			}
		}
		exec.ct0ea = exec.ct0ca + dst_offset as u32;

		Ok(())
	}
}
