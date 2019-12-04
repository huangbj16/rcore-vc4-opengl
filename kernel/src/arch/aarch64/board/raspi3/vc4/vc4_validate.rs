use crate::drivers::gpu::gpu_device::*;
use super::vc4_gem::vc4_exec_info;
use alloc::vec::Vec;
use rcore_fs::vfs::*;

impl GpuDevice {
	pub fn vc4_validate_bin_cl(&self, exec: &mut vc4_exec_info, src: & Vec<u8>) -> Result<()>
	{
		// let len = exec.args.bin_cl_size;
		// let mut dst_offset = 0;
		// let mut src_offset = 0;

		// while src_offset < len {
		// 	let mut dst_pkt = validated + dst_offset;

		// 	uint8_t cmd = unvalidated[src_offset];

		// 	if cmd >= CMD_INFO.len() {
		// 		print!("vc4: 0x{08x}: packet {} out of bounds\n", src_offset, cmd);
		// 		Err(FsError::INVALIDPARAMETER)
		// 	}

		// 	let info = &cmd_info[cmd];
		// 	if (info.name.isempty()) {
		// 		print!("vc4: 0x{08x}: packet {} invalid\n", src_offset, cmd);
		// 		Err(FsError::INVALIDPARAMETER)
		// 	}

		// 	if (src_offset + info.len > len) {
		// 		print!("vc4: 0x{08x}: packet {} ({}) length 0x{08x} "
		// 			"exceeds bounds (0x{08x})\n",
		// 			src_offset, cmd, info.name, info.len,
		// 			src_offset + len);
		// 		Err(FsError::INVALIDPARAMETER)
		// 	}

		// 	if (cmd != VC4_PACKET_GEM_HANDLES) {
		// 		for i in 0..info.len {
		// 			unsafe { *(*(dst_pkt + i) as *mut u8) = unvalidated[src_offset + i]; }
		// 		}
		// 		//memcpy(dst_pkt, src_pkt, info.len);
		// 	}

		// 	//TODO
		// 	if let Some(func) = info.func {
		// 		func(dev, exec, dst_pkt + 1, src_pkt + 1)?;
		// 	}

		// 	src_offset += info.len;
		// 	/* GEM handle loading doesn't produce HW packets. */
		// 	if (cmd != VC4_PACKET_GEM_HANDLES)
		// 		dst_offset += info.len;

		// 	/* When the CL hits halt, it'll stop reading anything else. */
		// 	if (cmd == VC4_PACKET_HALT)
		// 		break;
		// }

		// exec.ct0ea = exec.ct0ca + dst_offset;

		Ok(())
	}
}