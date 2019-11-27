
// Defines for v3d register offsets
pub const V3D_IDENT0  : usize =0x000>>2; // V3D Identification 0 (V3D block identity)
pub const V3D_IDENT1  : usize =0x004>>2; // V3D Identification 1 (V3D Configuration A)
pub const V3D_IDENT2  : usize =0x008>>2; // V3D Identification 1 (V3D Configuration B)

pub const V3D_SCRATCH : usize =0x010>>2; // Scratch Register

pub const V3D_L2CACTL : usize =0x020>>2; // 2 Cache Control
pub const V3D_SLCACTL : usize =0x024>>2; // Slices Cache Control

pub const V3D_INTCTL  : usize =0x030>>2; // Interrupt Control
pub const V3D_INTENA  : usize =0x034>>2; // Interrupt Enables
pub const V3D_INTDIS  : usize =0x038>>2; // Interrupt Disables

pub const V3D_CT0CS   : usize =0x100>>2; // Control List Executor Thread 0 Control and Status.
pub const V3D_CT1CS   : usize =0x104>>2; // Control List Executor Thread 1 Control and Status.
pub const V3D_CT0EA   : usize =0x108>>2; // Control List Executor Thread 0 End Address.
pub const V3D_CT1EA   : usize =0x10c>>2; // Control List Executor Thread 1 End Address.
pub const V3D_CT0CA   : usize =0x110>>2; // Control List Executor Thread 0 Current Address.
pub const V3D_CT1CA   : usize =0x114>>2; // Control List Executor Thread 1 Current Address.
pub const V3D_CT00RA0 : usize =0x118>>2; // Control List Executor Thread 0 Return Address.
pub const V3D_CT01RA0 : usize =0x11c>>2; // Control List Executor Thread 1 Return Address.
pub const V3D_CT0LC   : usize =0x120>>2; // Control List Executor Thread 0 List Counter
pub const V3D_CT1LC   : usize =0x124>>2; // Control List Executor Thread 1 List Counter
pub const V3D_CT0PC   : usize =0x128>>2; // Control List Executor Thread 0 Primitive List Counter
pub const V3D_CT1PC   : usize =0x12c>>2; // Control List Executor Thread 1 Primitive List Counter

pub const V3D_PCS     : usize =0x130>>2; // V3D Pipeline Control and Status
pub const V3D_BFC     : usize =0x134>>2; // Binning Mode Flush Count
pub const V3D_RFC     : usize =0x138>>2; // Rendering Mode Frame Count

pub const V3D_BPCA    : usize =0x300>>2; // Current Address of Binning Memory Pool
pub const V3D_BPCS    : usize =0x304>>2; // Remaining Size of Binning Memory Pool
pub const V3D_BPOA    : usize =0x308>>2; // Address of Overspill Binning Memory Block
pub const V3D_BPOS    : usize =0x30c>>2; // Size of Overspill Binning Memory Block
pub const V3D_BXCF    : usize =0x310>>2; // Binner Debug

pub const V3D_SQRSV0  : usize =0x410>>2; // Reserve QPUs 0-7
pub const V3D_SQRSV1  : usize =0x414>>2; // Reserve QPUs 8-15
pub const V3D_SQCNTL  : usize =0x418>>2; // QPU Scheduler Control

pub const V3D_SRQPC   : usize =0x430>>2; // QPU User Program Request Program Address
pub const V3D_SRQUA   : usize =0x434>>2; // QPU User Program Request Uniforms Address
pub const V3D_SRQUL   : usize =0x438>>2; // QPU User Program Request Uniforms Length
pub const V3D_SRQCS   : usize =0x43c>>2; // QPU User Program Request Control and Status

pub const V3D_VPACNTL : usize =0x500>>2; // VPM Allocator Control
pub const V3D_VPMBASE : usize =0x504>>2; // VPM base (user) memory reservation

pub const V3D_PCTRC   : usize =0x670>>2; // Performance Counter Clear
pub const V3D_PCTRE   : usize =0x674>>2; // Performance Counter Enables

pub const V3D_PCTR0   : usize =0x680>>2; // Performance Counter Count 0
pub const V3D_PCTRS0  : usize =0x684>>2; // Performance Counter Mapping 0
pub const V3D_PCTR1   : usize =0x688>>2; // Performance Counter Count 1
pub const V3D_PCTRS1  : usize =0x68c>>2; // Performance Counter Mapping 1
pub const V3D_PCTR2   : usize =0x690>>2; // Performance Counter Count 2
pub const V3D_PCTRS2  : usize =0x694>>2; // Performance Counter Mapping 2
pub const V3D_PCTR3   : usize =0x698>>2; // Performance Counter Count 3
pub const V3D_PCTRS3  : usize =0x69c>>2; // Performance Counter Mapping 3
pub const V3D_PCTR4   : usize =0x6a0>>2; // Performance Counter Count 4
pub const V3D_PCTRS4  : usize =0x6a4>>2; // Performance Counter Mapping 4
pub const V3D_PCTR5   : usize =0x6a8>>2; // Performance Counter Count 5
pub const V3D_PCTRS5  : usize =0x6ac>>2; // Performance Counter Mapping 5
pub const V3D_PCTR6   : usize =0x6b0>>2; // Performance Counter Count 6
pub const V3D_PCTRS6  : usize =0x6b4>>2; // Performance Counter Mapping 6
pub const V3D_PCTR7   : usize =0x6b8>>2; // Performance Counter Count 7
pub const V3D_PCTRS7  : usize =0x6bc>>2; // Performance Counter Mapping 7 
pub const V3D_PCTR8   : usize =0x6c0>>2; // Performance Counter Count 8
pub const V3D_PCTRS8  : usize =0x6c4>>2; // Performance Counter Mapping 8
pub const V3D_PCTR9   : usize =0x6c8>>2; // Performance Counter Count 9
pub const V3D_PCTRS9  : usize =0x6cc>>2; // Performance Counter Mapping 9
pub const V3D_PCTR10  : usize =0x6d0>>2; // Performance Counter Count 10
pub const V3D_PCTRS10 : usize =0x6d4>>2; // Performance Counter Mapping 10
pub const V3D_PCTR11  : usize =0x6d8>>2; // Performance Counter Count 11
pub const V3D_PCTRS11 : usize =0x6dc>>2; // Performance Counter Mapping 11
pub const V3D_PCTR12  : usize =0x6e0>>2; // Performance Counter Count 12
pub const V3D_PCTRS12 : usize =0x6e4>>2; // Performance Counter Mapping 12
pub const V3D_PCTR13  : usize =0x6e8>>2; // Performance Counter Count 13
pub const V3D_PCTRS13 : usize =0x6ec>>2; // Performance Counter Mapping 13
pub const V3D_PCTR14  : usize =0x6f0>>2; // Performance Counter Count 14
pub const V3D_PCTRS14 : usize =0x6f4>>2; // Performance Counter Mapping 14
pub const V3D_PCTR15  : usize =0x6f8>>2; // Performance Counter Count 15
pub const V3D_PCTRS15 : usize =0x6fc>>2; // Performance Counter Mapping 15

pub const V3D_DBGE    : usize =0xf00>>2; // PSE Error Signals
pub const V3D_FDBGO   : usize =0xf04>>2; // FEP Overrun Error Signals
pub const V3D_FDBGB   : usize =0xf08>>2; // FEP Interface Ready and Stall Signals, FEP Busy Signals
pub const V3D_FDBGR   : usize =0xf0c>>2; // FEP Internal Ready Signals
pub const V3D_FDBGS   : usize =0xf10>>2; // FEP Internal Stall Input Signals

pub const V3D_ERRSTAT : usize =0xf20>>2; // Miscellaneous Error Signals (VPM, VDW, VCD, VCM, L2C)