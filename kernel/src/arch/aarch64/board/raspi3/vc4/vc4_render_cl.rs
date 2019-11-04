use vc4_cl::*;
use vc4_drv::*;
use vc4_drm::*;
use vc4_packet::*;
use mailbox::*;
use core::mem::size_of;
use crate::syscall::SysError::*;

struct vc4_rcl_setup {
	let mut color_read: vc4_bo;
	let mut color_write: vc4_bo;
	let mut zs_read: vc4_bo;
	let mut zs_write: vc4_bo;

	let mut rcl: vc4_cl;
	let mut next_offset: u32;
}

/*
 * Emits a no-op STORE_TILE_BUFFER_GENERAL.
 *
 * If we emit a PACKET_TILE_COORDINATES, it must be followed by a store of
 * some sort before another load is triggered.
 */
pub fn vc4_store_before_load(rcl: &vc4_cl)
{
	cl_u8(&rcl, VC4_PACKET_STORE_TILE_BUFFER_GENERAL);
	cl_u16(&rcl, VC4_SET_FIELD(VC4_LOADSTORE_TILE_BUFFER_NONE, VC4_LOADSTORE_TILE_BUFFER_BUFFER) | VC4_STORE_TILE_BUFFER_DISABLE_COLOR_CLEAR | VC4_STORE_TILE_BUFFER_DISABLE_ZS_CLEAR | VC4_STORE_TILE_BUFFER_DISABLE_VG_MASK_CLEAR);
	cl_u32(&rcl, 0); /* no address, since we're in None mode */
}

/*
 * Emits a PACKET_TILE_COORDINATES if one isn't already pending.
 *
 * The tile coordinates packet triggers a pending load if there is one, are
 * used for clipping during rendering, and determine where loads/stores happen
 * relative to their base address.
 */
pub fn vc4_tile_coordinates(rcl: &vc4_cl, x: u32, y: u32)
{
	cl_u8(&rcl, VC4_PACKET_TILE_COORDINATES);
	cl_u8(&rcl, x);
	cl_u8(&rcl, y);
}

pub fn emit_tile(exec: &vc4_exec_info, setup: &vc4_rcl_setup, x: u32, y: u32, first: bool, last: bool)
{
	// drm_vc4_submit_cl *args = exec->args;
	// vc4_cl *rcl = &setup.rcl;
	let has_bin: bool = (exec.args.bin_cl_size != 0) as bool;

	/* Note that the load doesn't actually occur until the
	 * tile coords packet is processed, and only one load
	 * may be outstanding at a time.
	 */
	if !setup.color_read.is_None() {
		cl_u8(&setup.rcl, VC4_PACKET_LOAD_TILE_BUFFER_GENERAL);
		cl_u16(&setup.rcl, exec.args.color_read.bits);
		cl_u32(&setup.rcl, setup.color_read.paddr + exec.args.color_read.offset);
	}

	if !setup.zs_read.is_None() {
		if !setup.color_read.is_None() {
			/* Exec previous load. */
			vc4_tile_coordinates(&setup.rcl, x, y);
			vc4_store_before_load(&setup.rcl);
		}

		cl_u8(&setup.rcl, VC4_PACKET_LOAD_TILE_BUFFER_GENERAL);
		cl_u16(&setup.rcl, exec.args.zs_read.bits);
		cl_u32(&setup.rcl, setup.zs_read.paddr + exec.args.zs_read.offset);
	}

	/* Clipping depends on tile coordinates having been
	 * emitted, so we always need one here.
	 */
	vc4_tile_coordinates(&setup.rcl, x, y);

	/* Wait for the binner before jumping to the first
	 * tile's lists.
	 */
	if first && has_bin
		cl_u8(&setup.rcl, VC4_PACKET_WAIT_ON_SEMAPHORE);

	if has_bin {
		cl_u8(&setup.rcl, VC4_PACKET_BRANCH_TO_SUB_LIST);
		cl_u32(&setup.rcl, (exec.tile_alloc_offset +
			     (y * exec.bin_tiles_x + x) * 32));
	}

	if !setup.zs_write.is_None() {
		let last_tile_write = (!setup.color_write.is_None()) as bool;

		cl_u8(&setup.rcl, VC4_PACKET_STORE_TILE_BUFFER_GENERAL);
		if last_tile_write 
			cl_u16(&setup.rcl, exec.args.zs_write.bits | 0);
		else
			cl_u16(&setup.rcl, exec.args.zs_write.bits | VC4_STORE_TILE_BUFFER_DISABLE_COLOR_CLEAR);
		if last && last_tile_write
			cl_u32(&setup.rcl, (setup.zs_write.paddr + exec.args.zs_write.offset) | VC4_LOADSTORE_TILE_BUFFER_EOF);
		else 
			cl_u32(&setup.rcl, (setup.zs_write.paddr + exec.args.zs_write.offset) | 0);
	}

	if !setup.color_write.is_None() {
		if !setup.zs_write.is_None() {
			/* Reset after previous store */
			vc4_tile_coordinates(&setup.rcl, x, y);
		}

		if last
			cl_u8(&setup.rcl, VC4_PACKET_STORE_MS_TILE_BUFFER_AND_EOF);
		else
			cl_u8(&setup.rcl, VC4_PACKET_STORE_MS_TILE_BUFFER);
	}
}

pub fn vc4_create_rcl_bo(dev: &device, exec: &vc4_exec_info, setup: &vc4_rcl_setup) -> i32
{
	// drm_vc4_submit_cl *args = exec->args;
	let has_bin = (exec.args.bin_cl_size != 0) as bool;
	let min_x_tile = exec.args.min_x_tile as u8;
	let min_y_tile = exec.args.min_y_tile as u8;
	let max_x_tile = exec.args.max_x_tile as u8;
	let max_y_tile = exec.args.max_y_tile as u8;
	let xtiles = (max_x_tile - min_x_tile + 1) as u8;
	let ytiles = (max_y_tile - min_y_tile + 1) as u8;

	let mut size = VC4_PACKET_TILE_RENDERING_MODE_CONFIG_SIZE;
	let mut loop_body_size = VC4_PACKET_TILE_COORDINATES_SIZE;

	if exec.args.flags & VC4_SUBMIT_CL_USE_CLEAR_COLOR {
		size += VC4_PACKET_CLEAR_COLORS_SIZE + VC4_PACKET_TILE_COORDINATES_SIZE + VC4_PACKET_STORE_TILE_BUFFER_GENERAL_SIZE;
	}

	if (setup.color_read) {
		loop_body_size += VC4_PACKET_LOAD_TILE_BUFFER_GENERAL_SIZE;
	}
	if setup.zs_read {
		if setup.color_read {
			loop_body_size += VC4_PACKET_TILE_COORDINATES_SIZE;
			loop_body_size +=VC4_PACKET_STORE_TILE_BUFFER_GENERAL_SIZE;
		}
		loop_body_size += VC4_PACKET_LOAD_TILE_BUFFER_GENERAL_SIZE;
	}

	if has_bin {
		size += VC4_PACKET_WAIT_ON_SEMAPHORE_SIZE;
		loop_body_size += VC4_PACKET_BRANCH_TO_SUB_LIST_SIZE;
	}

	if setup.zs_write
		loop_body_size += VC4_PACKET_STORE_TILE_BUFFER_GENERAL_SIZE;
	if setup.color_write
		loop_body_size += VC4_PACKET_STORE_MS_TILE_BUFFER_SIZE;

	/* We need a VC4_PACKET_TILE_COORDINATES in between each store. */
	let temp1: u32;
	let temp2: u32;
	if setup.color_write 
		temp1 = 1;
	else 
		temp1 = 0;
	if setup.zs_write
		temp2 = 1;
	else
		temp2 = 0;
	loop_body_size += VC4_PACKET_TILE_COORDINATES_SIZE * (temp1 + temp2 - 1);

	size += xtiles * ytiles * loop_body_size;

	vc4_cl *rcl = &setup.rcl;
	let rcl_bo = vc4_bo_create(dev, size, VC4_BO_TYPE_RCL);
	if (rcl_bo == NULL) {
		E_NOMEM
	}
	vc4_init_cl(rcl);
	setup.rcl.base = setup.rcl.next = rcl_bo.vaddr;
	setup.rcl.size = size;
	list_add_before(&exec.unref_list, &rcl_bo.unref_head);

	/* The tile buffer gets cleared when the previous tile is stored.  If
	 * the clear values changed between frames, then the tile buffer has
	 * stale clear values in it, so we have to do a store in None mode (no
	 * writes) so that we trigger the tile buffer clear.
	 */
	if (exec.args.flags & VC4_SUBMIT_CL_USE_CLEAR_COLOR) {
		cl_u8(&rcl, VC4_PACKET_CLEAR_COLORS);
		cl_u32(&rcl, exec.args.clear_color[0]);
		cl_u32(&rcl, exec.args.clear_color[1]);
		cl_u32(&rcl, exec.args.clear_z);
		cl_u8(&rcl, exec.args.clear_s);

		vc4_tile_coordinates(&rcl, 0, 0);

		cl_u8(&rcl, VC4_PACKET_STORE_TILE_BUFFER_GENERAL);
		cl_u16(&rcl, VC4_LOADSTORE_TILE_BUFFER_NONE);
		cl_u32(&rcl, 0); /* no address, since we're in None mode */
	}

	cl_u8(&rcl, VC4_PACKET_TILE_RENDERING_MODE_CONFIG);
	if setup.color_write != NULL 
		cl_u32(&rcl, (setup.color_write->paddr + exec.args.color_write.offset));
	else
		cl_u32(&rcl, 0);
	cl_u16(&rcl, exec.args.width);
	cl_u16(&rcl, exec.args.height);
	cl_u16(&rcl, exec.args.color_write.bits);

	let x: u8, y: u8;
	for y in min_y_tile..=max_y_tile {
		for x in min_x_tile..=max_x_tile {
			bool first = (x == min_x_tile && y == min_y_tile) as bool;
			bool last = (x == max_x_tile && y == max_y_tile) as bool;
			emit_tile(&exec, &setup, x, y, first, last);
		}
	}

	assert!(cl_offset(&rcl) == size);
	exec.ct1ca = rcl_bo.paddr;
	exec.ct1ea = rcl_bo.paddr + cl_offset(rcl);

	0
}

pub fn vc4_rcl_surface_setup(exec: &mut vc4_exec_info, /*???how to deal with ** vc4_bo **obj */, surf: &mut drm_vc4_submit_rcl_surface) -> i32
{
	let tiling =
		VC4_GET_FIELD(surf.bits, VC4_LOADSTORE_TILE_BUFFER_TILING) as u8;
	let buffer =
		VC4_GET_FIELD(surf.bits, VC4_LOADSTORE_TILE_BUFFER_BUFFER) as u8;
	let format =
		VC4_GET_FIELD(surf.bits, VC4_LOADSTORE_TILE_BUFFER_FORMAT) as u8;
	let mut cpp: i32;

	if surf.hindex == ~0
		0

	//???question remains.
	*obj = vc4_use_bo(exec, surf.hindex);
	if (!*obj)
		E_INVAL

	if (surf.bits & ~(VC4_LOADSTORE_TILE_BUFFER_TILING_MASK |
			   VC4_LOADSTORE_TILE_BUFFER_BUFFER_MASK |
			   VC4_LOADSTORE_TILE_BUFFER_FORMAT_MASK)) {
		print!("vc4: Unknown bits in load/store: 0x{04x}\n", surf.bits);
		E_INVAL
	}

	if (tiling > VC4_TILING_FORMAT_LT) {
		print!("vc4: Bad tiling format\n");
		E_INVAL
	}

	if (buffer == VC4_LOADSTORE_TILE_BUFFER_ZS) {
		if (format != 0) {
			print!("vc4: No color format should be set for ZS\n");
			E_INVAL
		}
		cpp = 4;
	} else if (buffer == VC4_LOADSTORE_TILE_BUFFER_COLOR) {
		match format {
			VC4_LOADSTORE_TILE_BUFFER_BGR565 | VC4_LOADSTORE_TILE_BUFFER_BGR565_DITHER => 
				cpp = 2;
			VC4_LOADSTORE_TILE_BUFFER_RGBA8888 => 
				cpp = 4;
			_ => {
				print!("vc4: Bad tile buffer format\n");
				E_INVAL
			}
		}
	} else {
		print!("vc4: Bad load/store buffer {}.\n", buffer);
		E_INVAL
	}

	if (surf.offset & 0xf) {
		print!("vc4: load/store buffer must be 16b aligned.\n");
		E_INVAL
	}

	0
}

pub fn vc4_rcl_render_config_surface_setup( vc4_exec_info *exec, vc4_bo **obj, surf: drm_vc4_submit_rcl_surface) -> i32
{
	let tiling =
		VC4_GET_FIELD(surf.bits, VC4_RENDER_CONFIG_MEMORY_FORMAT) as u8;
	let  format = VC4_GET_FIELD(surf.bits, VC4_RENDER_CONFIG_FORMAT) as u8;
	let mut cpp: i32;

	if (surf.bits & ~(VC4_RENDER_CONFIG_MEMORY_FORMAT_MASK |
			   VC4_RENDER_CONFIG_FORMAT_MASK)) {
		print!("vc4: Unknown bits in render config: 0x{04x}\n",
			surf.bits);
		E_INVAL
	}

	if (surf.hindex == ~0)
		return 0;

	*obj = vc4_use_bo(exec, surf.hindex);
	if (!*obj)
		E_INVAL

	if (tiling > VC4_TILING_FORMAT_LT) {
		print!("vc4: Bad tiling format\n");
		E_INVAL
	}

	match format {
		VC4_RENDER_CONFIG_FORMAT_BGR565_DITHERED | VC4_RENDER_CONFIG_FORMAT_BGR565 => 
			cpp = 2;
		VC4_RENDER_CONFIG_FORMAT_RGBA8888 => 
			cpp = 4;
		_ => {
			print!("vc4: Bad tile buffer format\n");
			E_INVAL
		}
	}

	0
}

pub fn vc4_get_rcl(dev: &mut device, exec: &mut vc4_exec_info) -> i32
{
	// drm_vc4_submit_cl *args = exec->args;
	bool has_bin = (exec.args.bin_cl_size != 0) as bool;
	let mut ret = 0;

	if (exec.args.min_x_tile > exec.args.max_x_tile ||
	    exec.args.min_y_tile > exec.args.max_y_tile) {
		print!("vc4: Bad render tile set ({},{})-({},{})\n",
			exec.args.min_x_tile, exec.args.min_y_tile, exec.args.max_x_tile,
			exec.args.max_y_tile);
		E_INVAL
	}

	if (has_bin && (exec.args.max_x_tile > exec->bin_tiles_x ||
			exec.args.max_y_tile > exec->bin_tiles_y)) {
		print!("vc4: Render tiles ({},{}) outside of bin config "
			"({},{})\n",
			exec.args.max_x_tile, exec.args.max_y_tile, exec->bin_tiles_x,
			exec->bin_tiles_y);
		E_INVAL
	}

	let mut setup = vc4_rcl_setup;//init = 0

	ret = vc4_rcl_surface_setup(&exec, &setup.color_read, &exec.args.color_read);
	if ret != 0
		ret

	ret = vc4_rcl_render_config_surface_setup(&exec, &setup.color_write,
						  &exec.args.color_write);
	if ret != 0
		ret

	ret = vc4_rcl_surface_setup(&exec, &setup.zs_read, &exec.args.zs_read);
	if ret != 0
		ret

	ret = vc4_rcl_surface_setup(&exec, &setup.zs_write, &exec.args.zs_write);
	if ret != 0
		ret

	/* We shouldn't even have the job submitted to us if there's no
	 * surface to write out.
	 */
	if (!setup.color_write && !setup.zs_write) {
		print!("vc4: RCL requires color or Z/S write\n");
		E_INVAL
	}

	vc4_create_rcl_bo(dev, &exec, &setup)
}
