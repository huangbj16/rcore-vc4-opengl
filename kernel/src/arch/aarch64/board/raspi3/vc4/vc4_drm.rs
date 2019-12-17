

pub const DRM_IOCTL_VC4_SUBMIT_CL                         : usize = 0x00;
pub const DRM_IOCTL_VC4_WAIT_SEQNO                        : usize = 0x01;
pub const DRM_IOCTL_VC4_WAIT_BO                           : usize = 0x02;
pub const DRM_IOCTL_VC4_CREATE_BO                         : usize = 0x03;
pub const DRM_IOCTL_VC4_MMAP_BO                           : usize = 0x04;
pub const DRM_IOCTL_VC4_CREATE_SHADER_BO                  : usize = 0x05;
pub const DRM_IOCTL_VC4_GET_HANG_STATE                    : usize = 0x06;
pub const DRM_IOCTL_VC4_GET_PARAM                         : usize = 0x07;
pub const DRM_IOCTL_VC4_SET_TILING                        : usize = 0x08;
pub const DRM_IOCTL_VC4_GET_TILING                        : usize = 0x09;
pub const DRM_IOCTL_VC4_LABEL_BO                          : usize = 0x0a;
pub const DRM_IOCTL_VC4_FREE_BO                           : usize = 0x0b;

pub const VC4_CREATE_BO_IS_FRAMEBUFFER					: u32 = (1 << 0);

#[repr(C)]
pub struct drm_vc4_free_bo {
	/** Handle for the object to free. */
	pub handle: u32
}

#[repr(C)]
pub struct drm_vc4_create_bo {
	pub size: u32,
	/** Returned GEM handle for the BO. */
	pub handle: u32,

	pub flags: u32,
}

#[repr(C)]
pub struct drm_vc4_mmap_bo {
	/** Handle for the object being mapped. */
	pub handle: u32,
	/** offset into the drm node to use for subsequent mmap call. */
	pub offset: u64,
}