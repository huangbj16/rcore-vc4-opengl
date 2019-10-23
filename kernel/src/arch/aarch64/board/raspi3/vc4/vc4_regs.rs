/*
 *  Copyright © 2014-2015 Broadcom
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 */
#[allow(non_camel_case_types)]

pub const V3D_BASE :usize =	0x20c00000;

//#ifndef BIT
pub const fn BIT(nr: usize) -> usize {
    1 << (nr)
}
//#endif

pub const fn VC4_MASK(high: usize, low: usize) -> usize {
    (((1 << ((high) - (low) + 1)) - 1) << (low))
}

/* Using the GNU statement expression extension */
pub fn VC4_SET_FIELD(value: usize, field_shift: usize, field_mask: usize) -> usize {
    let fieldval : usize = (value) << field_shift;
    assert!((fieldval &  !field_mask) == 0);
    fieldval & field_mask
}

pub const fn VC4_GET_FIELD(word: usize, field_shift: usize, field_mask: usize) -> usize{
     (((word) & field_mask) >> field_shift)
}

pub const V3D_IDENT0 :usize =  0x00000;
pub const V3D_EXPECTED_IDENT0 :usize =  ((2 << 24) | (('V' as usize) << 0) | (('3' as usize) << 8) | (('D' as usize) << 16));

pub const V3D_IDENT1  :usize =  0x00004;

/* Multiples of 1kb */
pub const V3D_IDENT1_VPM_SIZE_MASK: usize =VC4_MASK(31, 28);
pub const V3D_IDENT1_VPM_SIZE_SHIFT: usize =28;
pub const V3D_IDENT1_NSEM_MASK: usize =VC4_MASK(23, 16);
pub const V3D_IDENT1_NSEM_SHIFT: usize =16;
pub const V3D_IDENT1_TUPS_MASK: usize =VC4_MASK(15, 12);
pub const V3D_IDENT1_TUPS_SHIFT: usize =12;
pub const V3D_IDENT1_QUPS_MASK: usize =VC4_MASK(11, 8);
pub const V3D_IDENT1_QUPS_SHIFT: usize =8;
pub const V3D_IDENT1_NSLC_MASK: usize =VC4_MASK(7, 4);
pub const V3D_IDENT1_NSLC_SHIFT: usize =4;
pub const V3D_IDENT1_REV_MASK: usize =VC4_MASK(3, 0);
pub const V3D_IDENT1_REV_SHIFT: usize =0;
pub const V3D_IDENT2: usize =0x00008;
pub const V3D_SCRATCH: usize =0x00010;
pub const V3D_L2CACTL: usize =0x00020;
pub const V3D_L2CACTL_L2CCLR: usize =BIT(2);
pub const V3D_L2CACTL_L2CDIS: usize =BIT(1);
pub const V3D_L2CACTL_L2CENA: usize =BIT(0);
pub const V3D_SLCACTL: usize =0x00024;
pub const V3D_SLCACTL_T1CC_MASK: usize =VC4_MASK(27, 24);
pub const V3D_SLCACTL_T1CC_SHIFT: usize =24;
pub const V3D_SLCACTL_T0CC_MASK: usize =VC4_MASK(19, 16);
pub const V3D_SLCACTL_T0CC_SHIFT: usize =16;
pub const V3D_SLCACTL_UCC_MASK: usize =VC4_MASK(11, 8);
pub const V3D_SLCACTL_UCC_SHIFT: usize =8;
pub const V3D_SLCACTL_ICC_MASK: usize =VC4_MASK(3, 0);
pub const V3D_SLCACTL_ICC_SHIFT: usize =0;
pub const V3D_INTCTL: usize =0x00030;
pub const V3D_INTENA: usize =0x00034;
pub const V3D_INTDIS: usize =0x00038;
pub const V3D_INT_SPILLUSE: usize =BIT(3);
pub const V3D_INT_OUTOMEM: usize =BIT(2);
pub const V3D_INT_FLDONE: usize =BIT(1);
pub const V3D_INT_FRDONE: usize =BIT(0);
pub const V3D_CT0CS: usize =0x00100;
pub const V3D_CT1CS: usize =0x00104;
pub const fn V3D_CTNCS(n: usize) -> usize {
	(V3D_CT0CS + 4 * n)
}
pub const V3D_CTRSTA: usize =BIT(15);
pub const V3D_CTSEMA: usize =BIT(12);
pub const V3D_CTRTSD: usize =BIT(8);
pub const V3D_CTRUN: usize =BIT(5);
pub const V3D_CTSUBS: usize =BIT(4);
pub const V3D_CTERR: usize =BIT(3);
pub const V3D_CTMODE: usize =BIT(0);
pub const V3D_CT0EA: usize =0x00108;
pub const V3D_CT1EA: usize =0x0010c;
pub const fn V3D_CTNEA(n: usize) -> usize {
	(V3D_CT0EA + 4 * (n))
}
pub const V3D_CT0CA: usize =0x00110;
pub const V3D_CT1CA: usize =0x00114;
pub const fn V3D_CTNCA(n: usize) -> usize {
	(V3D_CT0CA + 4 * (n))
}
pub const V3D_CT00RA0: usize =0x00118;
pub const V3D_CT01RA0: usize =0x0011c;
pub const fn V3D_CTNRA0(n: usize) -> usize {
	(V3D_CT00RA0 + 4 * (n))
}
pub const V3D_CT0LC: usize =0x00120;
pub const V3D_CT1LC: usize =0x00124;
pub const fn V3D_CTNLC(n: usize) -> usize {
	(V3D_CT0LC + 4 * (n))
}
pub const V3D_CT0PC: usize =0x00128;
pub const V3D_CT1PC: usize =0x0012c;
pub const fn V3D_CTNPC(n: usize) -> usize {
	(V3D_CT0PC + 4 * (n))
}
pub const V3D_PCS: usize =0x00130;
pub const V3D_BMOOM: usize =BIT(8);
pub const V3D_RMBUSY: usize =BIT(3);
pub const V3D_RMACTIVE: usize =BIT(2);
pub const V3D_BMBUSY: usize =BIT(1);
pub const V3D_BMACTIVE: usize =BIT(0);
pub const V3D_BFC: usize =0x00134;
pub const V3D_RFC: usize =0x00138;
pub const V3D_BPCA: usize =0x00300;
pub const V3D_BPCS: usize =0x00304;
pub const V3D_BPOA: usize =0x00308;
pub const V3D_BPOS: usize =0x0030c;
pub const V3D_BXCF: usize =0x00310;
pub const V3D_SQRSV0: usize =0x00410;
pub const V3D_SQRSV1: usize =0x00414;
pub const V3D_SQCNTL: usize =0x00418;
pub const V3D_SRQPC: usize =0x00430;
pub const V3D_SRQUA: usize =0x00434;
pub const V3D_SRQUL: usize =0x00438;
pub const V3D_SRQCS: usize =0x0043c;
pub const V3D_VPACNTL: usize =0x00500;
pub const V3D_VPMBASE: usize =0x00504;
pub const V3D_PCTRC: usize =0x00670;
pub const V3D_PCTRE: usize =0x00674;
pub const V3D_PCTR0: usize =0x00680;
pub const V3D_PCTRS0: usize =0x00684;
pub const V3D_PCTR1: usize =0x00688;
pub const V3D_PCTRS1: usize =0x0068c;
pub const V3D_PCTR2: usize =0x00690;
pub const V3D_PCTRS2: usize =0x00694;
pub const V3D_PCTR3: usize =0x00698;
pub const V3D_PCTRS3: usize =0x0069c;
pub const V3D_PCTR4: usize =0x006a0;
pub const V3D_PCTRS4: usize =0x006a4;
pub const V3D_PCTR5: usize =0x006a8;
pub const V3D_PCTRS5: usize =0x006ac;
pub const V3D_PCTR6: usize =0x006b0;
pub const V3D_PCTRS6: usize =0x006b4;
pub const V3D_PCTR7: usize =0x006b8;
pub const V3D_PCTRS7: usize =0x006bc;
pub const V3D_PCTR8: usize =0x006c0;
pub const V3D_PCTRS8: usize =0x006c4;
pub const V3D_PCTR9: usize =0x006c8;
pub const V3D_PCTRS9: usize =0x006cc;
pub const V3D_PCTR10: usize =0x006d0;
pub const V3D_PCTRS10: usize =0x006d4;
pub const V3D_PCTR11: usize =0x006d8;
pub const V3D_PCTRS11: usize =0x006dc;
pub const V3D_PCTR12: usize =0x006e0;
pub const V3D_PCTRS12: usize =0x006e4;
pub const V3D_PCTR13: usize =0x006e8;
pub const V3D_PCTRS13: usize =0x006ec;
pub const V3D_PCTR14: usize =0x006f0;
pub const V3D_PCTRS14: usize =0x006f4;
pub const V3D_PCTR15: usize =0x006f8;
pub const V3D_PCTRS15: usize =0x006fc;
pub const V3D_DBGE: usize =0x00f00;
pub const V3D_FDBGO: usize =0x00f04;
pub const V3D_FDBGB: usize =0x00f08;
pub const V3D_FDBGR: usize =0x00f0c;
pub const V3D_FDBGS: usize =0x00f10;
pub const V3D_ERRSTAT: usize =0x00f20;

pub const PV_CONTROL: usize = 0x00;
pub const PV_CONTROL_FORMAT_MASK: usize = VC4_MASK(23, 21);
pub const PV_CONTROL_FORMAT_SHIFT: usize = 21;
pub const PV_CONTROL_FORMAT_24: usize = 0;
pub const PV_CONTROL_FORMAT_DSIV_16: usize = 1;
pub const PV_CONTROL_FORMAT_DSIC_16: usize = 2;
pub const PV_CONTROL_FORMAT_DSIV_18: usize = 3;
pub const PV_CONTROL_FORMAT_DSIV_24: usize = 4;

pub const PV_CONTROL_FIFO_LEVEL_MASK: usize = VC4_MASK(20, 15);
pub const PV_CONTROL_FIFO_LEVEL_SHIFT: usize = 15;
pub const PV_CONTROL_CLR_AT_START: usize = BIT(14);
pub const PV_CONTROL_TRIGGER_UNDERFLOW: usize = BIT(13);
pub const PV_CONTROL_WAIT_HSTART: usize = BIT(12);
pub const PV_CONTROL_PIXEL_REP_MASK: usize = VC4_MASK(5, 4);
pub const PV_CONTROL_PIXEL_REP_SHIFT: usize = 4;
pub const PV_CONTROL_CLK_SELECT_DSI: usize = 0;
pub const PV_CONTROL_CLK_SELECT_DPI_SMI_HDMI: usize = 1;
pub const PV_CONTROL_CLK_SELECT_VEC: usize = 2;
pub const PV_CONTROL_CLK_SELECT_MASK: usize = VC4_MASK(3, 2);
pub const PV_CONTROL_CLK_SELECT_SHIFT: usize = 2;
pub const PV_CONTROL_FIFO_CLR: usize = BIT(1);
pub const PV_CONTROL_EN: usize = BIT(0);

pub const PV_V_CONTROL: usize = 0x04;
pub const PV_VCONTROL_ODD_DELAY_MASK: usize = VC4_MASK(22, 6);
pub const PV_VCONTROL_ODD_DELAY_SHIFT: usize = 6;
pub const PV_VCONTROL_ODD_FIRST: usize = BIT(5);
pub const PV_VCONTROL_INTERLACE: usize = BIT(4);
pub const PV_VCONTROL_DSI: usize = BIT(3);
pub const PV_VCONTROL_COMMAND: usize = BIT(2);
pub const PV_VCONTROL_CONTINUOUS: usize = BIT(1);
pub const PV_VCONTROL_VIDEN: usize = BIT(0);

pub const PV_VSYNCD_EVEN: usize = 0x08;

pub const PV_HORZA: usize = 0x0c;
pub const PV_HORZA_HBP_MASK: usize = VC4_MASK(31, 16);
pub const PV_HORZA_HBP_SHIFT: usize = 16;
pub const PV_HORZA_HSYNC_MASK: usize = VC4_MASK(15, 0);
pub const PV_HORZA_HSYNC_SHIFT: usize = 0;

pub const PV_HORZB: usize = 0x10;
pub const PV_HORZB_HFP_MASK: usize = VC4_MASK(31, 16);
pub const PV_HORZB_HFP_SHIFT: usize = 16;
pub const PV_HORZB_HACTIVE_MASK: usize = VC4_MASK(15, 0);
pub const PV_HORZB_HACTIVE_SHIFT: usize = 0;

pub const PV_VERTA: usize = 0x14;
pub const PV_VERTA_VBP_MASK: usize = VC4_MASK(31, 16);
pub const PV_VERTA_VBP_SHIFT: usize = 16;
pub const PV_VERTA_VSYNC_MASK: usize = VC4_MASK(15, 0);
pub const PV_VERTA_VSYNC_SHIFT: usize = 0;

pub const PV_VERTB: usize = 0x18;
pub const PV_VERTB_VFP_MASK: usize = VC4_MASK(31, 16);
pub const PV_VERTB_VFP_SHIFT: usize = 16;
pub const PV_VERTB_VACTIVE_MASK: usize = VC4_MASK(15, 0);
pub const PV_VERTB_VACTIVE_SHIFT: usize = 0;

pub const PV_VERTA_EVEN: usize = 0x1c;
pub const PV_VERTB_EVEN: usize = 0x20;

pub const PV_INTEN: usize = 0x24;
pub const PV_INTSTAT: usize = 0x28;
pub const PV_INT_VID_IDLE: usize = BIT(9);
pub const PV_INT_VFP_END: usize = BIT(8);
pub const PV_INT_VFP_START: usize = BIT(7);
pub const PV_INT_VACT_START: usize = BIT(6);
pub const PV_INT_VBP_START: usize = BIT(5);
pub const PV_INT_VSYNC_START: usize = BIT(4);
pub const PV_INT_HFP_START: usize = BIT(3);
pub const PV_INT_HACT_START: usize = BIT(2);
pub const PV_INT_HBP_START: usize = BIT(1);
pub const PV_INT_HSYNC_START: usize = BIT(0);

pub const PV_STAT: usize = 0x2c;

pub const PV_HACT_ACT: usize = 0x30;

pub const SCALER_DISPCTRL: usize = 0x00000000;
/* Global register for clock gating the HVS */
pub const SCALER_DISPCTRL_ENABLE: usize = BIT(31);
pub const SCALER_DISPCTRL_DSP2EISLUR: usize = BIT(15);
pub const SCALER_DISPCTRL_DSP1EISLUR: usize = BIT(14);
pub const SCALER_DISPCTRL_DSP3_MUX_MASK: usize = VC4_MASK(19, 18);
pub const SCALER_DISPCTRL_DSP3_MUX_SHIFT: usize = 18;

/* Enables Display 0 short line and underrun contribution to
 * SCALER_DISPSTAT_IRQDISP0.  Note that short frame contributions are
 * always enabled.
 */
pub const SCALER_DISPCTRL_DSP0EISLUR: usize = BIT(13);
pub const SCALER_DISPCTRL_DSP2EIEOLN: usize = BIT(12);
pub const SCALER_DISPCTRL_DSP2EIEOF: usize = BIT(11);
pub const SCALER_DISPCTRL_DSP1EIEOLN: usize = BIT(10);
pub const SCALER_DISPCTRL_DSP1EIEOF: usize = BIT(9);
/* Enables Display 0 end-of-line-N contribution to
 * SCALER_DISPSTAT_IRQDISP0
 */
pub const SCALER_DISPCTRL_DSP0EIEOLN: usize = BIT(8);
/* Enables Display 0 EOF contribution to SCALER_DISPSTAT_IRQDISP0 */
pub const SCALER_DISPCTRL_DSP0EIEOF: usize = BIT(7);

pub const SCALER_DISPCTRL_SLVRDEIRQ: usize = BIT(6);
pub const SCALER_DISPCTRL_SLVWREIRQ: usize = BIT(5);
pub const SCALER_DISPCTRL_DMAEIRQ: usize = BIT(4);
pub const SCALER_DISPCTRL_DISP2EIRQ: usize = BIT(3);
pub const SCALER_DISPCTRL_DISP1EIRQ: usize = BIT(2);
/* Enables interrupt generation on the enabled EOF/EOLN/EISLUR
 * bits and short frames..
 */
pub const SCALER_DISPCTRL_DISP0EIRQ: usize = BIT(1);
/* Enables interrupt generation on scaler profiler interrupt. */
pub const SCALER_DISPCTRL_SCLEIRQ: usize = BIT(0);

pub const SCALER_DISPSTAT: usize = 0x00000004;
pub const SCALER_DISPSTAT_COBLOW2: usize = BIT(29);
pub const SCALER_DISPSTAT_EOLN2: usize = BIT(28);
pub const SCALER_DISPSTAT_ESFRAME2: usize = BIT(27);
pub const SCALER_DISPSTAT_ESLINE2: usize = BIT(26);
pub const SCALER_DISPSTAT_EUFLOW2: usize = BIT(25);
pub const SCALER_DISPSTAT_EOF2: usize = BIT(24);

pub const SCALER_DISPSTAT_COBLOW1: usize = BIT(21);
pub const SCALER_DISPSTAT_EOLN1: usize = BIT(20);
pub const SCALER_DISPSTAT_ESFRAME1: usize = BIT(19);
pub const SCALER_DISPSTAT_ESLINE1: usize = BIT(18);
pub const SCALER_DISPSTAT_EUFLOW1: usize = BIT(17);
pub const SCALER_DISPSTAT_EOF1: usize = BIT(16);

pub const SCALER_DISPSTAT_RESP_MASK: usize = VC4_MASK(15, 14);
pub const SCALER_DISPSTAT_RESP_SHIFT: usize = 14;
pub const SCALER_DISPSTAT_RESP_OKAY: usize = 0;
pub const SCALER_DISPSTAT_RESP_EXOKAY: usize = 1;
pub const SCALER_DISPSTAT_RESP_SLVERR: usize = 2;
pub const SCALER_DISPSTAT_RESP_DECERR: usize = 3;

pub const SCALER_DISPSTAT_COBLOW0: usize = BIT(13);
/* Set when the DISPEOLN line is done compositing. */
pub const SCALER_DISPSTAT_EOLN0: usize = BIT(12);
/* Set when VSTART is seen but there are still pixels in the current
 * output line.
 */
pub const SCALER_DISPSTAT_ESFRAME0: usize = BIT(11);
/* Set when HSTART is seen but there are still pixels in the current
 * output line.
 */
pub const SCALER_DISPSTAT_ESLINE0: usize = BIT(10);
/* Set when the the downstream tries to read from the display FIFO
 * while it's empty.
 */
pub const SCALER_DISPSTAT_EUFLOW0: usize = BIT(9);
/* Set when the display mode changes from RUN to EOF */
pub const SCALER_DISPSTAT_EOF0: usize = BIT(8);

/* Set on AXI invalid DMA ID error. */
pub const SCALER_DISPSTAT_DMA_ERROR: usize = BIT(7);
/* Set on AXI slave read decode error */
pub const SCALER_DISPSTAT_IRQSLVRD: usize = BIT(6);
/* Set on AXI slave write decode error */
pub const SCALER_DISPSTAT_IRQSLVWR: usize = BIT(5);
/* Set when SCALER_DISPSTAT_DMA_ERROR is set, or
 * SCALER_DISPSTAT_RESP_ERROR is not SCALER_DISPSTAT_RESP_OKAY.
 */
pub const SCALER_DISPSTAT_IRQDMA: usize = BIT(4);
pub const SCALER_DISPSTAT_IRQDISP2: usize = BIT(3);
pub const SCALER_DISPSTAT_IRQDISP1: usize = BIT(2);
/* Set when any of the EOF/EOLN/ESFRAME/ESLINE bits are set and their
 * corresponding interrupt bit is enabled in DISPCTRL.
 */
pub const SCALER_DISPSTAT_IRQDISP0: usize = BIT(1);
/* On read, the profiler interrupt.  On write, clear *all* interrupt bits. */
pub const SCALER_DISPSTAT_IRQSCL: usize = BIT(0);


pub const SCALER_DISPID : usize = 0x00000008;
pub const SCALER_DISPECTRL : usize = 0x0000000c;
pub const SCALER_DISPPROF : usize = 0x00000010;
pub const SCALER_DISPDITHER : usize = 0x00000014;
pub const SCALER_DISPEOLN : usize = 0x00000018;
pub const SCALER_DISPLIST0 : usize = 0x00000020;
pub const SCALER_DISPLIST1 : usize = 0x00000024;
pub const SCALER_DISPLIST2 : usize = 0x00000028;
pub const SCALER_DISPLSTAT : usize = 0x0000002c;

pub const fn SCALER_DISPLISTX(x: usize) -> usize {
	(SCALER_DISPLIST0 +	(x) * (SCALER_DISPLIST1 - SCALER_DISPLIST0))
}

pub const SCALER_DISPLACT0 : usize = 0x00000030;
pub const SCALER_DISPLACT1 : usize = 0x00000034;
pub const SCALER_DISPLACT2 : usize = 0x00000038;
pub const fn SCALER_DISPLACTX(x: usize) -> usize {
	(SCALER_DISPLACT0 +	 (x) * (SCALER_DISPLACT1 - SCALER_DISPLACT0))
}

pub const SCALER_DISPCTRL0 : usize = 0x00000040;
pub const SCALER_DISPCTRLX_ENABLE : usize = BIT(31);
pub const SCALER_DISPCTRLX_RESET : usize = BIT(30);
pub const SCALER_DISPCTRLX_WIDTH_MASK : usize = VC4_MASK(23, 12);
pub const SCALER_DISPCTRLX_WIDTH_SHIFT : usize = 12;
pub const SCALER_DISPCTRLX_HEIGHT_MASK : usize = VC4_MASK(11, 0);
pub const SCALER_DISPCTRLX_HEIGHT_SHIFT : usize = 0;

pub const SCALER_DISPBKGND0 : usize = 0x00000044;
pub const SCALER_DISPBKGND_AUTOHS : usize = BIT(31);
pub const SCALER_DISPBKGND_INTERLACE : usize = BIT(30);
pub const SCALER_DISPBKGND_GAMMA : usize = BIT(29);
pub const SCALER_DISPBKGND_TESTMODE_MASK : usize = VC4_MASK(28, 25);
pub const SCALER_DISPBKGND_TESTMODE_SHIFT : usize = 25;
/* Enables filling the scaler line with the RGB value in the low 24
 * bits before compositing. Costs cycles, so should be skipped if
 * opaque display planes will cover everything.
 */
pub const SCALER_DISPBKGND_FILL : usize = BIT(24);

pub const SCALER_DISPSTAT0 : usize = 0x00000048;
pub const SCALER_DISPSTATX_MODE_MASK : usize = VC4_MASK(31, 30);
pub const SCALER_DISPSTATX_MODE_SHIFT : usize = 30;
pub const SCALER_DISPSTATX_MODE_DISABLED : usize = 0;
pub const SCALER_DISPSTATX_MODE_INIT : usize = 1;
pub const SCALER_DISPSTATX_MODE_RUN : usize = 2;
pub const SCALER_DISPSTATX_MODE_EOF : usize = 3;
pub const SCALER_DISPSTATX_FULL : usize = BIT(29);
pub const SCALER_DISPSTATX_EMPTY : usize = BIT(28);
pub const SCALER_DISPSTATX_FRAME_COUNT_MASK : usize = VC4_MASK(17, 12);
pub const SCALER_DISPSTATX_FRAME_COUNT_SHIFT : usize = 12;
pub const SCALER_DISPSTATX_LINE_MASK : usize = VC4_MASK(11, 0);
pub const SCALER_DISPSTATX_LINE_SHIFT : usize = 0;

pub const SCALER_DISPBASE0 : usize = 0x0000004c;
/* Last pixel in the COB (display FIFO memory) allocated to this HVS
 * channel. Must be 4-pixel aligned (and thus 4 pixels less than the
 * next COB base).
 */
pub const SCALER_DISPBASEX_TOP_MASK : usize = VC4_MASK(31, 16);
pub const SCALER_DISPBASEX_TOP_SHIFT : usize = 16;
/* First pixel in the COB (display FIFO memory) allocated to this HVS
 * channel. Must be 4-pixel aligned.
 */
pub const SCALER_DISPBASEX_BASE_MASK : usize = VC4_MASK(15, 0);
pub const SCALER_DISPBASEX_BASE_SHIFT : usize = 0;

pub const SCALER_DISPCTRL1 : usize = 0x00000050;
pub const SCALER_DISPBKGND1 : usize = 0x00000054;

pub const fn SCALER_DISPBKGNDX(x: usize) -> usize {
				(SCALER_DISPBKGND0 + (x) * (SCALER_DISPBKGND1 - SCALER_DISPBKGND0))
}
pub const SCALER_DISPSTAT1 : usize = 0x00000058;
pub const fn SCALER_DISPSTATX(x: usize) -> usize {
				(SCALER_DISPSTAT0 + (x) * (SCALER_DISPSTAT1 - SCALER_DISPSTAT0))
}
pub const SCALER_DISPBASE1 : usize = 0x0000005c;
pub const fn SCALER_DISPBASEX(x: usize) -> usize {
				(SCALER_DISPBASE0 + (x) * (SCALER_DISPBASE1 - SCALER_DISPBASE0))
}
pub const SCALER_DISPCTRL2 : usize = 0x00000060;
pub const fn SCALER_DISPCTRLX(x: usize) -> usize {
				(SCALER_DISPCTRL0 + (x) * (SCALER_DISPCTRL1 - SCALER_DISPCTRL0))
}
pub const SCALER_DISPBKGND2 : usize = 0x00000064;
pub const SCALER_DISPSTAT2 : usize = 0x00000068;
pub const SCALER_DISPBASE2 : usize = 0x0000006c;
pub const SCALER_DISPALPHA2 : usize = 0x00000070;
pub const SCALER_GAMADDR : usize = 0x00000078;
pub const SCALER_GAMADDR_AUTOINC : usize = BIT(31);
/* Enables all gamma ramp SRAMs, not just those of CRTCs with gamma
 * enabled.
 */
pub const SCALER_GAMADDR_SRAMENB : usize = BIT(30);

pub const SCALER_GAMDATA : usize = 0x000000e0;
pub const SCALER_DLIST_START : usize = 0x00002000;
pub const SCALER_DLIST_SIZE : usize = 0x00004000;

pub const VC4_HDMI_CORE_REV : usize = 0x000;

pub const VC4_HDMI_SW_RESET_CONTROL : usize = 0x004;
pub const VC4_HDMI_SW_RESET_FORMAT_DETECT : usize = BIT(1);
pub const VC4_HDMI_SW_RESET_HDMI : usize = BIT(0);

pub const VC4_HDMI_HOTPLUG_INT : usize = 0x008;

pub const VC4_HDMI_HOTPLUG : usize = 0x00c;
pub const VC4_HDMI_HOTPLUG_CONNECTED : usize = BIT(0);

/* 3 bits per field, where each field maps from that corresponding MAI
 * bus channel to the given HDMI channel.
 */
pub const VC4_HDMI_MAI_CHANNEL_MAP : usize = 0x090;

pub const VC4_HDMI_MAI_CONFIG : usize = 0x094;
pub const VC4_HDMI_MAI_CONFIG_FORMAT_REVERSE : usize = BIT(27);
pub const VC4_HDMI_MAI_CONFIG_BIT_REVERSE : usize = BIT(26);
pub const VC4_HDMI_MAI_CHANNEL_MASK_MASK : usize = VC4_MASK(15, 0);
pub const VC4_HDMI_MAI_CHANNEL_MASK_SHIFT : usize = 0;

/* Last received format word on the MAI bus. */
pub const VC4_HDMI_MAI_FORMAT : usize = 0x098;

pub const VC4_HDMI_AUDIO_PACKET_CONFIG : usize = 0x09c;
pub const VC4_HDMI_AUDIO_PACKET_ZERO_DATA_ON_SAMPLE_FLAT : usize = BIT(29);
pub const VC4_HDMI_AUDIO_PACKET_ZERO_DATA_ON_INACTIVE_CHANNELS : usize = BIT(24);
pub const VC4_HDMI_AUDIO_PACKET_FORCE_SAMPLE_PRESENT : usize = BIT(19);
pub const VC4_HDMI_AUDIO_PACKET_FORCE_B_FRAME : usize = BIT(18);
pub const VC4_HDMI_AUDIO_PACKET_B_FRAME_IDENTIFIER_MASK : usize = VC4_MASK(13, 10);
pub const VC4_HDMI_AUDIO_PACKET_B_FRAME_IDENTIFIER_SHIFT : usize = 10;
/* If set, then multichannel, otherwise 2 channel. */
pub const VC4_HDMI_AUDIO_PACKET_AUDIO_LAYOUT : usize = BIT(9);
/* If set, then AUDIO_LAYOUT overrides audio_cea_mask */
pub const VC4_HDMI_AUDIO_PACKET_FORCE_AUDIO_LAYOUT : usize = BIT(8);
pub const VC4_HDMI_AUDIO_PACKET_CEA_MASK_MASK : usize = VC4_MASK(7, 0);
pub const VC4_HDMI_AUDIO_PACKET_CEA_MASK_SHIFT : usize = 0;

pub const VC4_HDMI_RAM_PACKET_CONFIG : usize = 0x0a0;
pub const VC4_HDMI_RAM_PACKET_ENABLE : usize = BIT(16);

pub const VC4_HDMI_RAM_PACKET_STATUS : usize = 0x0a4;

pub const VC4_HDMI_CRP_CFG : usize = 0x0a8;
/* When set, the CTS_PERIOD counts based on MAI bus sync pulse instead
 * of pixel clock.
 */
pub const VC4_HDMI_CRP_USE_MAI_BUS_SYNC_FOR_CTS : usize = BIT(26);
/* When set, no CRP packets will be sent. */
pub const VC4_HDMI_CRP_CFG_DISABLE : usize = BIT(25);
/* If set, generates CTS values based on N, audio clock, and video
 * clock.  N must be divisible by 128.
 */
pub const VC4_HDMI_CRP_CFG_EXTERNAL_CTS_EN : usize = BIT(24);
pub const VC4_HDMI_CRP_CFG_N_MASK : usize = VC4_MASK(19, 0);
pub const VC4_HDMI_CRP_CFG_N_SHIFT : usize = 0;

/* 20-bit fields containing CTS values to be transmitted if !EXTERNAL_CTS_EN */
pub const VC4_HDMI_CTS_0 : usize = 0x0ac;
pub const VC4_HDMI_CTS_1 : usize = 0x0b0;
/* 20-bit fields containing number of clocks to send CTS0/1 before
 * switching to the other one.
 */
pub const VC4_HDMI_CTS_PERIOD_0 : usize = 0x0b4;
pub const VC4_HDMI_CTS_PERIOD_1 : usize = 0x0b8;

pub const VC4_HDMI_HORZA : usize = 0x0c4;
pub const VC4_HDMI_HORZA_VPOS : usize = BIT(14);
pub const VC4_HDMI_HORZA_HPOS : usize = BIT(13);
/* Horizontal active pixels (hdisplay). */
pub const VC4_HDMI_HORZA_HAP_MASK : usize = VC4_MASK(12, 0);
pub const VC4_HDMI_HORZA_HAP_SHIFT : usize = 0;

pub const VC4_HDMI_HORZB : usize = 0x0c8;
/* Horizontal pack porch (htotal - hsync_end). */
pub const VC4_HDMI_HORZB_HBP_MASK : usize = VC4_MASK(29, 20);
pub const VC4_HDMI_HORZB_HBP_SHIFT : usize = 20;
/* Horizontal sync pulse (hsync_end - hsync_start). */
pub const VC4_HDMI_HORZB_HSP_MASK : usize = VC4_MASK(19, 10);
pub const VC4_HDMI_HORZB_HSP_SHIFT : usize = 10;
/* Horizontal front porch (hsync_start - hdisplay). */
pub const VC4_HDMI_HORZB_HFP_MASK : usize = VC4_MASK(9, 0);
pub const VC4_HDMI_HORZB_HFP_SHIFT : usize = 0;

pub const VC4_HDMI_FIFO_CTL : usize = 0x05c;
pub const VC4_HDMI_FIFO_CTL_RECENTER_DONE : usize = BIT(14);
pub const VC4_HDMI_FIFO_CTL_USE_EMPTY : usize = BIT(13);
pub const VC4_HDMI_FIFO_CTL_ON_VB : usize = BIT(7);
pub const VC4_HDMI_FIFO_CTL_RECENTER : usize = BIT(6);
pub const VC4_HDMI_FIFO_CTL_FIFO_RESET : usize = BIT(5);
pub const VC4_HDMI_FIFO_CTL_USE_PLL_LOCK : usize = BIT(4);
pub const VC4_HDMI_FIFO_CTL_INV_CLK_XFR : usize = BIT(3);
pub const VC4_HDMI_FIFO_CTL_CAPTURE_PTR : usize = BIT(2);
pub const VC4_HDMI_FIFO_CTL_USE_FULL : usize = BIT(1);
pub const VC4_HDMI_FIFO_CTL_MASTER_SLAVE_N : usize = BIT(0);
pub const VC4_HDMI_FIFO_VALID_WRITE_MASK : usize = 0xefff;

pub const VC4_HDMI_SCHEDULER_CONTROL : usize = 0x0c0;
pub const VC4_HDMI_SCHEDULER_CONTROL_MANUAL_FORMAT : usize = BIT(15);
pub const VC4_HDMI_SCHEDULER_CONTROL_IGNORE_VSYNC_PREDICTS : usize = BIT(5);
pub const VC4_HDMI_SCHEDULER_CONTROL_VERT_ALWAYS_KEEPOUT : usize = BIT(3);
pub const VC4_HDMI_SCHEDULER_CONTROL_HDMI_ACTIVE : usize = BIT(1);
pub const VC4_HDMI_SCHEDULER_CONTROL_MODE_HDMI : usize = BIT(0);

pub const VC4_HDMI_VERTA0 : usize = 0x0cc;
pub const VC4_HDMI_VERTA1 : usize = 0x0d4;
/* Vertical sync pulse (vsync_end - vsync_start). */
pub const VC4_HDMI_VERTA_VSP_MASK : usize = VC4_MASK(24, 20);
pub const VC4_HDMI_VERTA_VSP_SHIFT : usize = 20;
/* Vertical front porch (vsync_start - vdisplay). */
pub const VC4_HDMI_VERTA_VFP_MASK : usize = VC4_MASK(19, 13);
pub const VC4_HDMI_VERTA_VFP_SHIFT : usize = 13;
/* Vertical active lines (vdisplay). */
pub const VC4_HDMI_VERTA_VAL_MASK : usize = VC4_MASK(12, 0);
pub const VC4_HDMI_VERTA_VAL_SHIFT : usize = 0;

pub const VC4_HDMI_VERTB0 : usize = 0x0d0;
pub const VC4_HDMI_VERTB1 : usize = 0x0d8;
/* Vertical sync pulse offset (for interlaced) */
pub const VC4_HDMI_VERTB_VSPO_MASK : usize = VC4_MASK(21, 9);
pub const VC4_HDMI_VERTB_VSPO_SHIFT : usize = 9;
/* Vertical pack porch (vtotal - vsync_end). */
pub const VC4_HDMI_VERTB_VBP_MASK : usize = VC4_MASK(8, 0);
pub const VC4_HDMI_VERTB_VBP_SHIFT : usize = 0;

pub const VC4_HDMI_TX_PHY_RESET_CTL : usize = 0x2c0;

pub const VC4_HDMI_TX_PHY_CTL0 : usize = 0x2c4;
pub const VC4_HDMI_TX_PHY_RNG_PWRDN : usize = BIT(25);

pub const fn VC4_HDMI_GCP(x : usize) -> usize {
	(0x400 + ((x) * 0x4))
}
pub const fn VC4_HDMI_RAM_PACKET(x: usize) -> usize {
	(0x400 + ((x) * 0x24))
}
pub const VC4_HDMI_PACKET_STRIDE : usize = 0x24;

pub const VC4_HD_M_CTL : usize = 0x00c;
pub const VC4_HD_M_REGISTER_FILE_STANDBY : usize = (3 << 6);
pub const VC4_HD_M_RAM_STANDBY : usize = (3 << 4);
pub const VC4_HD_M_SW_RST : usize = BIT(2);
pub const VC4_HD_M_ENABLE : usize = BIT(0);

pub const VC4_HD_MAI_CTL : usize = 0x014;
/* Set when audio stream is received at a slower rate than the
 * sampling period, so MAI fifo goes empty.  Write 1 to clear.
 */
pub const VC4_HD_MAI_CTL_DLATE : usize = BIT(15);
pub const VC4_HD_MAI_CTL_BUSY : usize = BIT(14);
pub const VC4_HD_MAI_CTL_CHALIGN : usize = BIT(13);
pub const VC4_HD_MAI_CTL_WHOLSMP : usize = BIT(12);
pub const VC4_HD_MAI_CTL_FULL : usize = BIT(11);
pub const VC4_HD_MAI_CTL_EMPTY : usize = BIT(10);
pub const VC4_HD_MAI_CTL_FLUSH : usize = BIT(9);
/* If set, MAI bus generates SPDIF (bit 31) parity instead of passing
 * through.
 */
pub const VC4_HD_MAI_CTL_PAREN : usize = BIT(8);
pub const VC4_HD_MAI_CTL_CHNUM_MASK : usize = VC4_MASK(7, 4);
pub const VC4_HD_MAI_CTL_CHNUM_SHIFT : usize = 4;
pub const VC4_HD_MAI_CTL_ENABLE : usize = BIT(3);
/* Underflow error status bit, write 1 to clear. */
pub const VC4_HD_MAI_CTL_ERRORE : usize = BIT(2);
/* Overflow error status bit, write 1 to clear. */
pub const VC4_HD_MAI_CTL_ERRORF : usize = BIT(1);
/* Single-shot reset bit.  Read value is undefined. */
pub const VC4_HD_MAI_CTL_RESET : usize = BIT(0);

pub const VC4_HD_MAI_THR : usize = 0x018;
pub const VC4_HD_MAI_THR_PANICHIGH_MASK : usize = VC4_MASK(29, 24);
pub const VC4_HD_MAI_THR_PANICHIGH_SHIFT : usize = 24;
pub const VC4_HD_MAI_THR_PANICLOW_MASK : usize = VC4_MASK(21, 16);
pub const VC4_HD_MAI_THR_PANICLOW_SHIFT : usize = 16;
pub const VC4_HD_MAI_THR_DREQHIGH_MASK : usize = VC4_MASK(13, 8);
pub const VC4_HD_MAI_THR_DREQHIGH_SHIFT : usize = 8;
pub const VC4_HD_MAI_THR_DREQLOW_MASK : usize = VC4_MASK(5, 0);
pub const VC4_HD_MAI_THR_DREQLOW_SHIFT : usize = 0;

/* Format header to be placed on the MAI data. Unused. */
pub const VC4_HD_MAI_FMT : usize = 0x01c;

/* Register for DMAing in audio data to be transported over the MAI
 * bus to the Falcon core.
 */
pub const VC4_HD_MAI_DATA : usize = 0x020;

/* Divider from HDMI HSM clock to MAI serial clock.  Sampling period
 * converges to N / (M + 1) cycles.
 */
pub const VC4_HD_MAI_SMP : usize = 0x02c;
pub const VC4_HD_MAI_SMP_N_MASK : usize = VC4_MASK(31, 8);
pub const VC4_HD_MAI_SMP_N_SHIFT : usize = 8;
pub const VC4_HD_MAI_SMP_M_MASK : usize = VC4_MASK(7, 0);
pub const VC4_HD_MAI_SMP_M_SHIFT : usize = 0;

pub const VC4_HD_VID_CTL : usize = 0x038;
pub const VC4_HD_VID_CTL_ENABLE : usize = BIT(31);
pub const VC4_HD_VID_CTL_UNDERFLOW_ENABLE : usize = BIT(30);
pub const VC4_HD_VID_CTL_FRAME_COUNTER_RESET : usize = BIT(29);
pub const VC4_HD_VID_CTL_VSYNC_LOW : usize = BIT(28);
pub const VC4_HD_VID_CTL_HSYNC_LOW : usize = BIT(27);

pub const VC4_HD_CSC_CTL : usize = 0x040;
pub const VC4_HD_CSC_CTL_ORDER_MASK : usize = VC4_MASK(7, 5);
pub const VC4_HD_CSC_CTL_ORDER_SHIFT : usize = 5;
pub const VC4_HD_CSC_CTL_ORDER_RGB : usize = 0;
pub const VC4_HD_CSC_CTL_ORDER_BGR : usize = 1;
pub const VC4_HD_CSC_CTL_ORDER_BRG : usize = 2;
pub const VC4_HD_CSC_CTL_ORDER_GRB : usize = 3;
pub const VC4_HD_CSC_CTL_ORDER_GBR : usize = 4;
pub const VC4_HD_CSC_CTL_ORDER_RBG : usize = 5;
pub const VC4_HD_CSC_CTL_PADMSB : usize = BIT(4);
pub const VC4_HD_CSC_CTL_MODE_MASK : usize = VC4_MASK(3, 2);
pub const VC4_HD_CSC_CTL_MODE_SHIFT : usize = 2;
pub const VC4_HD_CSC_CTL_MODE_RGB_TO_SD_YPRPB : usize = 0;
pub const VC4_HD_CSC_CTL_MODE_RGB_TO_HD_YPRPB : usize = 1;
pub const VC4_HD_CSC_CTL_MODE_CUSTOM : usize = 3;
pub const VC4_HD_CSC_CTL_RGB2YCC : usize = BIT(1);
pub const VC4_HD_CSC_CTL_ENABLE : usize = BIT(0);

pub const VC4_HD_CSC_12_11 : usize = 0x044;
pub const VC4_HD_CSC_14_13 : usize = 0x048;
pub const VC4_HD_CSC_22_21 : usize = 0x04c;
pub const VC4_HD_CSC_24_23 : usize = 0x050;
pub const VC4_HD_CSC_32_31 : usize = 0x054;
pub const VC4_HD_CSC_34_33 : usize = 0x058;

pub const VC4_HD_FRAME_COUNT : usize = 0x068;

/* HVS display list information. */
pub const HVS_BOOTLOADER_DLIST_END : usize = 32;

#[allow(non_camel_case_types)]
enum hvs_pixel_format {
	/* 8bpp */
	HVS_PIXEL_FORMAT_RGB332 = 0,
	/* 16bpp */
	HVS_PIXEL_FORMAT_RGBA4444 = 1,
	HVS_PIXEL_FORMAT_RGB555 = 2,
	HVS_PIXEL_FORMAT_RGBA5551 = 3,
	HVS_PIXEL_FORMAT_RGB565 = 4,
	/* 24bpp */
	HVS_PIXEL_FORMAT_RGB888 = 5,
	HVS_PIXEL_FORMAT_RGBA6666 = 6,
	/* 32bpp */
	HVS_PIXEL_FORMAT_RGBA8888 = 7,

	HVS_PIXEL_FORMAT_YCBCR_YUV420_3PLANE = 8,
	HVS_PIXEL_FORMAT_YCBCR_YUV420_2PLANE = 9,
	HVS_PIXEL_FORMAT_YCBCR_YUV422_3PLANE = 10,
	HVS_PIXEL_FORMAT_YCBCR_YUV422_2PLANE = 11,
}

/* Note: the LSB is the rightmost character shown.  Only valid for
 * HVS_PIXEL_FORMAT_RGB8888, not RGB888.
 */
pub const HVS_PIXEL_ORDER_RGBA : usize = 0;
pub const HVS_PIXEL_ORDER_BGRA : usize = 1;
pub const HVS_PIXEL_ORDER_ARGB : usize = 2;
pub const HVS_PIXEL_ORDER_ABGR : usize = 3;

pub const HVS_PIXEL_ORDER_XBRG : usize = 0;
pub const HVS_PIXEL_ORDER_XRBG : usize = 1;
pub const HVS_PIXEL_ORDER_XRGB : usize = 2;
pub const HVS_PIXEL_ORDER_XBGR : usize = 3;

pub const HVS_PIXEL_ORDER_XYCBCR : usize = 0;
pub const HVS_PIXEL_ORDER_XYCRCB : usize = 1;
pub const HVS_PIXEL_ORDER_YXCBCR : usize = 2;
pub const HVS_PIXEL_ORDER_YXCRCB : usize = 3;

pub const SCALER_CTL0_END : usize = BIT(31);
pub const SCALER_CTL0_VALID : usize = BIT(30);

pub const SCALER_CTL0_SIZE_MASK : usize = VC4_MASK(29, 24);
pub const SCALER_CTL0_SIZE_SHIFT : usize = 24;

pub const SCALER_CTL0_TILING_MASK : usize = VC4_MASK(21, 20);
pub const SCALER_CTL0_TILING_SHIFT : usize = 20;
pub const SCALER_CTL0_TILING_LINEAR : usize = 0;
pub const SCALER_CTL0_TILING_64B : usize = 1;
pub const SCALER_CTL0_TILING_128B : usize = 2;
pub const SCALER_CTL0_TILING_256B_OR_T : usize = 3;

pub const SCALER_CTL0_HFLIP : usize = BIT(16);
pub const SCALER_CTL0_VFLIP : usize = BIT(15);

pub const SCALER_CTL0_ORDER_MASK : usize = VC4_MASK(14, 13);
pub const SCALER_CTL0_ORDER_SHIFT : usize = 13;

pub const SCALER_CTL0_SCL1_MASK : usize = VC4_MASK(10, 8);
pub const SCALER_CTL0_SCL1_SHIFT : usize = 8;

pub const SCALER_CTL0_SCL0_MASK : usize = VC4_MASK(7, 5);
pub const SCALER_CTL0_SCL0_SHIFT : usize = 5;

pub const SCALER_CTL0_SCL_H_PPF_V_PPF : usize = 0;
pub const SCALER_CTL0_SCL_H_TPZ_V_PPF : usize = 1;
pub const SCALER_CTL0_SCL_H_PPF_V_TPZ : usize = 2;
pub const SCALER_CTL0_SCL_H_TPZ_V_TPZ : usize = 3;
pub const SCALER_CTL0_SCL_H_PPF_V_NONE : usize = 4;
pub const SCALER_CTL0_SCL_H_NONE_V_PPF : usize = 5;
pub const SCALER_CTL0_SCL_H_NONE_V_TPZ : usize = 6;
pub const SCALER_CTL0_SCL_H_TPZ_V_NONE : usize = 7;

/* Set to indicate no scaling. */
pub const SCALER_CTL0_UNITY : usize = BIT(4);

pub const SCALER_CTL0_PIXEL_FORMAT_MASK : usize = VC4_MASK(3, 0);
pub const SCALER_CTL0_PIXEL_FORMAT_SHIFT : usize = 0;

pub const SCALER_POS0_FIXED_ALPHA_MASK : usize = VC4_MASK(31, 24);
pub const SCALER_POS0_FIXED_ALPHA_SHIFT : usize = 24;

pub const SCALER_POS0_START_Y_MASK : usize = VC4_MASK(23, 12);
pub const SCALER_POS0_START_Y_SHIFT : usize = 12;

pub const SCALER_POS0_START_X_MASK : usize = VC4_MASK(11, 0);
pub const SCALER_POS0_START_X_SHIFT : usize = 0;

pub const SCALER_POS1_SCL_HEIGHT_MASK : usize = VC4_MASK(27, 16);
pub const SCALER_POS1_SCL_HEIGHT_SHIFT : usize = 16;

pub const SCALER_POS1_SCL_WIDTH_MASK : usize = VC4_MASK(11, 0);
pub const SCALER_POS1_SCL_WIDTH_SHIFT : usize = 0;

pub const SCALER_POS2_ALPHA_MODE_MASK : usize = VC4_MASK(31, 30);
pub const SCALER_POS2_ALPHA_MODE_SHIFT : usize = 30;
pub const SCALER_POS2_ALPHA_MODE_PIPELINE : usize = 0;
pub const SCALER_POS2_ALPHA_MODE_FIXED : usize = 1;
pub const SCALER_POS2_ALPHA_MODE_FIXED_NONZERO : usize = 2;
pub const SCALER_POS2_ALPHA_MODE_FIXED_OVER_0x07 : usize = 3;

pub const SCALER_POS2_HEIGHT_MASK : usize = VC4_MASK(27, 16);
pub const SCALER_POS2_HEIGHT_SHIFT : usize = 16;

pub const SCALER_POS2_WIDTH_MASK : usize = VC4_MASK(11, 0);
pub const SCALER_POS2_WIDTH_SHIFT : usize = 0;

/* Color Space Conversion words.  Some values are S2.8 signed
 * integers, except that the 2 integer bits map as {0x0: 0, 0x1: 1,
 * 0x2: 2, 0x3: -1}
 */
/* bottom 8 bits of S2.8 contribution of Cr to Blue */
pub const SCALER_CSC0_COEF_CR_BLU_MASK : usize = VC4_MASK(31, 24);
pub const SCALER_CSC0_COEF_CR_BLU_SHIFT : usize = 24;
/* Signed offset to apply to Y before CSC. (Y' = Y + YY_OFS) */
pub const SCALER_CSC0_COEF_YY_OFS_MASK : usize = VC4_MASK(23, 16);
pub const SCALER_CSC0_COEF_YY_OFS_SHIFT : usize = 16;
/* Signed offset to apply to CB before CSC (Cb' = Cb - 128 + CB_OFS). */
pub const SCALER_CSC0_COEF_CB_OFS_MASK : usize = VC4_MASK(15, 8);
pub const SCALER_CSC0_COEF_CB_OFS_SHIFT : usize = 8;
/* Signed offset to apply to CB before CSC (Cr' = Cr - 128 + CR_OFS). */
pub const SCALER_CSC0_COEF_CR_OFS_MASK : usize = VC4_MASK(7, 0);
pub const SCALER_CSC0_COEF_CR_OFS_SHIFT : usize = 0;
pub const SCALER_CSC0_ITR_R_601_5 : usize = 0x00f00000;
pub const SCALER_CSC0_ITR_R_709_3 : usize = 0x00f00000;
pub const SCALER_CSC0_JPEG_JFIF : usize = 0x00000000;

/* S2.8 contribution of Cb to Green */
pub const SCALER_CSC1_COEF_CB_GRN_MASK : usize = VC4_MASK(31, 22);
pub const SCALER_CSC1_COEF_CB_GRN_SHIFT : usize = 22;
/* S2.8 contribution of Cr to Green */
pub const SCALER_CSC1_COEF_CR_GRN_MASK : usize = VC4_MASK(21, 12);
pub const SCALER_CSC1_COEF_CR_GRN_SHIFT : usize = 12;
/* S2.8 contribution of Y to all of RGB */
pub const SCALER_CSC1_COEF_YY_ALL_MASK : usize = VC4_MASK(11, 2);
pub const SCALER_CSC1_COEF_YY_ALL_SHIFT : usize = 2;
/* top 2 bits of S2.8 contribution of Cr to Blue */
pub const SCALER_CSC1_COEF_CR_BLU_MASK : usize = VC4_MASK(1, 0);
pub const SCALER_CSC1_COEF_CR_BLU_SHIFT : usize = 0;
pub const SCALER_CSC1_ITR_R_601_5 : usize = 0xe73304a8;
pub const SCALER_CSC1_ITR_R_709_3 : usize = 0xf2b784a8;
pub const SCALER_CSC1_JPEG_JFIF : usize = 0xea34a400;

/* S2.8 contribution of Cb to Red */
pub const SCALER_CSC2_COEF_CB_RED_MASK : usize = VC4_MASK(29, 20);
pub const SCALER_CSC2_COEF_CB_RED_SHIFT : usize = 20;
/* S2.8 contribution of Cr to Red */
pub const SCALER_CSC2_COEF_CR_RED_MASK : usize = VC4_MASK(19, 10);
pub const SCALER_CSC2_COEF_CR_RED_SHIFT : usize = 10;
/* S2.8 contribution of Cb to Blue */
pub const SCALER_CSC2_COEF_CB_BLU_MASK : usize = VC4_MASK(19, 10);
pub const SCALER_CSC2_COEF_CB_BLU_SHIFT : usize = 10;
pub const SCALER_CSC2_ITR_R_601_5 : usize = 0x00066204;
pub const SCALER_CSC2_ITR_R_709_3 : usize = 0x00072a1c;
pub const SCALER_CSC2_JPEG_JFIF : usize = 0x000599c5;

pub const SCALER_TPZ0_VERT_RECALC : usize = BIT(31);
pub const SCALER_TPZ0_SCALE_MASK : usize = VC4_MASK(28, 8);
pub const SCALER_TPZ0_SCALE_SHIFT : usize = 8;
pub const SCALER_TPZ0_IPHASE_MASK : usize = VC4_MASK(7, 0);
pub const SCALER_TPZ0_IPHASE_SHIFT : usize = 0;
pub const SCALER_TPZ1_RECIP_MASK : usize = VC4_MASK(15, 0);
pub const SCALER_TPZ1_RECIP_SHIFT : usize = 0;

/* Skips interpolating coefficients to 64 phases, so just 8 are used.
 * Required for nearest neighbor.;
 */
pub const SCALER_PPF_NOINTERP : usize = BIT(31);
/* Replaes the highest valued coefficient with one that makes all 4
 * sum to unity.
 */
pub const SCALER_PPF_AGC : usize = BIT(30);
pub const SCALER_PPF_SCALE_MASK : usize = VC4_MASK(24, 8);
pub const SCALER_PPF_SCALE_SHIFT : usize = 8;
pub const SCALER_PPF_IPHASE_MASK : usize = VC4_MASK(6, 0);
pub const SCALER_PPF_IPHASE_SHIFT : usize = 0;

pub const SCALER_PPF_KERNEL_OFFSET_MASK : usize = VC4_MASK(13, 0);
pub const SCALER_PPF_KERNEL_OFFSET_SHIFT : usize = 0;
pub const SCALER_PPF_KERNEL_UNCACHED : usize = BIT(31);

/* PITCH0/1/2 fields for raster. */
pub const SCALER_SRC_PITCH_MASK : usize = VC4_MASK(15, 0);
pub const SCALER_SRC_PITCH_SHIFT : usize = 0;

/* PITCH0 fields for T-tiled. */
pub const SCALER_PITCH0_TILE_WIDTH_L_MASK : usize = VC4_MASK(22, 16);
pub const SCALER_PITCH0_TILE_WIDTH_L_SHIFT : usize = 16;
pub const SCALER_PITCH0_TILE_LINE_DIR : usize = BIT(15);
pub const SCALER_PITCH0_TILE_INITIAL_LINE_DIR : usize = BIT(14);
/* Y offset within a tile. */
pub const SCALER_PITCH0_TILE_Y_OFFSET_MASK : usize = VC4_MASK(13, 7);
pub const SCALER_PITCH0_TILE_Y_OFFSET_SHIFT : usize = 7;
pub const SCALER_PITCH0_TILE_WIDTH_R_MASK : usize = VC4_MASK(6, 0);
pub const SCALER_PITCH0_TILE_WIDTH_R_SHIFT : usize = 0;