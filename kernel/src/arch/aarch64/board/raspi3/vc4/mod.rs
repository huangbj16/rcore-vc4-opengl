

pub mod v3dReg;
mod vc4_drv;

use bcm2837::v3d::V3d;
use crate::drivers::gpu::fb;

use super::mailbox;
use spin::Mutex;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use self::vc4_drv::vc4_bo;
use self::v3dReg::*;

lazy_static! {
    static ref V3D: Mutex<V3d> = Mutex::new(V3d::new());
}

pub struct vc4_dev {
	/* The memory used for storing binner tile alloc, tile state,
	 * and overflow memory allocations.  This is freed when V3D
	 * powers down.
	 */
	//bin_bo: usize,//struct vc4_bo *
	bin_bo: Vec<Arc<Mutex<vc4_bo>>>,

	//Size of blocks allocated within bin_bo. 
	bin_alloc_size: u32,

	/* Special bo for framebuffer, does not need to be freed. */
	// use framebuffer directly
	//fb_bo: usize,//struct vc4_bo *

	handle_bo_map: BTreeMap<u32, Arc<Mutex<vc4_bo>>>,//struct vc4_bo *
}

impl vc4_dev {
	fn new() -> Option<vc4_dev> {
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
				let device = vc4_dev {
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
}

pub static VIDEOCORE: Mutex<Option<vc4_dev>> = Mutex::new(None);

/// Initialize videocore
///
/// Called in arch mod if the board have a videovore
pub fn init() {
	let vdc = vc4_dev::new();

    if let Some(v) = vdc {
    	let mut size : u32 = 512 * 1024;
    	// if let Some(bo) = v.vc4_bo_create(size, VC4_BO_TYPE_BIN) {
    	// 	v.bin_bo = 
    	// 	v.bin_alloc_size = size;

    	// 	info!("videocore: init end");
    	// 	*VIDEOCORE.lock() = Some(v);
    	// }
    } else {
    	info!("videocore: init failed!");
    }
}