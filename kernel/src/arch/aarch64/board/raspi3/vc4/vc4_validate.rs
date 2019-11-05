use vc4_cl::*;
use vc4_drv::*;
use vc4_drm::*;
use vc4_packet::*;
use mailbox::*;
use core::mem::size_of;
use crate::syscall::SysError::*;

// #define dev: & device, exec: &mut vc4_exec_info, validated: usize, untrusted: usize                                                          \
// 	device *dev, vc4_exec_info *exec, void *validated,       \
// 		void *untrusted
// dev: & device, exec: &mut vc4_exec_info, validated: usize, untrusted: usize

pub fn vc4_use_bo(exec: &mut vc4_exec_info, hindex: u32) . &vc4_bo
{
	if (hindex >= exec.bo_count) {
		print!("vc4: BO index {} greater than BO count {}\n", hindex, exec.bo_count);
		Option<None>
	}

	&exec.bo[hindex]
}

pub fn vc4_use_handle(exec: &mut vc4_exec_info, gem_handles_packet_index: u32) . & vc4_bo
{
	vc4_use_bo(&exec, exec.bo_index[gem_handles_packet_index]);
}

pub fn validate_indexed_prim_list(dev: & device, exec: &mut vc4_exec_info, validated: usize, untrusted: usize)  -> i32
{
	let length = get_unaligned_32(untrusted + 1): u32;
	let offset = get_unaligned_32(untrusted + 5): u32;
	let max_index = get_unaligned_32(untrusted + 9): u32;
	let index_size: u32;
	// uint32_t index_size = (*(uint8_t *)(untrusted + 0) >> 4) ? 2 : 1;
	if ((untrusted as u8) >> 4) == 1
		index_size = 2;
	else
		index_size = 1;

	/* Check overflow condition */
	if (exec.shader_state_count == 0) {
		print!("vc4: shader state must precede primitives\n");
		E_INVAL
	}
	let mut shader_state = &mut exec.shader_state[exec.shader_state_count - 1];

	if (max_index > shader_state.max_index)
		shader_state.max_index = max_index;

	let mut ib = vc4_use_handle(&exec, 0);
	if (!ib)
		E_INVAL

	if (offset > ib.size || (ib.size - offset) / index_size < length) {
		print!("vc4: IB access overflow ({} + {}*{} > {})\n", offset, length, index_size, ib.size);
		E_INVAL
	}

	put_unaligned_32(validated + 5, ib.paddr + offset);

	0
}

pub fn validate_gl_array_primitive(dev: & device, exec: &mut vc4_exec_info, validated: usize, untrusted: usize)  -> i32
{
	let length = get_unaligned_32(untrusted + 1);
	let base_index = get_unaligned_32(untrusted + 5);

	/* Check overflow condition */
	if (exec.shader_state_count == 0) {
		print!("vc4: shader state must precede primitives\n");
		E_INVAL
	}

	if (length + base_index < length) {
		print!("vc4: primitive vertex count overflow\n");
		E_INVAL
	}
	let max_index = length + base_index - 1;
	let mut shader_state = &exec.shader_state[exec.shader_state_count - 1];

	if (max_index > shader_state.max_index)
		shader_state.max_index = max_index;

	0
}

pub fn validate_nv_shader_state(dev: & device, exec: &mut vc4_exec_info, validated: usize, untrusted: usize)  -> i32
{
	let i = exec.shader_state_count++;

	if (i >= exec.shader_state_size) {
		print!("vc4: More requests for shader states than declared\n");
		E_INVAL
	}

	exec.shader_state[i].addr = get_unaligned_32(&untrusted);
	exec.shader_state[i].max_index = 0;

	put_unaligned_32(&validated, (exec.shader_rec_p + exec.shader_state[i].addr));

	exec.shader_rec_p += 16;

	0
}

validate_tile_binning_config(dev: & device, exec: &mut vc4_exec_info, validated: usize, untrusted: usize)  -> i32
{
	let vc4 = to_vc4_dev(dev);
	let tile_state_size as u8;
	let tile_count as u8;
	let bin_addr as usize;

	//???怎么只取指定的8个bits?
	exec.bin_tiles_x = *(uint8_t *)(untrusted + 12);
	exec.bin_tiles_y = *(uint8_t *)(untrusted + 13);
	tile_count = exec.bin_tiles_x * exec.bin_tiles_y;
	flags = *(uint8_t *)(untrusted + 14);

	bin_addr = vc4.bin_bo.paddr;

	/* The tile state data array is 48 bytes per tile, and we put it at
	 * the start of a BO containing both it and the tile alloc.
	 */
	tile_state_size = 48 * tile_count;

	*(uint8_t *)(validated + 14) =
		((flags & ~(VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_MASK |
			    VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_MASK)) |
		 VC4_BIN_CONFIG_AUTO_INIT_TSDA |
		 VC4_SET_FIELD(VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_32,
			       VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE) |
		 VC4_SET_FIELD(VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_128,
			       VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE));

	/* Since the tile alloc array will follow us, align. */
	exec.tile_alloc_offset = bin_addr + ROUNDUP(tile_state_size, 4096);

	/* tile alloc address. */
	put_unaligned_32(validated + 0, exec.tile_alloc_offset);
	/* tile alloc size. */
	put_unaligned_32(validated + 4, bin_addr + vc4.bin_alloc_size -
						exec.tile_alloc_offset);
	/* tile state address. */
	put_unaligned_32(validated + 8, bin_addr);

	0
}

validate_gem_handles(dev: & device, exec: &mut vc4_exec_info, validated: usize, untrusted: usize)  -> i32
{
	// memcpy(exec.bo_index, untrusted, sizeof(exec.bo_index));
	exec.bo_index = untrusted as [u32;5];
	0
}


//???weird macro rules......
/*
#define ARRAY_SIZE(arr) (sizeof(arr) / sizeof((arr)[0]))

#define VC4_DEFINE_PACKET(packet, func)                                        \
	[packet] = { packet##_SIZE, #packet, func }

const cmd_info {
	uint16_t len;
	const char *name;
	int (*func)(device *dev, exec: &mut vc4_exec_info,
		    void *validated, void *untrusted);
} cmd_info[] = {
	VC4_DEFINE_PACKET(VC4_PACKET_HALT, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_NOP, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_FLUSH, NULL), // validate_flush
	VC4_DEFINE_PACKET(VC4_PACKET_FLUSH_ALL, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_START_TILE_BINNING,
			  NULL), // validate_start_tile_binning
	VC4_DEFINE_PACKET(VC4_PACKET_INCREMENT_SEMAPHORE,
			  NULL), // validate_increment_semaphore

	VC4_DEFINE_PACKET(VC4_PACKET_GL_INDEXED_PRIMITIVE,
			  validate_indexed_prim_list),
	VC4_DEFINE_PACKET(VC4_PACKET_GL_ARRAY_PRIMITIVE,
			  validate_gl_array_primitive),

	VC4_DEFINE_PACKET(VC4_PACKET_PRIMITIVE_LIST_FORMAT, NULL),

	VC4_DEFINE_PACKET(VC4_PACKET_GL_SHADER_STATE,
			  NULL), // validate_gl_shader_state
	VC4_DEFINE_PACKET(VC4_PACKET_NV_SHADER_STATE, validate_nv_shader_state),

	VC4_DEFINE_PACKET(VC4_PACKET_CONFIGURATION_BITS, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_FLAT_SHADE_FLAGS, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_POINT_SIZE, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_LINE_WIDTH, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_RHT_X_BOUNDARY, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_DEPTH_OFFSET, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_CLIP_WINDOW, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_VIEWPORT_OFFSET, NULL),
	VC4_DEFINE_PACKET(VC4_PACKET_CLIPPER_XY_SCALING, NULL),
	/* Note: The docs say this was also 105, but it was 106 in the
	 * initial userland code drop.
	 */
	VC4_DEFINE_PACKET(VC4_PACKET_CLIPPER_Z_SCALING, NULL),

	VC4_DEFINE_PACKET(VC4_PACKET_TILE_BINNING_MODE_CONFIG,
			  validate_tile_binning_config),

	VC4_DEFINE_PACKET(VC4_PACKET_GEM_HANDLES, validate_gem_handles),
};
*/

vc4_validate_bin_cl(dev: & device, validated: usize, unvalidated: usize, exec: &mut vc4_exec_info) -> i32
{
	let len = exec.args.bin_cl_size;
	let mut dst_offset = 0;
	let mut src_offset = 0;

	while src_offset < len {
		let mut dst_pkt = validated + dst_offset;
		let mut src_pkt = unvalidated + src_offset;
		//???hard
		uint8_t cmd = *(uint8_t *)src_pkt;
		const cmd_info *info;

		if cmd >= ARRAY_SIZE(cmd_info) {
			print!("vc4: 0x{08x}: packet {} out of bounds\n", src_offset, cmd);
			E_INVAL
		}

		info = &cmd_info[cmd];
		if (!info.name) {
			print!("vc4: 0x{08x}: packet {} invalid\n", src_offset, cmd);
			E_INVAL
		}

		if (src_offset + info.len > len) {
			print!("vc4: 0x{08x}: packet {} ({}) length 0x{08x} "
				"exceeds bounds (0x{08x})\n",
				src_offset, cmd, info.name, info.len,
				src_offset + len);
			E_INVAL
		}

		if (cmd != VC4_PACKET_GEM_HANDLES)
			memcpy(dst_pkt, src_pkt, info.len);

		if (info.func &&
		    info.func(dev, exec, dst_pkt + 1, src_pkt + 1)) {
			print!("vc4: 0x{08x}: packet {} ({}) failed to validate\n",
				src_offset, cmd, info.name);
			E_INVAL
		}

		src_offset += info.len;
		/* GEM handle loading doesn't produce HW packets. */
		if (cmd != VC4_PACKET_GEM_HANDLES)
			dst_offset += info.len;

		/* When the CL hits halt, it'll stop reading anything else. */
		if (cmd == VC4_PACKET_HALT)
			break;
	}

	exec.ct0ea = exec.ct0ca + dst_offset;

	0
}

validate_nv_shader_rec(dev: & device, exec: &mut vc4_exec_info, state: & vc4_shader_state)  -> i32
{
	let *src_handles;
	let shader_reloc_count = 1;
	vc4_bo *bo[shader_reloc_count];
	let nr_relocs = 3, packet_size = 16;
	int i;

	if (nr_relocs * 4 > exec.shader_rec_size) {
		print!("vc4: overflowed shader recs reading {} handles "
			"from {} bytes left\n",
			nr_relocs, exec.shader_rec_size);
		E_INVAL
	}
	src_handles = exec.shader_rec_u;
	exec.shader_rec_u += nr_relocs * 4;
	exec.shader_rec_size -= nr_relocs * 4;

	if (packet_size > exec.shader_rec_size) {
		print!("vc4: overflowed shader recs copying {}b packet "
			"from {} bytes left\n",
			packet_size, exec.shader_rec_size);
		E_INVAL
	}
	let pkt_u = exec.shader_rec_u;
	let pkt_v = exec.shader_rec_v;
	memcpy(pkt_v, pkt_u, packet_size);
	exec.shader_rec_u += packet_size;
	exec.shader_rec_v += packet_size;
	exec.shader_rec_size -= packet_size;

	for (i = 0; i < nr_relocs; i++) {
		bo[i] = vc4_use_bo(exec, src_handles[i]);
		if (!bo[i])
			E_INVAL
	}
	//???
	uint8_t stride = *(uint8_t *)(pkt_u + 1);
	let fs_offset = get_unaligned_32(pkt_u + 4);
	let uniform_offset = get_unaligned_32(pkt_u + 8);
	let data_offset = get_unaligned_32(pkt_u + 12);
	let max_index;

	put_unaligned_32(pkt_v + 4, bo[0].paddr + fs_offset);
	put_unaligned_32(pkt_v + 8, bo[1].paddr + uniform_offset);

	if (stride != 0) {
		max_index = (bo[2].size - data_offset) / stride;
		if (state.max_index > max_index) {
			print!("vc4: primitives use index {} out of "
				"supplied {}\n",
				state.max_index, max_index);
			E_INVAL
		}
	}

	put_unaligned_32(pkt_v + 12, bo[2].paddr + data_offset);

	0;
}

vc4_validate_shader_recs(dev: & device, exec: &mut vc4_exec_info)  -> i32
{
	let i;
	let mut ret = 0i32;

	for i in 0..=exec.shader_state_count {
		ret = validate_nv_shader_rec(dev, exec, &exec.shader_state[i]);
		if ret != 0
			ret
	}

	ret
}
