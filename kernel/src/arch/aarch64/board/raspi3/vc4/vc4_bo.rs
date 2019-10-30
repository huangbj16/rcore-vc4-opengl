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

pub fn vc4_bo_create(dev: &device, size: u32, bo_type: vc4_kernel_bo_type) -> vc4_bo
{
	let vc4 = to_vc4_dev(dev);

	if (bo_type == VC4_BO_TYPE_FB)
		return vc4.fb_bo;

	if size == 0:
		return None;

	size = ROUNDUP(size, PAGE_SIZE);

	let handle =
		mbox_mem_alloc(size, PAGE_SIZE, MEM_FLAG_COHERENT | MEM_FLAG_ZERO);
	if handle == 0 {
		printf!("VC4: unable to allocate memory with size {%08x}\n", size);
		return None;
	}
	if handle >= VC4_DEV_BO_NENTRY {
		printf!("VC4: too many bo handles, VC4_DEV_BO_NENTRY = {%d}\n",
			VC4_DEV_BO_NENTRY);
		// goto free_mem;
		mbox_mem_free(handle);
		return None;
	}

	let bus_addr = mbox_mem_lock(handle);
	if bus_addr == 0 {
		printf!("VC4: unable to lock memory at handle {%08x}\n", handle);
		// goto free_mem;
		mbox_mem_free(handle);
		return None;
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
		return;

	//???where is iounmap?
	__ucore_iounmap(bo.vaddr, bo.size);
	mbox_mem_unlock(&bo.handle);
	mbox_mem_free(&bo.handle);
	// free not necessary
	// memset(bo, 0, sizeof(struct vc4_bo));
}

struct vc4_bo *vc4_lookup_bo(struct device *dev, uint32_t handle)
{
	struct vc4_dev *vc4 = to_vc4_dev(dev);
	struct vc4_bo *bo;

	if (handle >= VC4_DEV_BO_NENTRY) {
		return NULL;
	}

	bo = &vc4->handle_bo_map[handle];
	if (bo->handle != handle || !bo->size) {
		return NULL;
	}

	return bo;
}

int vc4_create_bo_ioctl(struct device *dev, void *data)
{
	struct drm_vc4_create_bo *args = data;
	struct vc4_bo *bo = NULL;
	int ret;

	if (args->flags & VC4_CREATE_BO_IS_FRAMEBUFFER)
		bo = vc4_bo_create(dev, args->size, VC4_BO_TYPE_FB);
	else
		bo = vc4_bo_create(dev, args->size, VC4_BO_TYPE_V3D);
	if (bo == NULL)
		return -E_NOMEM;

	args->size = bo->size;
	args->handle = bo->handle;

	return 0;
}

static int vc4_mmap(struct device *dev, struct vma_struct *vma, uintptr_t paddr)
{
	uintptr_t start = paddr;
	start &= ~(PGSIZE - 1);
	void *r = (void *)remap_pfn_range(vma->vm_start, start >> PGSHIFT,
					  vma->vm_end - vma->vm_start);
	if (!r) {
		return -E_NOMEM;
	}
	vma->vm_start = (uintptr_t)r;
	return 0;
}

int vc4_mmap_bo_ioctl(struct device *dev, void *data)
{
	struct drm_vc4_mmap_bo *args = data;
	struct vc4_bo *bo;
	int ret = 0;

	bo = vc4_lookup_bo(dev, args->handle);
	if (bo == NULL) {
		return -E_INVAL;
	}

	struct vma_struct *vma = NULL;
	vma = (struct vma_struct *)kmalloc(sizeof(struct vma_struct));
	if (!vma) {
		return -E_NOMEM;
	}

	uint32_t len = ROUNDUP(bo->size, PGSIZE);
	vma->vm_start = 0;
	vma->vm_end = vma->vm_start + len;

	ret = vc4_mmap(dev, vma, bo->paddr);
	args->offset = vma->vm_start;
	kfree(vma);

	return ret;
}

int vc4_free_bo_ioctl(struct device *dev, void *data)
{
	struct drm_vc4_free_bo *args = data;
	struct vc4_bo *bo;
	int ret = 0;

	bo = vc4_lookup_bo(dev, args->handle);
	if (bo == NULL) {
		return -E_INVAL;
	}

	vc4_bo_destroy(dev, bo);

	return ret;
}
