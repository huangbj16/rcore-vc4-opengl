use crate::drivers::gpu::gpu_device::*;
use super::vc4_drm::*;
use rcore_fs::vfs::*;
use super::super::mailbox;

use super::super::super::memory;
pub use crate::syscall::mem::MmapProt;
use rcore_memory::memory_set::handler::Linear;
use rcore_memory::PAGE_SIZE;

use spin::Mutex;
use alloc::sync::Arc;
use crate::process::current_thread;

use bcm2837::addr;

pub const VC4_BO_TYPE_FB : u32 = 0;
pub const VC4_BO_TYPE_V3D : u32 = 1;
pub const VC4_BO_TYPE_BIN : u32 = 2;
pub const VC4_BO_TYPE_RCL : u32 = 3;
pub const VC4_BO_TYPE_BCL : u32 = 4;

fn roundDown(a:u32, n:u32) -> u32 {
	a - a % n
}

fn roundUp(a:u32, n:u32) -> u32 {
	roundDown(a + n - 1, n)
}

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

	pub fn vc4_bo_create(&mut self, size: u32, bo_type: u32) -> Option<Arc<Mutex<gpu_bo>>>
	{
		// default frame buffer
		// if (bo_type == VC4_BO_TYPE_FB)
		// 	Some(self.fb_bo.clone())

		if size == 0 {
			return None
		}

		let size = roundUp(size, PAGE_SIZE as u32);

		if let Ok(handle) = mailbox::mem_alloc(size, PAGE_SIZE as u32, mailbox::MEM_FLAG_COHERENT | mailbox::MEM_FLAG_ZERO) {
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
				let vaddr = memory::ioremap(paddr as usize, size as usize, "bo");
				let result = self.handle_bo_map.insert(handle, Arc::new(Mutex::new(gpu_bo {
																	size: size,
																	handle: handle,
																	paddr: paddr as usize,
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

	pub fn vc4_bo_destroy(&mut self, handle: u32) {
		let mut ifDestroy = false;
		if let Some(bo) = self.handle_bo_map.get(&handle) {
			let bo_entry = bo.lock();
			if bo_entry.bo_type == VC4_BO_TYPE_FB {
				return
			}

			memory::iounmap(bo_entry.vaddr, bo_entry.size as usize);
			if let Ok(res) = mailbox::mem_unlock(handle) {
				if let Ok(res) = mailbox::mem_free(handle) {
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

	pub fn vc4_create_bo_ioctl(&mut self, data: usize) -> Result<()>
	{
		let mut args = unsafe { &mut *(data as *mut drm_vc4_create_bo) };

		let mut bo: Option<Arc<Mutex<gpu_bo>>>;

		if (args.flags & VC4_CREATE_BO_IS_FRAMEBUFFER) != 0 {
			bo = self.vc4_bo_create(args.size, VC4_BO_TYPE_FB);
		} else {
			bo = self.vc4_bo_create(args.size, VC4_BO_TYPE_V3D);
		}

		if let Some(bs) = bo {
			let bo_entry = bs.lock();
			args.size = bo_entry.size as u32;
			args.handle = bo_entry.handle;
		} else {
			return Err(FsError::IOCTLError)//error
		}
		Ok(())//success
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

	pub fn vc4_mmap_bo_ioctl(&self, data: usize) -> Result<()>
	{
		let args = unsafe { &mut *(data as *mut drm_vc4_mmap_bo) };

		if let Some(bo) = self.vc4_lookup_bo(args.handle) {
			//attr
			let defaultProt: usize = 0x3;
			let prot = MmapProt::from_bits_truncate(defaultProt);
			let attr = prot.to_attr();
			let attr = attr.mmio(crate::arch::paging::MMIOType::NormalNonCacheable as u8);
			
			let mut vaddr: usize = PAGE_SIZE;
			{
				let bo_entry = bo.lock();
				let len = roundUp(bo_entry.size, PAGE_SIZE as u32);
				let thread = unsafe { current_thread() };
				vaddr = thread.vm.lock().find_free_area(vaddr, len as usize);
				thread.vm.lock().push(
					vaddr,
					vaddr + len as usize,
					attr,
					Linear::new((bo_entry.paddr - vaddr) as isize),
					"mmap_vc4_bo",
				)
			}
			info!("mmap for /dev/gpu0");
			args.offset = vaddr;
			Ok(())
		} else {
			return Err(FsError::InvalidParam)
		}
	}
}
