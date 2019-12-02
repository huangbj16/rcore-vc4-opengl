use rcore_fs::vfs::*;

pub mod v3dReg;
mod vc4_drv;
mod vc4_drm;
mod vc4_bo;

use bcm2837::v3d::V3d;
use bcm2837::addr;
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

lazy_static! {
    static ref V3D: Mutex<V3d> = Mutex::new(V3d::new());
}

pub const VC4_BO_TYPE_FB : u32 = 0;
pub const VC4_BO_TYPE_V3D : u32 = 1;
pub const VC4_BO_TYPE_BIN : u32 = 2;
pub const VC4_BO_TYPE_RCL : u32 = 3;
pub const VC4_BO_TYPE_BCL : u32 = 4;

fn roundDown(a:usize, n:usize) -> usize {
	a - a % n
}

fn roundUp(a:usize, n:usize) -> usize {
	roundDown(a + n - 1, n)
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
		let mut size: usize = 512 * 1024;
		if let Some(bbo) = device.vc4_bo_create(size, VC4_BO_TYPE_BIN) {
			device.bin_bo = Some(bbo.clone());
			Some(device)
		} else {
			None
		}
	}

	pub fn vc4_bo_create(&mut self, size: usize, bo_type: u32) -> Option<Arc<Mutex<gpu_bo>>>
	{
		// default frame buffer
		// if (bo_type == VC4_BO_TYPE_FB)
		// 	Some(self.fb_bo.clone())

		if size == 0 {
			return None
		}

		let size = roundUp(size, PAGE_SIZE);

		if let Ok(handle) = mailbox::mem_alloc(size as u32, PAGE_SIZE as u32, mailbox::MEM_FLAG_COHERENT | mailbox::MEM_FLAG_ZERO) {
			// we use Tree, so don't have to check this?
			// if handle >= VC4_DEV_BO_NENTRY {
			// 	println!("VC4: too many bo handles, VC4_DEV_BO_NENTRY = {%d}\n",
			// 		VC4_DEV_BO_NENTRY);
			// 	// goto free_mem;
			// 	mailbox::mem_free(handle);
			// 	return None
			// }

			if let Ok(bus_addr) = mailbox::mem_lock(handle) {
				let paddr = addr::bus_to_phys(bus_addr);
				let vaddr = memory::ioremap(paddr as usize, size, "bo");
				let result = self.handle_bo_map.insert(handle, Arc::new(Mutex::new(gpu_bo {
																	size: size,
																	handle: handle,
																	paddr: paddr,
																	vaddr: vaddr,
																	bo_type: bo_type	
																})));
				if let Some(bo) = self.handle_bo_map.get(&handle) {
					Some(bo.clone())
				} else {
					None
				}
			} else {
				println!("VC4: unable to lock memory at handle {}", handle);
				if let Err(r) = mailbox::mem_free(handle) {
					println!("VC4: unable to free memory at handle {}", handle);
				}
				None
			}
		} else {
			println!("VC4: unable to allocate memory with size {:#x}\n", size);
			None
		}
	}

	pub fn io_control(&mut self, cmd: u32, data: usize) -> Result<()> {
		match cmd as usize {
			// DRM_IOCTL_VC4_SUBMIT_CL => {
			// 	self.vc4_submit_cl_ioctl(data);
			// 	Ok(())
			// }
			// DRM_IOCTL_VC4_CREATE_BO => {
			// 	self.vc4_create_bo_ioctl(data);
			// 	Ok(())
			// }
			// DRM_IOCTL_VC4_MMAP_BO => {
			// 	self.vc4_mmap_bo_ioctl(data);
			// 	Ok(())
			// }
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