// #include <vmm.h>
// #include <list.h>
// #include <error.h>

use vc4_drv::*;
use vc4_drm::*;
use mailbox::*;
use crate::syscall::SysError::*;
use crate::rcore_memory::PAGE_SIZE;
// #include "vc4_drv.h"
// #include "vc4_drm.h"
// #include "mailbox_property.h"

pub fn vc4_bo_create(dev: &device, size: u32, bo_type: vc4_kernel_bo_type) -> &mut vc4_bo
{
	let vc4 = to_vc4_dev(dev);

	if (bo_type == VC4_BO_TYPE_FB)
		vc4.fb_bo

	if size == 0:
		None

	size = ROUNDUP(size, PAGE_SIZE);

	let handle =
		mbox_mem_alloc(size, PAGE_SIZE, MEM_FLAG_COHERENT | MEM_FLAG_ZERO);
	if handle == 0 {
		println!("VC4: unable to allocate memory with size {%08x}\n", size);
		None
	}
	if handle >= VC4_DEV_BO_NENTRY {
		println!("VC4: too many bo handles, VC4_DEV_BO_NENTRY = {%d}\n",
			VC4_DEV_BO_NENTRY);
		// goto free_mem;
		mbox_mem_free(handle);
		None
	}

	let bus_addr = mbox_mem_lock(handle);
	if bus_addr == 0 {
		println!("VC4: unable to lock memory at handle {%08x}\n", handle);
		// goto free_mem;
		mbox_mem_free(handle);
		None
	}

	__boot_map_iomem(bus_addr, size, bus_addr);

	let mut bo = vc4.handle_bo_map[handle];
	bo.size = size;
	bo.handle = handle;
	bo.paddr = bus_addr;
	bo.vaddr = bus_addr;
	bo.bo_type = bo_type;
	list_init(&bo.unref_head);

	// printf!("vc4_bo_create: %08x %08x %08x %08x\n", bo->size, bo->handle,
	// 	bo->paddr, bo->vaddr);

	bo
}

pub fn vc4_bo_destroy(dev: device, bo: &mut vc4_bo)
{
	// printf!("vc4_bo_destroy: %08x %08x %08x %08x\n", bo->size, bo->handle,
	// 	bo->paddr, bo->vaddr);

	if bo.bo_type == VC4_BO_TYPE_FB :
		None

	//???where is iounmap?
	__ucore_iounmap(bo.vaddr, bo.size);
	mbox_mem_unlock(&bo.handle);
	mbox_mem_free(&bo.handle);
	// free not necessary
	// memset(bo, 0, sizeof(struct vc4_bo));
}

impl vc4_dev {
	pub fn vc4_lookup_bo(&self, handle: u32) -> &mut vc4_bo
	{
		let mut vc4 = to_vc4_dev(dev);
		let mut bo: &mut vc4_bo;

		if handle >= VC4_DEV_BO_NENTRY {
			None
		}

		bo = &mut vc4.handle_bo_map[handle];
		if bo->handle != handle || !bo->size {
			None
		}

		bo
	}
}

pub fn vc4_create_bo_ioctl(dev: &mut device, args: &mut drm_vc4_create_bo) -> i32
{
	//neglected
	// struct drm_vc4_create_bo *args = data; 
	let mut bo : &mut vc4_bo;
	let ret: i32;

	if (args.flags & VC4_CREATE_BO_IS_FRAMEBUFFER as i32) != 0 {
		bo = vc4_bo_create(dev, args.size, VC4_BO_TYPE_FB);
	}
	else {
		bo = vc4_bo_create(dev, args.size, VC4_BO_TYPE_V3D);
	}
	if bo.is_None()
		ENOMEM//error

	args.size = bo.size;
	args.handle = bo.handle;

	0//success
}

pub fn vc4_mmap(dev: & device, vma: &mut vma_struct, paddr: usize) -> i32
{
	let mut start = paddr;
	start &= ~(PAGE_SIZE - 1);
	let r = remap_pfn_range(vma.vm_start, start >> PAGE_SHIFT,
					  vma.vm_end - vma.vm_start);
	if r == 0 {
		ENOMEM//error
	}
	vma.vm_start = r;
	0//success
}

pub fn vc4_mmap_bo_ioctl(dev: &mut device, data: &mut drm_vc4_mmap_bo) -> i32
{
	//neglected
	// struct drm_vc4_mmap_bo *args = data;
	let mut bo: &mut vc4_bo;
	let mut ret = 0i32;

	bo = vc4_lookup_bo(dev, args.handle);
	if bo.is_None() {
		EINVAL
	}

	let mut vma = vma_struct;//init
	if vma.is_None() {
		ENOMEM
	}

	let mut len = ROUNDUP(bo.size, PAGE_SIZE);
	vma.vm_start = 0;
	vma.vm_end = len;

	ret = vc4_mmap(dev, vma, bo.paddr);
	args.offset = vma.vm_start;
	// not needed, auto GC;
	// kfree(vma);

	ret
}

pub fn vc4_free_bo_ioctl(dev: &mut device, args: &mut drm_vc4_free_bo) -> i32
{
	
	let mut bo: &mut vc4_bo;
	let mut ret = 0i32;

	bo = vc4_lookup_bo(dev, args.handle);
	if bo.is_None() {
		EINVAL
	}

	vc4_bo_destroy(dev, bo);

	ret
}
