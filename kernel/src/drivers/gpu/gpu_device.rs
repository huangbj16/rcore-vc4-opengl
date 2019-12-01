use rcore_fs::vfs::*;
use spin::Mutex;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;

pub struct vc4_bo {
	size: usize,
	handle: u32,
	paddr: u32,
	vaddr: usize,//void*
	bo_type: u32,
}

pub struct GpuDevice {
	//bin_bo: usize,//struct vc4_bo *
	pub bin_bo: Vec<Arc<Mutex<vc4_bo>>>,

	//Size of blocks allocated within bin_bo. 
	pub bin_alloc_size: u32,

	pub handle_bo_map: BTreeMap<u32, Arc<Mutex<vc4_bo>>>,
}

pub static GPU_DEVICE: Mutex<Option<GpuDevice>> = Mutex::new(None);
