

enum vc4_kernel_bo_type {
	VC4_BO_TYPE_FB,
	VC4_BO_TYPE_V3D,
	VC4_BO_TYPE_BIN,
	VC4_BO_TYPE_RCL,
	VC4_BO_TYPE_BCL,
}

pub struct vc4_bo {
	size: usize,
	handle: u32,
	paddr: u32,
	vaddr: usize,//void*
	bo_type: vc4_kernel_bo_type,
	/* List entry for the BO's position in either
	 * vc4_exec_info->unref_list or vc4_dev->bo_cache.time_list
	 */
}