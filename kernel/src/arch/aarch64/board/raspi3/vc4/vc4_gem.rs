// #include <proc.h>
// #include <error.h>
// #include <assert.h>
//???where is proc?
use vc4_drv::*;
use vc4_drm::*;
use vc4_regs::*;
use mailbox::*;
use core::mem::size_of;
use crate::syscall::SysError::*;
// #include "vc4_drv.h"
// #include "vc4_drm.h"
// #include "vc4_regs.h"

pub fn submit_cl(dev: &mut device, thread: u32, start: u32, end: u32)
{
	/* Set the current and end address of the control list.
	 * Writing the end register is what starts the job.
	 */

	// stop the thread
	V3D_WRITE(V3D_CTNCS(thread), 0x20);

	// Wait for thread to stop
	while (V3D_READ(V3D_CTNCS(thread)) & 0x20);

	V3D_WRITE(V3D_CTNCA(thread), start);
	V3D_WRITE(V3D_CTNEA(thread), end);
}

pub fn vc4_flush_caches(dev: &mut device)
{
	/* Flush the GPU L2 caches.  These caches sit on top of system
	 * L3 (the 128kb or so shared with the CPU), and are
	 * non-allocating in the L3.
	 */
	V3D_WRITE(V3D_L2CACTL, V3D_L2CACTL_L2CCLR);

	V3D_WRITE(V3D_SLCACTL, VC4_SET_FIELD(0xf, V3D_SLCACTL_T1CC) |
				       VC4_SET_FIELD(0xf, V3D_SLCACTL_T0CC) |
				       VC4_SET_FIELD(0xf, V3D_SLCACTL_UCC) |
				       VC4_SET_FIELD(0xf, V3D_SLCACTL_ICC));
}

pub fn vc4_submit_next_bin_job(dev: &mut device, exec: &mut vc4_exec_info)
{
	if exec.is_None()
		return;

	vc4_flush_caches(dev);

	/* Either put the job in the binner if it uses the binner, or
	 * immediately move it to the to-be-rendered queue.
	 */
	if exec.ct0ca == exec.ct0ea {
		return;
	}

	// reset binning frame count
	V3D_WRITE(V3D_BFC, 1);

	submit_cl(dev, 0, exec.ct0ca, exec.ct0ea);

	// wait for binning to finish
	while (V3D_READ(V3D_BFC) == 0);
}

pub fn vc4_submit_next_render_job(dev: &mut device, exec: &mut vc4_exec_info)
{
	if exec.is_None()
		return;

	// reset rendering frame count
	V3D_WRITE(V3D_RFC, 1);

	submit_cl(&dev, 1, exec.ct1ca, exec.ct1ea);

	// wait for render to finish
	while (V3D_READ(V3D_RFC) == 0);
}

/* Queues a struct vc4_exec_info for execution.  If no job is
 * currently executing, then submits it.
 *
 * Unlike most GPUs, our hardware only handles one command list at a
 * time.  To queue multiple jobs at once, we'd need to edit the
 * previous command list to have a jump to the new one at the end, and
 * then bump the end address.  That's a change for a later date,
 * though.
 */
pub fn vc4_queue_submit(dev: &mut device, exec: &mut vc4_exec_info)
{
	// TODO
	vc4_submit_next_bin_job(&dev, &exec);
	vc4_submit_next_render_job(&dev, &exec);
}

/**
 * vc4_cl_lookup_bos() - Sets up exec->bo[] with the GEM objects
 * referenced by the job.
 * @dev: device
 * @exec: V3D job being set up
 *
 * The command validator needs to reference BOs by their index within
 * the submitted job's BO list.  This does the validation of the job's
 * BO list and reference counting for the lifetime of the job.
 *
 * Note that this function doesn't need to unreference the BOs on
 * failure, because that will happen at vc4_complete_exec() time.
 */
pub fn vc4_cl_lookup_bos(dev: &mut device, exec: &mut vc4_exec_info) -> i32
{
	let vc4 = to_vc4_dev(&dev);
	// struct drm_vc4_submit_cl *args = exec->args;
	// struct mm_struct *mm = current->mm;
	assert!(!current.mm.is_None());

	exec.bo_count = exec.args.bo_handle_count;

	if exec.bo_count == 0 {
		0
	}

	exec.fb_bo = vc4.fb_bo;
	//??? 2-D array pointer, unsolved.
	// exec.bo = (struct vc4_bo **)kmalloc(exec->bo_count *
	// 				     sizeof(struct vc4_bo *));
	if exec.bo.is_None() {
		print!("vc4: Failed to allocate validated BO pointers\n");
		E_NOMEM
	}

	// handles = (uint32_t *)kmalloc(exec->bo_count, sizeof(uint32_t));
	let mut handles = vec![0; exec.bo_count];
	if handles.is_None() {
		print!("vc4: Failed to allocate incoming GEM handles\n");
		E_NOMEM
	}

	if copy_from_user(mm, handles, exec.args.bo_handles, exec->bo_count * core::mem::size_of::<u32>(), 0) == 0 {
		print!("vc4: Failed to copy in GEM handles\n");
		E_FAULT
	}

	for i in 0..exec.bo_count {
		let bo = vc4_lookup_bo(&dev, handles[i]);
		if bo.is_None() {
			print!("vc4: Failed to look up GEM BO %d: %d\n", i, handles[i]);
			E_INVAL
		}
		exec.bo[i] = bo;
	}

	0//???should be Ok(0)?
}

pub fn vc4_get_bcl(dev: &mut device, exec: &mut vc4_exec_info) -> i32
{
	// struct drm_vc4_submit_cl *args = exec->args;
	void *temp = NULL;
	void *bin;
	assert!(!current.mm.is_None());

	let bin_offset = 0u32;
	let shader_rec_offset = ROUNDUP(bin_offset + exec.args.bin_cl_size, 16) as u32;
	let uniforms_offset = shader_rec_offset + exec.args.shader_rec_size as u32;
	let exec_size = uniforms_offset + exec.args.uniforms_size;
	let temp_size = exec_size + (core::mem::size_of<vc4_shader_state>() * exec.args.shader_rec_count);

	if (shader_rec_offset < exec.args.bin_cl_size || uniforms_offset < shader_rec_offset || exec_size < uniforms_offset || exec.args.shader_rec_count >= ((!0U) / core::mem::size_of<vc4_shader_state>) || temp_size < exec_size) {
		print!("vc4: overflow in exec arguments\n");
		E_INVAL
	}

	//??? allocate a piece of memory for some reason,
	// such operation may not be suitable in rust
	// temp = (void *)kmalloc(temp_size);
	// if (!temp) {
	// 	print!("vc4: Failed to allocate storage for copying "
	// 		"in bin/render CLs.\n");
	// 	ret = -E_NOMEM;
	// 	goto fail;
	// }
	// bin = temp + bin_offset;
	// exec->shader_rec_u = temp + shader_rec_offset;
	// exec->shader_state = temp + exec_size;
	// exec->shader_state_size = exec.args.shader_rec_count;

	if (exec.args.bin_cl_size != 0 && !copy_from_user(current.mm, bin, exec.args.bin_cl, exec.args.bin_cl_size, 0)) {
		E_FAULT
	}

	if (exec.args.shader_rec_size && !copy_from_user(current.mm, exec->shader_rec_u, (uintptr_t)exec.args.shader_rec, exec.args.shader_rec_size, 0)) {
		E_FAULT
	}

	let mut bo = vc4_bo_create(dev, exec_size, VC4_BO_TYPE_BCL);
	if bo.is_None() {
		print!("vc4: Couldn't allocate BO for binning\n");
		E_NOMEM
	}
	exec.exec_bo = &bo;

	list_add_before(&exec.unref_list, &exec.exec_bo.unref_head);

	exec.ct0ca = exec.exec_bo.paddr + bin_offset;

	exec.bin_u = bin;

	exec.shader_rec_v = exec.exec_bo.vaddr + shader_rec_offset;
	exec.shader_rec_p = exec.exec_bo.paddr + shader_rec_offset;
	exec.shader_rec_size = exec.args.shader_rec_size;

	let mut ret = 0i32;
	ret = vc4_validate_bin_cl(dev, exec->exec_bo->vaddr + bin_offset, bin,
				  exec);
	if ret != 0
		ret

	ret = vc4_validate_shader_recs(dev, exec);
	if ret != 0
		ret

	ret
}

pub fn vc4_complete_exec(dev: &mut device, exec: &mut vc4_exec_info)
{
	struct vc4_dev *vc4 = to_vc4_dev(dev);

	if (exec->bo) {
		kfree(exec->bo);
	}

	while (!list_empty(&exec->unref_list)) {
		list_entry_t *le = list_next(&exec->unref_list);
		struct vc4_bo *bo = le2bo(le, unref_head);
		list_del(&bo->unref_head);
		vc4_bo_destroy(dev, bo);
	}

	kfree(exec);
}

/**
 * vc4_submit_cl_ioctl() - Submits a job (frame) to the VC4.
 * @dev: vc4 device
 * @data: ioctl argument
 *
 * This is the main entrypoint for userspace to submit a 3D frame to
 * the GPU.  Userspace provides the binner command list (if
 * applicable), and the kernel sets up the render command list to draw
 * to the framebuffer described in the ioctl, using the command lists
 * that the 3D engine's binner will produce.
 */
int vc4_submit_cl_ioctl(dev: &mut device, void *data)
{
	struct drm_vc4_submit_cl *args = data;
	exec: &mut vc4_exec_info;
	int ret = 0;

	exec = (struct vc4_exec_info *)kmalloc(sizeof(struct vc4_exec_info));
	if (!exec) {
		print!("vc4: malloc failure on exec struct\n");
		return -E_NOMEM;
	}

	memset(exec, 0, sizeof(struct vc4_exec_info));
	exec->args = args;
	list_init(&exec->unref_list);

	ret = vc4_cl_lookup_bos(dev, exec);
	if (ret)
		goto fail;

	if exec.args.bin_cl_size != 0 {
		ret = vc4_get_bcl(dev, exec);
		if (ret)
			goto fail;
	} else {
		exec->ct0ca = 0;
		exec->ct0ea = 0;
	}

	ret = vc4_get_rcl(dev, exec);
	if (ret)
		goto fail;

	/* Clear this out of the struct we'll be putting in the queue,
	 * since it's part of our stack.
	 */
	exec->args = NULL;

	vc4_queue_submit(dev, exec);

fail:
	vc4_complete_exec(dev, exec);

	return ret;
}
