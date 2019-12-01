use rcore_fs::vfs::*;

pub mod v3dReg;
mod vc4_drv;

use bcm2837::v3d::V3d;
use crate::drivers::gpu::gpu_device::*;
use crate::drivers::gpu::fb;

use super::mailbox;
use spin::Mutex;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use self::v3dReg::*;

lazy_static! {
    static ref V3D: Mutex<V3d> = Mutex::new(V3d::new());
}

impl GpuDevice {
	fn new() -> Option<GpuDevice> {
		// enable gpu
		if (mailbox::gpu_enable().is_ok()) {
			info!("videocore: enable gpu!");
		} else {
			return None
		}

		// check status
		{
			let v3d = V3D.lock();
			if (v3d.read(V3D_IDENT0) != 0x02443356) {
				info!("videocore: V3D pipeline isn't powered up");
				return None
			} else {
				info!("videocore: V3D pipeline has powered up");
			}
		}

		//check framebuffer
		{
			let lock = fb::FRAME_BUFFER.lock();
			if lock.is_some() {
				info!("videocore: bind framebuffer ok");
				let device = GpuDevice {
								bin_bo: Vec::new(),
								bin_alloc_size: 0,
								handle_bo_map: BTreeMap::new(),
							};
				Some(device)
			} else {
				info!("videocore: not able to bind framebuffer");
				return None
			}
		}
	}

	pub fn io_control(&self, cmd: u32, data: usize) -> Result<()> {
		Err(FsError::NotSupported)
	}
}


/// Initialize videocore
///
/// Called in arch mod if the board have a videovore
pub fn init() {
	let vdc = GpuDevice::new();

    if let Some(v) = vdc {
    	let mut size : u32 = 512 * 1024;
    	// if let Some(bo) = v.vc4_bo_create(size, VC4_BO_TYPE_BIN) {
    	// 	v.bin_bo = 
    	// 	v.bin_alloc_size = size;

    	// 	info!("videocore: init end");
    	*GPU_DEVICE.lock() = Some(v);
    	// }
    } else {
    	info!("videocore: init failed!");
    }
}