use rcore_fs::vfs::*;

pub mod v3dReg;
mod vc4_drv;
mod vc4_drm;
mod vc4_bo;
mod vc4_gem;
mod vc4_validate;

use bcm2837::v3d::V3d;
use crate::drivers::gpu::gpu_device::*;
use crate::drivers::gpu::fb;

use super::mailbox;
use spin::Mutex;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use self::v3dReg::*;
use super::super::memory;
use rcore_memory::PAGE_SIZE;
use vc4_drm::*;
use vc4_bo::*;
use vc4_gem::*;
use vc4_validate::*;

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
			if lock.is_none() {
				info!("videocore: not able to bind framebuffer");
				return None
			}
			
			info!("videocore: bind framebuffer ok");
		}

		let mut device = GpuDevice {
						bin_bo: None,
						bin_alloc_size: 0,
						handle_bo_map: BTreeMap::new(),
					};

		//alloc binner
		let mut size: u32 = 512 * 1024;
		if let Some(bbo) = device.vc4_bo_create(size, VC4_BO_TYPE_BIN) {
			device.bin_bo = Some(bbo.clone());
			Some(device)
		} else {
			None
		}
	}

	pub fn io_control(&mut self, cmd: u32, data: usize) -> Result<()> {
		match cmd as usize {
			DRM_IOCTL_VC4_SUBMIT_CL => {
				return self.vc4_submit_cl_ioctl(data)
			}
			DRM_IOCTL_VC4_CREATE_BO => {
				return self.vc4_create_bo_ioctl(data)
			}
			DRM_IOCTL_VC4_MMAP_BO => {
				return self.vc4_mmap_bo_ioctl(data)
			}
			DRM_IOCTL_VC4_FREE_BO => {
				return self.vc4_free_bo_ioctl(data)
			}
			_ => {
				Err(FsError::NotSupported)
			}
		}
	}
}


/// Initialize videocore
///
/// Called in arch mod if the board have a videovore
pub fn init() {
	let vdc = GpuDevice::new();

    if let Some(v) = vdc {
    	*GPU_DEVICE.lock() = Some(v);
    } else {
    	info!("videocore: init failed!");
    }
}