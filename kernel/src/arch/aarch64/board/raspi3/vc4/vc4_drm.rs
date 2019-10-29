//#ifndef VC4_DRM_H
//#define VC4_DRM_H

//#include <types.h>

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

const VC4_SUBMIT_RCL_SURFACE_READ_IS_FULL_RES		: usize = (1 << 0);//used to be inside the struct
struct drm_vc4_submit_rcl_surface {
	hindex: u32, /* Handle index, or ~0 if not present. */
	offset: u32, /* Offset to start of buffer. */
	/*
	 * Bits for either render config (color_write) or load/store packet.
	 * Bits should all be 0 for MSAA load/stores.
	 */
	bits: u16,
	flags: u16,
}

const VC4_SUBMIT_CL_USE_CLEAR_COLOR			: usize = (1 << 0);
struct drm_vc4_submit_cl {
	/* Pointer to the binner command list.
	 *
	 * This is the first set of commands executed, which runs the
	 * coordinate shader to determine where primitives land on the screen,
	 * then writes out the state updates and draw calls necessary per tile
	 * to the tile allocation BO.
	 */
	bin_cl: u64,

	/* Pointer to the shader records.
	 *
	 * Shader records are the structures read by the hardware that contain
	 * pointers to uniforms, shaders, and vertex attributes.  The
	 * reference to the shader record has enough information to determine
	 * how many pointers are necessary (fixed number for shaders/uniforms,
	 * and an attribute count), so those BO indices into bo_handles are
	 * just stored as __u32s before each shader record passed in.
	 */
	shader_rec: u64,

	/* Pointer to uniform data and texture handles for the textures
	 * referenced by the shader.
	 *
	 * For each shader state record, there is a set of uniform data in the
	 * order referenced by the record (FS, VS, then CS).  Each set of
	 * uniform data has a __u32 index into bo_handles per texture
	 * sample operation, in the order the QPU_W_TMUn_S writes appear in
	 * the program.  Following the texture BO handle indices is the actual
	 * uniform data.
	 *
	 * The individual uniform state blocks don't have sizes passed in,
	 * because the kernel has to determine the sizes anyway during shader
	 * code validation.
	 */
	uniforms: u64,
	bo_handles: u64,

	/* Size in bytes of the binner command list. */
	bin_cl_size: u32,
	/* Size in bytes of the set of shader records. */
	shader_rec_size: u32,
	/* Number of shader records.
	 *
	 * This could just be computed from the contents of shader_records and
	 * the address bits of references to them from the bin CL, but it
	 * keeps the kernel from having to resize some allocations it makes.
	 */
	shader_rec_count: u32,
	/* Size in bytes of the uniform state. */
	uniforms_size: u32,

	/* Number of BO handles passed in (size is that times 4). */
	bo_handle_count: u32,

	/* RCL setup: */
	width: u16,
	height: u16,
	min_x_tile: u8,
	min_y_tile: u8,
	max_x_tile: u8,
	max_y_tile: u8,
	color_read: drm_vc4_submit_rcl_surface,
	color_write: drm_vc4_submit_rcl_surface,
	zs_read: drm_vc4_submit_rcl_surface,
	zs_write: drm_vc4_submit_rcl_surface,
	msaa_color_write: drm_vc4_submit_rcl_surface,
	msaa_zs_write: drm_vc4_submit_rcl_surface,
	clear_color : [u32;2],
	clear_z: u32,
	clear_s: u8,

	pad = 24,//__u32 pad:24;

	flags: u32,

	/* Returned value of the seqno of this render job (for the
	 * wait ioctl).
	 */
	seqno: u64,
}

pub const VC4_CREATE_BO_IS_FRAMEBUFFER			: usize = (1 << 0);
struct drm_vc4_create_bo {
	size: u32,
	/** Returned GEM handle for the BO. */
	handle: u32,

	flags: u32,
}

struct drm_vc4_mmap_bo {
	/** Handle for the object being mapped. */
	handle: u32,
	/** offset into the drm node to use for subsequent mmap call. */
	offset: u32,
}

struct drm_vc4_free_bo {
	/** Handle for the object to free. */
	handle: u32,
}

//#endif // VC4_DRM_H
