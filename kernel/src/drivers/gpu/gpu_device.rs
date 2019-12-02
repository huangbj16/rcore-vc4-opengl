use rcore_fs::vfs::*;
use spin::Mutex;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;

pub struct gpu_bo {
	pub size: usize,
	pub handle: u32,
	pub paddr: u32,
	pub vaddr: usize,
	pub bo_type: u32,
}

pub struct GpuDevice {
	//bin_bo: usize,//struct gpu_bo *
	pub bin_bo: Option<Arc<Mutex<gpu_bo>>>,

	//Size of blocks allocated within bin_bo. 
	pub bin_alloc_size: u32,

	pub handle_bo_map: BTreeMap<u32, Arc<Mutex<gpu_bo>>>,
}

pub static GPU_DEVICE: Mutex<Option<GpuDevice>> = Mutex::new(None);
