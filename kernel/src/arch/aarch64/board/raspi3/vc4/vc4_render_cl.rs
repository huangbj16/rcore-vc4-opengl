use crate::drivers::gpu::gpu_device::*;
use super::vc4_drm::*;
use rcore_fs::vfs::*;
use super::vc4_packet::*;
use alloc::sync::Arc;
use spin::Mutex;
use super::vc4_gem::{drm_vc4_submit_rcl_surface, vc4_exec_info};
use super::vc4_validate::vc4_use_bo;
use super::vc4_bo::VC4_BO_TYPE_RCL;


pub struct vc4_cl {
	base: usize,
	next: usize,
	reloc_next: usize,
	size: u32,
}

pub fn vc4_set_field(value: u16, shift: u16, mask: u16) -> u16 {
	let fieldvar: u16 = value << shift;
	fieldvar & mask
}

impl vc4_cl {
	pub fn cl_offset(&self) -> usize {
		self.next - self.base
	}

	pub fn cl_u8(&mut self, data: u8) {
		unsafe {
			*(self.next as *mut u8) = data;
		}
		self.next += 1;
	}

	pub fn cl_u16(&mut self, data: u16) {
		unsafe {
			*(self.next as *mut u16) = data;
		}
		self.next += 2;
	}

	pub fn cl_u32(&mut self, data: u32) {
		unsafe {
			*(self.next as *mut u32) = data;
		}
		self.next += 4;
	}

	pub fn vc4_tile_coordinates(&mut self, x: u8, y: u8) {
		self.cl_u8(vc4_packet::VC4_PACKET_TILE_COORDINATES as u8);
		self.cl_u8(x);
		self.cl_u8(y);
	}

	pub fn vc4_store_before_load(&mut self) {
		self.cl_u8(vc4_packet::VC4_PACKET_STORE_TILE_BUFFER_GENERAL as u8);
		self.cl_u16(vc4_set_field(VC4_LOADSTORE_TILE_BUFFER_NONE,
					  VC4_LOADSTORE_TILE_BUFFER_BUFFER_SHIFT, 
					  VC4_LOADSTORE_TILE_BUFFER_BUFFER_MASK) |
				    VC4_STORE_TILE_BUFFER_DISABLE_COLOR_CLEAR |
				    VC4_STORE_TILE_BUFFER_DISABLE_ZS_CLEAR |
				    VC4_STORE_TILE_BUFFER_DISABLE_VG_MASK_CLEAR);
		self.cl_u32(0); /* no address, since we're in None mode */
	}
}

pub struct vc4_rcl_setup {
	pub color_read: Option<Arc<Mutex<gpu_bo>>>,
	pub color_write:Option<Arc<Mutex<gpu_bo>>>,
	pub zs_read: Option<Arc<Mutex<gpu_bo>>>,
	pub zs_write: Option<Arc<Mutex<gpu_bo>>>,

	pub rcl: vc4_cl,
	// pub next_offset: u32,
}

pub fn vc4_get_field(value: u16, shift: u16, mask: u16) -> u8 {
	let ans = (value & mask) >> shift;
	ans as u8
}

const VC4_SUBMIT_CL_USE_CLEAR_COLOR: u32 = 1 << 0;

pub fn emit_tile(exec: &vc4_exec_info, setup: &mut vc4_rcl_setup, x: u8, y: u8, first: bool, last: bool) {
	// drm_vc4_submit_cl *args = exec->args;
	// vc4_cl *rcl = &setup.rcl;
	let has_bin: bool = exec.args.bin_cl_size != 0;

	/* Note that the load doesn't actually occur until the
	 * tile coords packet is processed, and only one load
	 * may be outstanding at a time.
	 */
	if let Some(read_arc) = &setup.color_read {
		let color_entry = read_arc.lock();
		setup.rcl.cl_u8(vc4_packet::VC4_PACKET_LOAD_TILE_BUFFER_GENERAL as u8);
		setup.rcl.cl_u16(exec.args.color_read.bits);
		setup.rcl.cl_u32(color_entry.paddr + exec.args.color_read.offset);
	}

	if let Some(zs_arc) = &setup.zs_read {
		if setup.color_read.is_some() {
			/* Exec previous load. */
			setup.rcl.vc4_tile_coordinates(x, y);
			setup.rcl.vc4_store_before_load();
		}
		let zs_entry = zs_arc.lock();
		setup.rcl.cl_u8(vc4_packet::VC4_PACKET_LOAD_TILE_BUFFER_GENERAL as u8);
		setup.rcl.cl_u16(exec.args.zs_read.bits);
		setup.rcl.cl_u32(zs_entry.paddr + exec.args.zs_read.offset);
	}

	/* Clipping depends on tile coordinates having been
	 * emitted, so we always need one here.
	 */
	setup.rcl.vc4_tile_coordinates(x, y);

	/* Wait for the binner before jumping to the first
	 * tile's lists.
	 */
	if first && has_bin {
		setup.rcl.cl_u8(vc4_packet::VC4_PACKET_WAIT_ON_SEMAPHORE as u8);
	}

	if has_bin {
		setup.rcl.cl_u8(vc4_packet::VC4_PACKET_BRANCH_TO_SUB_LIST as u8);
		setup.rcl.cl_u32(exec.tile_alloc_offset +
			     ((y as u32) * (exec.bin_tiles_x as u32) + (x as u32)) * 32);
	}

	if let Some(zs_arc) = &setup.zs_write {
		let last_tile_write = setup.color_write.is_some();
		let zs_entry = zs_arc.lock();

		setup.rcl.cl_u8(vc4_packet::VC4_PACKET_STORE_TILE_BUFFER_GENERAL as u8);
		if last_tile_write {
			setup.rcl.cl_u16(exec.args.zs_write.bits | 0);
		} else {
			setup.rcl.cl_u16(exec.args.zs_write.bits | VC4_STORE_TILE_BUFFER_DISABLE_COLOR_CLEAR);
		}
		if last && last_tile_write {
			setup.rcl.cl_u32((zs_entry.paddr + exec.args.zs_write.offset) | VC4_LOADSTORE_TILE_BUFFER_EOF);
		} else {
			setup.rcl.cl_u32((zs_entry.paddr + exec.args.zs_write.offset) | 0);
		}
	}

	if setup.color_write.is_some() {
		if setup.zs_write.is_some() {
			/* Reset after previous store */
			setup.rcl.vc4_tile_coordinates(x, y);
		}

		if last {
			setup.rcl.cl_u8(vc4_packet::VC4_PACKET_STORE_MS_TILE_BUFFER_AND_EOF as u8);
		} else {
			setup.rcl.cl_u8(vc4_packet::VC4_PACKET_STORE_MS_TILE_BUFFER as u8);
		}
	}
}

impl GpuDevice {
	fn vc4_rcl_surface_setup(&mut self, exec: &mut vc4_exec_info, obj: &mut Option<Arc<Mutex<gpu_bo>>>, surf: drm_vc4_submit_rcl_surface) -> Result<()>
	{
		let tiling =
			vc4_get_field(surf.bits, VC4_LOADSTORE_TILE_BUFFER_TILING_SHIFT, VC4_LOADSTORE_TILE_BUFFER_TILING_MASK);
		let buffer =
			vc4_get_field(surf.bits, VC4_LOADSTORE_TILE_BUFFER_BUFFER_SHIFT, VC4_LOADSTORE_TILE_BUFFER_BUFFER_MASK);
		let format =
			vc4_get_field(surf.bits, VC4_LOADSTORE_TILE_BUFFER_FORMAT_SHIFT, VC4_LOADSTORE_TILE_BUFFER_FORMAT_MASK);
		let mut cpp: i32;

		if surf.hindex == (!0) {
			return Ok(())
		}

		if let Some(tmp) = vc4_use_bo(exec, surf.hindex) {
			*obj = Some(tmp.clone());
			if (surf.bits & (!(VC4_LOADSTORE_TILE_BUFFER_TILING_MASK |
					   VC4_LOADSTORE_TILE_BUFFER_BUFFER_MASK |
					   VC4_LOADSTORE_TILE_BUFFER_FORMAT_MASK))) != 0x0 {
				println!("vc4: Unknown bits in load/store: 0x{:04x}\n", surf.bits);
				return Err(FsError::InvalidParam)
			}

			if (tiling > VC4_TILING_FORMAT_LT) {
				print!("vc4: Bad tiling format\n");
				return Err(FsError::InvalidParam)
			}

			if (buffer as u16 == VC4_LOADSTORE_TILE_BUFFER_ZS) {
				if (format != 0) {
					print!("vc4: No color format should be set for ZS\n");
					return Err(FsError::InvalidParam)
				}
				cpp = 4;
			} else if (buffer as u16 == VC4_LOADSTORE_TILE_BUFFER_COLOR) {
				match format as u16 {
					VC4_LOADSTORE_TILE_BUFFER_BGR565 | VC4_LOADSTORE_TILE_BUFFER_BGR565_DITHER => 
						cpp = 2,
					VC4_LOADSTORE_TILE_BUFFER_RGBA8888 => 
						cpp = 4,
					_ => {
						print!("vc4: Bad tile buffer format\n");
						return Err(FsError::InvalidParam)
					}
				}
			} else {
				print!("vc4: Bad load/store buffer {}.\n", buffer);
				return Err(FsError::InvalidParam)
			}

			if (surf.offset & 0xf) != 0x0 {
				print!("vc4: load/store buffer must be 16b aligned.\n");
				return Err(FsError::InvalidParam)
			}

			return Ok(())
		} else {
			return Err(FsError::InvalidParam)
		}
	}

	pub fn vc4_rcl_render_config_surface_setup(&mut self, exec:&mut vc4_exec_info, obj: &mut Option<Arc<Mutex<gpu_bo>>>, surf: drm_vc4_submit_rcl_surface) -> Result<()>
	{
		let tiling = vc4_get_field(surf.bits, VC4_RENDER_CONFIG_MEMORY_FORMAT_SHIFT, VC4_RENDER_CONFIG_MEMORY_FORMAT_MASK);
		let  format = vc4_get_field(surf.bits, VC4_RENDER_CONFIG_FORMAT_SHIFT, VC4_RENDER_CONFIG_FORMAT_MASK);
		let mut cpp: i32;

		if (surf.bits & (!(VC4_RENDER_CONFIG_MEMORY_FORMAT_MASK | VC4_RENDER_CONFIG_FORMAT_MASK))) != 0x0 {
			println!("vc4: Unknown bits in render config: 0x{:x}\n",
				surf.bits);
			return Err(FsError::InvalidParam)
		}

		if (surf.hindex == (!0)) {
			return Ok(())
		}

		if let Some(tmp) = vc4_use_bo(exec, surf.hindex) {
			*obj = Some(tmp.clone());
			if (tiling > VC4_TILING_FORMAT_LT) {
				print!("vc4: Bad tiling format\n");
				return Err(FsError::InvalidParam)
			}

			match format as u16 {
				VC4_RENDER_CONFIG_FORMAT_BGR565_DITHERED | VC4_RENDER_CONFIG_FORMAT_BGR565 => 
					cpp = 2,
				VC4_RENDER_CONFIG_FORMAT_RGBA8888 => 
					cpp = 4,
				_ => {
					print!("vc4: Bad tile buffer format\n");
					return Err(FsError::InvalidParam)
				}
			}

			return Ok(())
		} else {
			return Err(FsError::InvalidParam)
		}
	}

	pub fn vc4_create_rcl_bo(&mut self, exec: &mut vc4_exec_info, setup: &mut vc4_rcl_setup) -> Result<()>
	{
		// drm_vc4_submit_cl *args = exec->args;
		let has_bin = exec.args.bin_cl_size != 0;
		let min_x_tile = exec.args.min_x_tile;
		let min_y_tile = exec.args.min_y_tile;
		let max_x_tile = exec.args.max_x_tile;
		let max_y_tile = exec.args.max_y_tile;
		let xtiles = max_x_tile - min_x_tile + 1;
		let ytiles = max_y_tile - min_y_tile + 1;

		let mut size = VC4_PACKET_TILE_RENDERING_MODE_CONFIG_SIZE;
		let mut loop_body_size = VC4_PACKET_TILE_COORDINATES_SIZE;

		if (exec.args.flags & VC4_SUBMIT_CL_USE_CLEAR_COLOR) != 0x0 {
			size += VC4_PACKET_CLEAR_COLORS_SIZE + VC4_PACKET_TILE_COORDINATES_SIZE + VC4_PACKET_STORE_TILE_BUFFER_GENERAL_SIZE;
		}

		if (setup.color_read.is_some()) {
			loop_body_size += VC4_PACKET_LOAD_TILE_BUFFER_GENERAL_SIZE;
		}
		if setup.zs_read.is_some() {
			if setup.color_read.is_some() {
				loop_body_size += VC4_PACKET_TILE_COORDINATES_SIZE;
				loop_body_size +=VC4_PACKET_STORE_TILE_BUFFER_GENERAL_SIZE;
			}
			loop_body_size += VC4_PACKET_LOAD_TILE_BUFFER_GENERAL_SIZE;
		}

		if has_bin {
			size += VC4_PACKET_WAIT_ON_SEMAPHORE_SIZE;
			loop_body_size += VC4_PACKET_BRANCH_TO_SUB_LIST_SIZE;
		}

		if setup.zs_write.is_some() {
			loop_body_size += VC4_PACKET_STORE_TILE_BUFFER_GENERAL_SIZE;
		}

		if setup.color_write.is_some() {
			loop_body_size += VC4_PACKET_STORE_MS_TILE_BUFFER_SIZE;
		}

		/* We need a VC4_PACKET_TILE_COORDINATES in between each store. */
		let temp1: u32;
		let temp2: u32;
		
		if setup.color_write.is_some() {
			temp1 = 1;
		} else {
		 	temp1 = 0;
		}

		if setup.zs_write.is_some() {
			temp2 = 1;
		} else {
			temp2 = 0;
		}

		loop_body_size += VC4_PACKET_TILE_COORDINATES_SIZE * (temp1 + temp2 - 1);

		size += (xtiles as u32) * (ytiles as u32) * loop_body_size;

		if let Some(rcl_bo) = self.vc4_bo_create(size, VC4_BO_TYPE_RCL) {
			let bo_entry = rcl_bo.lock();
			setup.rcl.base = bo_entry.vaddr;
			setup.rcl.next = bo_entry.vaddr;
			setup.rcl.size = size;
			//TODO exec.unref_list.push(rcl_bo.clone());
			//list_add_before(&exec.unref_list, &rcl_bo.unref_head);

			/* The tile buffer gets cleared when the previous tile is stored.  If
			 * the clear values changed between frames, then the tile buffer has
			 * stale clear values in it, so we have to do a store in None mode (no
			 * writes) so that we trigger the tile buffer clear.
			 */
			if (exec.args.flags & VC4_SUBMIT_CL_USE_CLEAR_COLOR) != 0x0 {
				setup.rcl.cl_u8(vc4_packet::VC4_PACKET_CLEAR_COLORS as u8);
				setup.rcl.cl_u32(exec.args.clear_color[0]);
				setup.rcl.cl_u32(exec.args.clear_color[1]);
				setup.rcl.cl_u32(exec.args.clear_z);
				setup.rcl.cl_u8(exec.args.clear_s);

				setup.rcl.vc4_tile_coordinates(0, 0);

				setup.rcl.cl_u8(vc4_packet::VC4_PACKET_STORE_TILE_BUFFER_GENERAL as u8);
				setup.rcl.cl_u16(VC4_LOADSTORE_TILE_BUFFER_NONE);
				setup.rcl.cl_u32(0); /* no address, since we're in None mode */
			}

			setup.rcl.cl_u8(vc4_packet::VC4_PACKET_TILE_RENDERING_MODE_CONFIG as u8);
			

			if let Some(color_arc) = &setup.color_write {
				//dereference color_arc
				let color_entry = color_arc.lock();
				setup.rcl.cl_u32((color_entry.paddr + exec.args.color_write.offset));
			} else {
				setup.rcl.cl_u32(0);
			}

			setup.rcl.cl_u16(exec.args.width);
			setup.rcl.cl_u16(exec.args.height);
			setup.rcl.cl_u16(exec.args.color_write.bits);

			for y in min_y_tile..(max_y_tile + 1) {
				for x in min_x_tile..(max_x_tile + 1) {
					let first = x == min_x_tile && y == min_y_tile;
					let last = x == max_x_tile && y == max_y_tile;
					emit_tile(&exec, setup, x, y, first, last);
				}
			}

			assert!(setup.rcl.cl_offset() == (size as usize));
			exec.ct1ca = bo_entry.paddr;
			exec.ct1ea = bo_entry.paddr + setup.rcl.cl_offset() as u32;
			Ok(())
		} else {
			return Err(FsError::InvalidParam)
		}
	}

	pub fn vc4_get_rcl(&mut self, exec: &mut vc4_exec_info) -> Result<()>
	{
		let has_bin = (exec.args.bin_cl_size != 0) as bool;

		if (exec.args.min_x_tile > exec.args.max_x_tile ||
		    exec.args.min_y_tile > exec.args.max_y_tile) {
			println!("vc4: Bad render tile set ({},{})-({},{})",
				exec.args.min_x_tile, exec.args.min_y_tile, exec.args.max_x_tile,
				exec.args.max_y_tile);
			return Err(FsError::InvalidParam)
		}

		if (has_bin && (exec.args.max_x_tile > exec.bin_tiles_x ||
				exec.args.max_y_tile > exec.bin_tiles_y)) {
			println!("vc4: Render tiles ({},{}) outside of bin config ({},{})",
				exec.args.max_x_tile, exec.args.max_y_tile, exec.bin_tiles_x,
				exec.bin_tiles_y);
			return Err(FsError::InvalidParam)
		}

		let mut setup = vc4_rcl_setup {
			color_read: None,
			color_write: None,
			zs_read: None,
			zs_write: None,
			rcl: vc4_cl {
				base: 0,
				next: 0,
				reloc_next: 0,
				size: 0,
			},
		};

		self.vc4_rcl_surface_setup(exec, &mut setup.color_read, exec.args.color_read.clone())?;
		self.vc4_rcl_render_config_surface_setup(exec, &mut setup.color_write, exec.args.color_write.clone())?;

		// vc4_rcl_surface_setup_zsread(setup, exec)?;
		self.vc4_rcl_surface_setup(exec, &mut setup.zs_read, exec.args.zs_read.clone())?;
		self.vc4_rcl_surface_setup(exec, &mut setup.zs_write, exec.args.zs_write.clone())?;

		/* We shouldn't even have the job submitted to us if there's no
		 * surface to write out.
		 */
		if (setup.color_write.is_none() && setup.zs_write.is_none()) {
			print!("vc4: RCL requires color or Z/S write\n");
			return Err(FsError::InvalidParam)
		}

		return self.vc4_create_rcl_bo(exec, &mut setup)
	}
}