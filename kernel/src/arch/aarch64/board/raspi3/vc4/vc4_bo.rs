use crate::drivers::gpu::gpu_device::*;
use super::vc4_drm::*;
use rcore_fs::vfs::*;
use super::VC4_BO_TYPE_FB;
use super::super::mailbox;
use super::super::super::memory;

use spin::Mutex;
use alloc::sync::Arc;

impl GpuDevice {
	pub fn vc4_lookup_bo(&self, handle: u32) -> Option<Arc<Mutex<gpu_bo>>>
	{
		// if handle >= VC4_DEV_BO_NENTRY {
		// 	None
		// }

		if let Some(bo) = self.handle_bo_map.get(&handle) {
			let bo_entry = bo.lock();
			if bo_entry.handle != handle || bo_entry.size == 0 {
				return None
			} else {
				Some(bo.clone());
			}
		}
		None
	}

	pub fn vc4_bo_destroy(&mut self, handle: u32) {
		let mut ifDestroy = false;
		if let Some(bo) = self.handle_bo_map.get(&handle) {
			let bo_entry = bo.lock();
			if bo_entry.bo_type == VC4_BO_TYPE_FB {
				return
			}

			memory::iounmap(bo_entry.vaddr, bo_entry.size);
			if let Ok(res) = mailbox::mem_unlock(handle) {
				let Ok(res) = mailbox::mem_free(handle) {
					ifDestroy = true;
				} else {
					println!("VC4: failed to free memory");
				}
			} else {
				println!("VC4: failed to unlock memory");
			}
		}

		if ifDestroy {
			self.handle_bo_map.remove(&handle);
		}
	}

	pub fn vc4_free_bo_ioctl(&mut self, data: usize) -> Result<()>
	{
		let args = unsafe { & *(data as *mut drm_vc4_free_bo) };

		if let Some(bo) = self.vc4_lookup_bo(args.handle) {
			self.vc4_bo_destroy(args.handle);
		} else {
			return Err(FsError::InvalidParam)
		}

		Ok(())
	}
}
