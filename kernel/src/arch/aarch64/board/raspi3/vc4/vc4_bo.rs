
use vc4_drv::*;
use vc4_drm::*;
use mailbox::*;
use crate::syscall::SysError::*;
use crate::rcore_memory::PAGE_SIZE;

// impl vc4_dev {
// 	pub fn vc4_bo_create(&self, size: u32, bo_type: vc4_kernel_bo_type) -> Option(Arc<Mutex<vc4_bo>>)
// 	{
// 		if (bo_type == VC4_BO_TYPE_FB) {
// 			info!("videocore: cannot provide fb bo");
// 			None
// 		}

// 		if size == 0:
// 			None

// 		size = ROUNDUP(size, PAGE_SIZE);

// 		let Ok(handle) = mailbox::mem_alloc(size, PAGE_SIZE, MEM_FLAG_COHERENT | MEM_FLAG_ZERO) {
// 			if handle >= VC4_DEV_BO_NENTRY {
// 				println!("VC4: too many bo handles, VC4_DEV_BO_NENTRY = {%d}\n",
// 					VC4_DEV_BO_NENTRY);
// 				// goto free_mem;
// 				mailbox::mem_free(handle);
// 				None
// 			}

// 			if let Ok(bus_addr) = mailbox::mem_lock(handle) {
// 				let paddr = bus_to_phys(bus_addr);
// 				let vaddr = memory::ioremap(paddr as usize, 0x800000, "bo");
// 				let result = self.handle_bo_map.insert(handle, vc4_bo {
// 																	size: size,
// 																	handle: handle,
// 																	paddr: paddr,
// 																	vaddr: vaddr,
// 																	bo_type: bo_type	
// 																});
// 				if let Some(bo) = self.handle_bo_map.get(&handle) {
// 					bo.clone()
// 				} else {
// 					None
// 				}
// 			} else {
// 				println!("VC4: unable to lock memory at handle {}", handle);
// 				mailbox::mem_free(handle);
// 				None
// 			}
// 		} else {
// 			println!("VC4: unable to allocate memory with size {%08x}\n", size);
// 			None
// 		}
// 	}
// }
