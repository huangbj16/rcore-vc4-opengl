use rcore_fs::vfs::*;

use crate::drivers::gpu::fb::FRAME_BUFFER;
use crate::memory::phys_to_virt;
use alloc::{string::String, sync::Arc, vec::Vec};
use core::any::Any;

#[derive(Default)]

pub struct GPU;

impl INode for GPU {
    fn io_control(&self, cmd: u32, data: usize) -> Result<i32> {
        
        match cmd {
            FBIOGET_FSCREENINFO => {
                let fb_fix_info = unsafe { &mut *(data as *mut fb_fix_screeninfo) };
                if let Some(fb) = FRAME_BUFFER.lock().as_ref() {
                    fb.fill_fix_screeninfo(fb_fix_info);
                }
                Ok(())
            }
            FBIOGET_VSCREENINFO => {
                let fb_var_info = unsafe { &mut *(data as *mut fb_var_screeninfo) };
                if let Some(fb) = FRAME_BUFFER.lock().as_ref() {
                    fb.fill_var_screeninfo(fb_var_info);
                }
                Ok(())
            }
            _ => {
                warn!("use never support ioctl !");
                Err(FsError::NotSupported)
            }
        }
        //let fb_fix_info = unsafe{ &mut *(data as *mut fb_fix_screeninfo) };
        //Ok(())
    }
}
