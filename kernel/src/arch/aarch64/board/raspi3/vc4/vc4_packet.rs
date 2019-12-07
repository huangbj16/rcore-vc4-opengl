// use vc4_regs::{BIT, VC4_MASK};

const fn Bit_8(nr: u8) -> u8 {
    1 << nr
}

const fn Mask_8(low: u8, high: u8) -> u8 {
    (1 << ((high) - (low) + 1) - 1) << (low)
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum vc4_packet {
    VC4_PACKET_HALT = 0,
    VC4_PACKET_NOP = 1,

    VC4_PACKET_FLUSH = 4,
    VC4_PACKET_FLUSH_ALL = 5,
    VC4_PACKET_START_TILE_BINNING = 6,
    VC4_PACKET_INCREMENT_SEMAPHORE = 7,
    VC4_PACKET_WAIT_ON_SEMAPHORE = 8,

    VC4_PACKET_BRANCH = 16,
    VC4_PACKET_BRANCH_TO_SUB_LIST = 17,

    VC4_PACKET_STORE_MS_TILE_BUFFER = 24,
    VC4_PACKET_STORE_MS_TILE_BUFFER_AND_EOF = 25,
    VC4_PACKET_STORE_FULL_RES_TILE_BUFFER = 26,
    VC4_PACKET_LOAD_FULL_RES_TILE_BUFFER = 27,
    VC4_PACKET_STORE_TILE_BUFFER_GENERAL = 28,
    VC4_PACKET_LOAD_TILE_BUFFER_GENERAL = 29,

    VC4_PACKET_GL_INDEXED_PRIMITIVE = 32,
    VC4_PACKET_GL_ARRAY_PRIMITIVE = 33,

    VC4_PACKET_COMPRESSED_PRIMITIVE = 48,
    VC4_PACKET_CLIPPED_COMPRESSED_PRIMITIVE = 49,

    VC4_PACKET_PRIMITIVE_LIST_FORMAT = 56,

    VC4_PACKET_GL_SHADER_STATE = 64,
    VC4_PACKET_NV_SHADER_STATE = 65,
    VC4_PACKET_VG_SHADER_STATE = 66,

    VC4_PACKET_CONFIGURATION_BITS = 96,
    VC4_PACKET_FLAT_SHADE_FLAGS = 97,
    VC4_PACKET_POINT_SIZE = 98,
    VC4_PACKET_LINE_WIDTH = 99,
    VC4_PACKET_RHT_X_BOUNDARY = 100,
    VC4_PACKET_DEPTH_OFFSET = 101,
    VC4_PACKET_CLIP_WINDOW = 102,
    VC4_PACKET_VIEWPORT_OFFSET = 103,
    VC4_PACKET_Z_CLIPPING = 104,
    VC4_PACKET_CLIPPER_XY_SCALING = 105,
    VC4_PACKET_CLIPPER_Z_SCALING = 106,

    VC4_PACKET_TILE_BINNING_MODE_CONFIG = 112,
    VC4_PACKET_TILE_RENDERING_MODE_CONFIG = 113,
    VC4_PACKET_CLEAR_COLORS = 114,
    VC4_PACKET_TILE_COORDINATES = 115,

    /* Not an actual hardware packet -- this is what we use to put
    * references to GEM bos in the command stream, since we need the u32
    * int the actual address packet in order to store the offset from the
    * start of the BO.
    */
    VC4_PACKET_GEM_HANDLES = 254,
}

pub const VC4_PACKET_HALT_SIZE: u32  =1;
pub const VC4_PACKET_NOP_SIZE: u32 =1;
pub const VC4_PACKET_FLUSH_SIZE: u32 =1;
pub const VC4_PACKET_FLUSH_ALL_SIZE: u32 =1;
pub const VC4_PACKET_START_TILE_BINNING_SIZE: u32 =1;
pub const VC4_PACKET_INCREMENT_SEMAPHORE_SIZE: u32 =1;
pub const VC4_PACKET_WAIT_ON_SEMAPHORE_SIZE: u32 =1;
pub const VC4_PACKET_BRANCH_SIZE: u32 =5;
pub const VC4_PACKET_BRANCH_TO_SUB_LIST_SIZE: u32 =5;
pub const VC4_PACKET_STORE_MS_TILE_BUFFER_SIZE: u32 =1;
pub const VC4_PACKET_STORE_MS_TILE_BUFFER_AND_EOF_SIZE: u32 =1;
pub const VC4_PACKET_STORE_FULL_RES_TILE_BUFFER_SIZE: u32 =5;
pub const VC4_PACKET_LOAD_FULL_RES_TILE_BUFFER_SIZE: u32 =5;
pub const VC4_PACKET_STORE_TILE_BUFFER_GENERAL_SIZE: u32 =7;
pub const VC4_PACKET_LOAD_TILE_BUFFER_GENERAL_SIZE: u32 =7;
pub const VC4_PACKET_GL_INDEXED_PRIMITIVE_SIZE: u32 =14;
pub const VC4_PACKET_GL_ARRAY_PRIMITIVE_SIZE: u32 =10;
pub const VC4_PACKET_COMPRESSED_PRIMITIVE_SIZE: u32 =1;
pub const VC4_PACKET_CLIPPED_COMPRESSED_PRIMITIVE_SIZE: u32 =1;
pub const VC4_PACKET_PRIMITIVE_LIST_FORMAT_SIZE: u32 =2;
pub const VC4_PACKET_GL_SHADER_STATE_SIZE: u32 =5;
pub const VC4_PACKET_NV_SHADER_STATE_SIZE: u32 =5;
pub const VC4_PACKET_VG_SHADER_STATE_SIZE: u32 =5;
pub const VC4_PACKET_CONFIGURATION_BITS_SIZE: u32 =4;
pub const VC4_PACKET_FLAT_SHADE_FLAGS_SIZE: u32 =5;
pub const VC4_PACKET_POINT_SIZE_SIZE: u32 =5;
pub const VC4_PACKET_LINE_WIDTH_SIZE: u32 =5;
pub const VC4_PACKET_RHT_X_BOUNDARY_SIZE: u32 =3;
pub const VC4_PACKET_DEPTH_OFFSET_SIZE: u32 =5;
pub const VC4_PACKET_CLIP_WINDOW_SIZE: u32 =9;
pub const VC4_PACKET_VIEWPORT_OFFSET_SIZE: u32 =5;
pub const VC4_PACKET_Z_CLIPPING_SIZE: u32 =9;
pub const VC4_PACKET_CLIPPER_XY_SCALING_SIZE: u32 =9;
pub const VC4_PACKET_CLIPPER_Z_SCALING_SIZE: u32 =9;
pub const VC4_PACKET_TILE_BINNING_MODE_CONFIG_SIZE: u32 =16;
pub const VC4_PACKET_TILE_RENDERING_MODE_CONFIG_SIZE: u32 =11;
pub const VC4_PACKET_CLEAR_COLORS_SIZE: u32 =14;
pub const VC4_PACKET_TILE_COORDINATES_SIZE: u32 =3;
pub const VC4_PACKET_GEM_HANDLES_SIZE: u32 =9;

// /* Number of multisamples supported. */
// pub const VC4_MAX_SAMPLES: usize =4;
// /* Size of a full resolution color or Z tile buffer load/store. */
// pub const VC4_TILE_BUFFER_SIZE: usize =(64 * 64 * 4);

// /** @{
//  * Bits used by packets like VC4_PACKET_STORE_TILE_BUFFER_GENERAL and
//  * VC4_PACKET_TILE_RENDERING_MODE_CONFIG.
// */
// pub const VC4_TILING_FORMAT_LINEAR  : usize =  0;
// pub const VC4_TILING_FORMAT_T    : usize =     1;
// pub const VC4_TILING_FORMAT_LT    : usize =    2;
// /** @} */

// /** @{
//  *
//  * low bits of VC4_PACKET_STORE_FULL_RES_TILE_BUFFER and
//  * VC4_PACKET_LOAD_FULL_RES_TILE_BUFFER.
//  */
// pub const VC4_LOADSTORE_FULL_RES_EOF: usize =BIT(3);
// pub const VC4_LOADSTORE_FULL_RES_DISABLE_CLEAR_ALL: usize =BIT(2);
// pub const VC4_LOADSTORE_FULL_RES_DISABLE_ZS: usize =BIT(1);
// pub const VC4_LOADSTORE_FULL_RES_DISABLE_COLOR: usize =BIT(0);

// /** @{
//  *
//  * byte 2 of VC4_PACKET_STORE_TILE_BUFFER_GENERAL and
//  * VC4_PACKET_LOAD_TILE_BUFFER_GENERAL (low bits of the address)
//  */

// pub const VC4_LOADSTORE_TILE_BUFFER_EOF: usize =BIT(3);
// pub const VC4_LOADSTORE_TILE_BUFFER_DISABLE_FULL_VG_MASK : usize =BIT(2);
// pub const VC4_LOADSTORE_TILE_BUFFER_DISABLE_FULL_ZS: usize =BIT(1);
// pub const VC4_LOADSTORE_TILE_BUFFER_DISABLE_FULL_COLOR  : usize = BIT(0);

// /** @} */

// /** @{
//  *
//  * byte 0-1 of VC4_PACKET_STORE_TILE_BUFFER_GENERAL and
//  * VC4_PACKET_LOAD_TILE_BUFFER_GENERAL
//  */
// pub const VC4_STORE_TILE_BUFFER_DISABLE_VG_MASK_CLEAR: usize = BIT(15);
// pub const VC4_STORE_TILE_BUFFER_DISABLE_ZS_CLEAR    : usize = BIT(14);
// pub const VC4_STORE_TILE_BUFFER_DISABLE_COLOR_CLEAR : usize = BIT(13);
// pub const VC4_STORE_TILE_BUFFER_DISABLE_SWAP: usize =BIT(12);

// pub const VC4_LOADSTORE_TILE_BUFFER_FORMAT_MASK: usize =VC4_MASK(9, 8);
// pub const VC4_LOADSTORE_TILE_BUFFER_FORMAT_SHIFT    : usize = 8;
// pub const VC4_LOADSTORE_TILE_BUFFER_RGBA8888: usize =0;
// pub const VC4_LOADSTORE_TILE_BUFFER_BGR565_DITHER   : usize = 1;
// pub const VC4_LOADSTORE_TILE_BUFFER_BGR565: usize =2;
// /** @} */

// /** @{
//  *
//  * byte 0 of VC4_PACKET_STORE_TILE_BUFFER_GENERAL and
//  * VC4_PACKET_LOAD_TILE_BUFFER_GENERAL
//  */
// pub const VC4_STORE_TILE_BUFFER_MODE_MASK: usize =VC4_MASK(7, 6);
// pub const VC4_STORE_TILE_BUFFER_MODE_SHIFT: usize =6;
// pub const VC4_STORE_TILE_BUFFER_MODE_SAMPLE0: usize =(0 << 6);
// pub const VC4_STORE_TILE_BUFFER_MODE_DECIMATE_X4    : usize = (1 << 6);
// pub const VC4_STORE_TILE_BUFFER_MODE_DECIMATE_X16   : usize = (2 << 6);

// /** The values of the field are VC4_TILING_FORMAT_* */
// pub const VC4_LOADSTORE_TILE_BUFFER_TILING_MASK: usize =VC4_MASK(5, 4);
// pub const VC4_LOADSTORE_TILE_BUFFER_TILING_SHIFT : usize =4;
// pub const VC4_LOADSTORE_TILE_BUFFER_BUFFER_MASK: usize =VC4_MASK(2, 0);
// pub const VC4_LOADSTORE_TILE_BUFFER_BUFFER_SHIFT : usize =0;
// pub const VC4_LOADSTORE_TILE_BUFFER_NONE: usize =0;
// pub const VC4_LOADSTORE_TILE_BUFFER_COLOR: usize =1;
// pub const VC4_LOADSTORE_TILE_BUFFER_ZS: usize =2;
// pub const VC4_LOADSTORE_TILE_BUFFER_Z: usize =3;
// pub const VC4_LOADSTORE_TILE_BUFFER_VG_MASK: usize =4;
// pub const VC4_LOADSTORE_TILE_BUFFER_FULL: usize =5;
// /** @} */

// pub const VC4_INDEX_BUFFER_U8: usize =(0 << 4);
// pub const VC4_INDEX_BUFFER_U16: usize =(1 << 4);

// /* This flag is only present in NV shader state. */
// pub const VC4_SHADER_FLAG_SHADED_CLIP_COORDS: usize =BIT(3);
// pub const VC4_SHADER_FLAG_ENABLE_CLIPPING: usize =BIT(2);
// pub const VC4_SHADER_FLAG_VS_POINT_SIZE: usize =BIT(1);
// pub const VC4_SHADER_FLAG_FS_SINGLE_THREAD: usize =BIT(0);

// /** @{ byte 2 of config bits. */
// pub const VC4_CONFIG_BITS_EARLY_Z_UPDATE: usize =BIT(1);
// pub const VC4_CONFIG_BITS_EARLY_Z: usize =BIT(0);
// /** @} */

// /** @{ byte 1 of config bits. */
// pub const VC4_CONFIG_BITS_Z_UPDATE: usize =BIT(7);
// /** same values in this 3-bit field as PIPE_FUNC_* */
// pub const VC4_CONFIG_BITS_DEPTH_FUNC_SHIFT: usize =4;
// pub const VC4_CONFIG_BITS_COVERAGE_READ_LEAVE: usize =BIT(3);

// pub const VC4_CONFIG_BITS_COVERAGE_UPDATE_NONZERO : usize =(0 << 1);
// pub const VC4_CONFIG_BITS_COVERAGE_UPDATE_ODD: usize =(1 << 1);
// pub const VC4_CONFIG_BITS_COVERAGE_UPDATE_OR: usize =(2 << 1);
// pub const VC4_CONFIG_BITS_COVERAGE_UPDATE_ZERO: usize =(3 << 1);

// pub const VC4_CONFIG_BITS_COVERAGE_PIPE_SELECT: usize =BIT(0);
// /** @} */

// /** @{ byte 0 of config bits. */
// pub const VC4_CONFIG_BITS_RASTERIZER_OVERSAMPLE_NONE : usize =(0 << 6);
// pub const VC4_CONFIG_BITS_RASTERIZER_OVERSAMPLE_4X   : usize =(1 << 6);
// pub const VC4_CONFIG_BITS_RASTERIZER_OVERSAMPLE_16X  : usize =(2 << 6);

// pub const VC4_CONFIG_BITS_AA_POINTS_AND_LINES: usize =BIT(4);
// pub const VC4_CONFIG_BITS_ENABLE_DEPTH_OFFSET: usize =BIT(3);
// pub const VC4_CONFIG_BITS_CW_PRIMITIVES: usize =BIT(2);
// pub const VC4_CONFIG_BITS_ENABLE_PRIM_BACK: usize =BIT(1);
// pub const VC4_CONFIG_BITS_ENABLE_PRIM_FRONT: usize =BIT(0);
// /** @} */

// /** @{ bits in the last u8 of VC4_PACKET_TILE_BINNING_MODE_CONFIG */
// pub const VC4_BIN_CONFIG_DB_NON_MS: usize =BIT(7);

pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_MASK: u8 = Mask_8(6, 5);
pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_SHIFT: u8 =5;
pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_32: u8 =0;
pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_64: u8 =1;
pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_128: u8 =2;
pub const VC4_BIN_CONFIG_ALLOC_BLOCK_SIZE_256: u8 =3;

pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_MASK : u8 = Mask_8(4, 3);
pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_SHIFT : u8 =3;
pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_32    : u8 =0;
pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_64    : u8 =1;
pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_128   : u8 =2;
pub const VC4_BIN_CONFIG_ALLOC_INIT_BLOCK_SIZE_256   : u8 =3;

pub const VC4_BIN_CONFIG_AUTO_INIT_TSDA: u8 = Bit_8(2);
pub const VC4_BIN_CONFIG_TILE_BUFFER_64BIT: u8 = Bit_8(1);
pub const VC4_BIN_CONFIG_MS_MODE_4X: u8 = Bit_8(0);
// /** @} */

// /** @{ bits in the last u16 of VC4_PACKET_TILE_RENDERING_MODE_CONFIG */
// pub const VC4_RENDER_CONFIG_DB_NON_MS: usize =BIT(12);
// pub const VC4_RENDER_CONFIG_EARLY_Z_COVERAGE_DISABLE : usize =BIT(11);
// pub const VC4_RENDER_CONFIG_EARLY_Z_DIRECTION_G: usize =BIT(10);
// pub const VC4_RENDER_CONFIG_COVERAGE_MODE: usize =BIT(9);
// pub const VC4_RENDER_CONFIG_ENABLE_VG_MASK: usize =BIT(8);

// /** The values of the field are VC4_TILING_FORMAT_* */
// pub const VC4_RENDER_CONFIG_MEMORY_FORMAT_MASK: usize =VC4_MASK(7, 6);
// pub const VC4_RENDER_CONFIG_MEMORY_FORMAT_SHIFT: usize =6;

// pub const VC4_RENDER_CONFIG_DECIMATE_MODE_1X: usize =(0 << 4);
// pub const VC4_RENDER_CONFIG_DECIMATE_MODE_4X: usize =(1 << 4);
// pub const VC4_RENDER_CONFIG_DECIMATE_MODE_16X: usize =(2 << 4);

// pub const VC4_RENDER_CONFIG_FORMAT_MASK: usize =VC4_MASK(3, 2);
// pub const VC4_RENDER_CONFIG_FORMAT_SHIFT: usize =2;
// pub const VC4_RENDER_CONFIG_FORMAT_BGR565_DITHERED:usize =  0;
// pub const VC4_RENDER_CONFIG_FORMAT_RGBA8888: usize =1;
// pub const VC4_RENDER_CONFIG_FORMAT_BGR565: usize =2;

// pub const VC4_RENDER_CONFIG_TILE_BUFFER_64BIT: usize =BIT(1);
// pub const VC4_RENDER_CONFIG_MS_MODE_4X: usize =BIT(0);

// pub const VC4_PRIMITIVE_LIST_FORMAT_16_INDEX: usize =(1 << 4);
// pub const VC4_PRIMITIVE_LIST_FORMAT_32_XY: usize =(3 << 4);
// pub const VC4_PRIMITIVE_LIST_FORMAT_TYPE_POINTS: usize =(0 << 0);
// pub const VC4_PRIMITIVE_LIST_FORMAT_TYPE_LINES: usize =(1 << 0);
// pub const VC4_PRIMITIVE_LIST_FORMAT_TYPE_TRIANGLES  : usize = (2 << 0);
// pub const VC4_PRIMITIVE_LIST_FORMAT_TYPE_RHT: usize =(3 << 0);

// #[allow(non_camel_case_types)]
// enum vc4_texture_data_type {
// VC4_TEXTURE_TYPE_RGBA8888 = 0,
// VC4_TEXTURE_TYPE_RGBX8888 = 1,
// VC4_TEXTURE_TYPE_RGBA4444 = 2,
// VC4_TEXTURE_TYPE_RGBA5551 = 3,
// VC4_TEXTURE_TYPE_RGB565 = 4,
// VC4_TEXTURE_TYPE_LUMINANCE = 5,
// VC4_TEXTURE_TYPE_ALPHA = 6,
// VC4_TEXTURE_TYPE_LUMALPHA = 7,
// VC4_TEXTURE_TYPE_ETC1 = 8,
// VC4_TEXTURE_TYPE_S16F = 9,
// VC4_TEXTURE_TYPE_S8 = 10,
// VC4_TEXTURE_TYPE_S16 = 11,
// VC4_TEXTURE_TYPE_BW1 = 12,
// VC4_TEXTURE_TYPE_A4 = 13,
// VC4_TEXTURE_TYPE_A1 = 14,
// VC4_TEXTURE_TYPE_RGBA64 = 15,
// VC4_TEXTURE_TYPE_RGBA32R = 16,
// VC4_TEXTURE_TYPE_YUV422R = 17,
// }

// pub const VC4_TEX_P0_OFFSET_MASK: usize =VC4_MASK(31, 12);
// pub const VC4_TEX_P0_OFFSET_SHIFT: usize =12;
// pub const VC4_TEX_P0_CSWIZ_MASK: usize =VC4_MASK(11, 10);
// pub const VC4_TEX_P0_CSWIZ_SHIFT: usize =10;
// pub const VC4_TEX_P0_CMMODE_MASK: usize =VC4_MASK(9, 9);
// pub const VC4_TEX_P0_CMMODE_SHIFT: usize =9;
// pub const VC4_TEX_P0_FLIPY_MASK: usize =VC4_MASK(8, 8);
// pub const VC4_TEX_P0_FLIPY_SHIFT: usize =8;
// pub const VC4_TEX_P0_TYPE_MASK: usize =VC4_MASK(7, 4);
// pub const VC4_TEX_P0_TYPE_SHIFT: usize =4;
// pub const VC4_TEX_P0_MIPLVLS_MASK: usize =VC4_MASK(3, 0);
// pub const VC4_TEX_P0_MIPLVLS_SHIFT: usize =0;
// pub const VC4_TEX_P1_TYPE4_MASK: usize =VC4_MASK(31, 31);
// pub const VC4_TEX_P1_TYPE4_SHIFT: usize =31;
// pub const VC4_TEX_P1_HEIGHT_MASK: usize =VC4_MASK(30, 20);
// pub const VC4_TEX_P1_HEIGHT_SHIFT: usize =20;
// pub const VC4_TEX_P1_ETCFLIP_MASK: usize =VC4_MASK(19, 19);
// pub const VC4_TEX_P1_ETCFLIP_SHIFT: usize =19;
// pub const VC4_TEX_P1_WIDTH_MASK: usize =VC4_MASK(18, 8);
// pub const VC4_TEX_P1_WIDTH_SHIFT: usize =8;
// pub const VC4_TEX_P1_MAGFILT_MASK: usize =VC4_MASK(7, 7);
// pub const VC4_TEX_P1_MAGFILT_SHIFT: usize =7;
// pub const VC4_TEX_P1_MAGFILT_LINEAR: usize =0;
// pub const VC4_TEX_P1_MAGFILT_NEAREST: usize =1;
// pub const VC4_TEX_P1_MINFILT_MASK: usize =VC4_MASK(6, 4);
// pub const VC4_TEX_P1_MINFILT_SHIFT: usize =4;
// pub const VC4_TEX_P1_MINFILT_LINEAR: usize =0;
// pub const VC4_TEX_P1_MINFILT_NEAREST: usize =1;
// pub const VC4_TEX_P1_MINFILT_NEAR_MIP_NEAR: usize =2;
// pub const VC4_TEX_P1_MINFILT_NEAR_MIP_LIN: usize =3;
// pub const VC4_TEX_P1_MINFILT_LIN_MIP_NEAR: usize =4;
// pub const VC4_TEX_P1_MINFILT_LIN_MIP_LIN: usize =5;
// pub const VC4_TEX_P1_WRAP_T_MASK: usize =VC4_MASK(3, 2);
// pub const VC4_TEX_P1_WRAP_T_SHIFT: usize =2;
// pub const VC4_TEX_P1_WRAP_S_MASK: usize =VC4_MASK(1, 0);
// pub const VC4_TEX_P1_WRAP_S_SHIFT: usize =0;
// pub const VC4_TEX_P1_WRAP_REPEAT: usize =0;
// pub const VC4_TEX_P1_WRAP_CLAMP: usize =1;
// pub const VC4_TEX_P1_WRAP_MIRROR: usize =2;
// pub const VC4_TEX_P1_WRAP_BORDER: usize =3;
// pub const VC4_TEX_P2_PTYPE_MASK: usize =VC4_MASK(31, 30);
// pub const VC4_TEX_P2_PTYPE_SHIFT: usize =30;
// pub const VC4_TEX_P2_PTYPE_IGNORED: usize =0;
// pub const VC4_TEX_P2_PTYPE_CUBE_MAP_STRIDE: usize =1;
// pub const VC4_TEX_P2_PTYPE_CHILD_IMAGE_DIMENSIONS : usize =  2;
// pub const VC4_TEX_P2_PTYPE_CHILD_IMAGE_OFFSETS: usize =3;

// /* VC4_TEX_P2_PTYPE_CUBE_MAP_STRIDE bits */
// pub const VC4_TEX_P2_CMST_MASK: usize =VC4_MASK(29, 12);
// pub const VC4_TEX_P2_CMST_SHIFT: usize =12;
// pub const VC4_TEX_P2_BSLOD_MASK: usize =VC4_MASK(0, 0);
// pub const VC4_TEX_P2_BSLOD_SHIFT: usize =0;

// /* VC4_TEX_P2_PTYPE_CHILD_IMAGE_DIMENSIONS */
// pub const VC4_TEX_P2_CHEIGHT_MASK: usize =VC4_MASK(22, 12);
// pub const VC4_TEX_P2_CHEIGHT_SHIFT: usize =12;
// pub const VC4_TEX_P2_CWIDTH_MASK: usize =VC4_MASK(10, 0);
// pub const VC4_TEX_P2_CWIDTH_SHIFT: usize =0;

// /* VC4_TEX_P2_PTYPE_CHILD_IMAGE_OFFSETS */
// pub const VC4_TEX_P2_CYOFF_MASK: usize =VC4_MASK(22, 12);
// pub const VC4_TEX_P2_CYOFF_SHIFT: usize =12;
// pub const VC4_TEX_P2_CXOFF_MASK: usize =VC4_MASK(10, 0);
// pub const VC4_TEX_P2_CXOFF_SHIFT: usize =0;