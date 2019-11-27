//! Raspberry PI 3 Model B/B+

use bcm2837::{addr::bus_to_phys, atags::Atags};

pub mod emmc;
pub mod irq;
pub mod mailbox;
pub mod serial;
pub mod timer;
pub mod vc4;

use crate::drivers::gpu::fb::{self, ColorDepth, ColorFormat, FramebufferInfo, FramebufferResult};
use crate::consts::{KERNEL_OFFSET};
use core::convert::TryInto;
use self::vc4::v3dReg::*;

pub const BOARD_NAME: &'static str = "Raspberry Pi 3";
pub const PERIPHERALS_START: usize = bcm2837::addr::PERIPHERALS_START;
pub const PERIPHERALS_END: usize = bcm2837::addr::PERIPHERALS_END;
pub const CPU_NUM: usize = 4;

/// BCM2837 spin table (ref: linux/arch/arm/boot/dts/bcm2837.dtsi)
#[no_mangle]
pub static CPU_SPIN_TABLE: [usize; CPU_NUM] = [0xd8, 0xe0, 0xe8, 0xf0];

/// Initialize serial port before other initializations.
pub fn init_serial_early() {
    serial::init();
}

/// Initialize raspi3 drivers
pub fn init_driver() {
    if let Ok(fb_info) = probe_fb_info(0, 0, 0) {
        fb::init(fb_info);
    }
    vc4::init();
    emmc::init();
}

/// Returns the (start address, end address) of the physical memory on this
/// system if it can be determined. If it cannot, `None` is returned.
///
/// This function is expected to return `Some` under all normal cirumstances.
pub fn probe_memory() -> Option<(usize, usize)> {
    let mut atags: Atags = Atags::get();
    while let Some(atag) = atags.next() {
        if let Some(mem) = atag.mem() {
            return Some((mem.start as usize, (mem.start + mem.size) as usize));
        }
    }
    None
}

fn probe_fb_info(width: u32, height: u32, depth: u32) -> FramebufferResult {
    let (width, height) = if width == 0 || height == 0 {
        mailbox::framebuffer_get_physical_size()?
    } else {
        (width, height)
    };

    let depth = if depth == 0 {
        mailbox::framebuffer_get_depth()?
    } else {
        depth
    };

    println!("get framebuffer size from mailbox, height: {}, width: {}, depth: {}", height, width, depth);

    let info = mailbox::framebuffer_alloc(width, height, depth)?;

    if info.bus_addr == 0 || info.screen_size == 0 {
        Err(format!("mailbox call returned an invalid address/size"))?;
    }
    if info.pitch == 0 || info.pitch != info.xres * info.depth / 8 {
        Err(format!(
            "mailbox call returned an invalid pitch value {}",
            info.pitch
        ))?;
    }

    println!("set framebuffer with xres: {}, yres: {}, pitch: {}, addr: {:#x}, screen_size: {}", info.xres, info.yres, info.pitch, info.bus_addr, info.screen_size);

    let paddr = bus_to_phys(info.bus_addr);
    let vaddr = super::memory::ioremap(paddr as usize, info.screen_size as usize, "fb");
    println!("paddr: {:#x}, vaddr: {:#x}", paddr, vaddr);
    if vaddr == 0 {
        Err(format!(
            "cannot remap memory range [{:#x?}..{:#x?}]",
            paddr,
            paddr + info.screen_size
        ))?;
    }

    let depth = ColorDepth::try_from(info.depth)?;
    let format = match info.depth {
        16 => ColorFormat::RGB565,
        32 => ColorFormat::BGRA8888,
        _ => Err(format!("unsupported color depth {}", info.depth))?,
    };
    Ok(FramebufferInfo {
        xres: info.xres,
        yres: info.yres,
        xres_virtual: info.xres_virtual,
        yres_virtual: info.yres_virtual,
        xoffset: info.xoffset,
        yoffset: info.yoffset,
        depth: depth,
        format: format,
        paddr: paddr as usize,
        vaddr: vaddr,
        screen_size: info.screen_size as usize,
    })
}


//hackdrivers

pub fn test_gpu() {
    if (mailbox::gpu_enable().is_ok()) {
        println!("enable success");
    }

    let bus_addr: usize = 0x3fc00000;
    let paddr: usize = 0x3fc00000;
    let vaddr = paddr.wrapping_add(KERNEL_OFFSET);
    println!("vaddr: {:#x}", vaddr);
    let accessible = vaddr;
    let mut ans = 0;
    unsafe {
        ans = *(accessible as *mut u32);
    }
    println!("status: {}", ans);
    if (ans != 0x02443356) { // Magic number.
        println!("Error: V3D pipeline isn't powered up and accessable.\n");
        return;
    } else {
        println!("Success: V3D pipline has powered up\n");
    }
    unsafe {
        test_triangle(vaddr);
    }
}

pub unsafe fn addbyte(p: &mut usize, d: u8) {
    *(*p as *mut u8) = d;
    *p = *p + 1;
}

pub unsafe fn addword(p: &mut usize, d: u32) {
    *(*p as *mut u8) = (d & 0xff).try_into().unwrap();
    *p = *p + 1;
    *(*p as *mut u8) = ((d >> 8) & 0xff).try_into().unwrap(); 
    *p = *p + 1;
    *(*p as *mut u8) = ((d >> 16) & 0xff).try_into().unwrap(); 
    *p = *p + 1;
    *(*p as *mut u8) = ((d >> 24) & 0xff).try_into().unwrap(); 
    *p = *p + 1;
}

pub unsafe fn addshort(p: &mut usize, d: u16) {
    *(*p as *mut u8) = (d & 0xff).try_into().unwrap();;
    *p = *p + 1;
    *(*p as *mut u8) = ((d >> 8) & 0xff).try_into().unwrap(); 
    *p = *p + 1;
}

pub unsafe fn addfloat(p: &mut usize, f: f32) {
    let fp = &f as *const f32;
    let dp = fp as *const u32;
    addword(p, *dp);
}

pub unsafe fn setV3D(v3d: usize, offset: usize, data: u32) {
    let tmp = v3d + offset * 4;
    *(tmp as *mut u32) = data; 
}

pub unsafe fn getV3D(v3d: usize, offset: usize) -> u32 {
    let tmp = v3d + offset * 4;
    *(tmp as *mut u32)
}

pub unsafe fn test_triangle(v3d: usize) {
    let handle = mailbox::mem_alloc(0x800000, 0x1000, mailbox::MEM_FLAG_COHERENT | mailbox::MEM_FLAG_ZERO).unwrap();
    println!("Successfully allocate memory, handle: {}", handle);

    let bus_addr = mailbox::mem_lock(handle).unwrap();
    println!("Bus address: {:#x}", bus_addr);
    let paddr = bus_to_phys(bus_addr);
    let vaddr = super::memory::ioremap(paddr as usize, 0x800000, "triangle");
    
    let mut p = vaddr;
    let mut list = vaddr;

    addbyte(&mut p, 112);
    addword(&mut p, bus_addr + 0x6200); // tile allocation memory address
    addword(&mut p, 0x8000); // tile allocation memory size
    addword(&mut p, bus_addr + 0x100); // Tile state data address
    addbyte(&mut p, 30); // 1920/64
    addbyte(&mut p, 17); // 1080/64 (16.875)
    addbyte(&mut p, 0x04); // config

    // Start tile binning.
    addbyte(&mut p, 6);

    // Primitive type
    addbyte(&mut p, 56);
    addbyte(&mut p, 0x32); // 16 bit triangle

    // Clip Window
    addbyte(&mut p, 102);
    addshort(&mut p, 0);
    addshort(&mut p, 0);
    addshort(&mut p, 1920); // width
    addshort(&mut p, 1080); // height

    // State
    addbyte(&mut p, 96);
    addbyte(&mut p, 0x03); // enable both foward and back facing polygons
    addbyte(&mut p, 0x00); // depth testing disabled
    addbyte(&mut p, 0x02); // enable early depth write

    // Viewport offset
    addbyte(&mut p, 103);
    addshort(&mut p, 0);
    addshort(&mut p, 0);

    // The triangle
    // No Vertex Shader state (takes pre-transformed vertexes, 
    // so we don't have to supply a working coordinate shader to test the binner.
    addbyte(&mut p, 65);
    addword(&mut p, bus_addr + 0x80); // Shader Record

    // primitive index list
    addbyte(&mut p, 32);
    addbyte(&mut p, 0x04); // 8bit index, trinagles
    addword(&mut p, 3); // Length
    addword(&mut p, bus_addr + 0x70); // address
    addword(&mut p, 2); // Maximum index

    // End of bin list
    // Flush
    addbyte(&mut p, 5);
    // Nop
    addbyte(&mut p, 1);
    // Halt
    addbyte(&mut p, 0);

    let length = p - list;
    //assert(length < 0x80);

    // Shader Record
    p = list + 0x80;
    addbyte(&mut p, 0x01); // flags
    addbyte(&mut p, 6*4); // stride
    addbyte(&mut p, 0xcc); // num uniforms (not used)
    addbyte(&mut p, 3); // num varyings
    addword(&mut p, bus_addr + 0xfe00); // Fragment shader code
    addword(&mut p, bus_addr + 0xff00); // Fragment shader uniforms
    addword(&mut p, bus_addr + 0xa0); // Vertex Data

    // Vertex Data
    p = list + 0xa0;
    // Vertex: Top, red
    addshort(&mut p, (1920/2) << 4); // X in 12.4 fixed point
    addshort(&mut p, 200 << 4); // Y in 12.4 fixed point
    addfloat(&mut p, 1.0); // Z
    addfloat(&mut p, 1.0); // 1/W
    addfloat(&mut p, 1.0); // Varying 0 (Red)
    addfloat(&mut p, 0.0); // Varying 1 (Green)
    addfloat(&mut p, 0.0); // Varying 2 (Blue)

    // Vertex: bottom left, Green
    addshort(&mut p, 560 << 4); // X in 12.4 fixed point
    addshort(&mut p, 800 << 4); // Y in 12.4 fixed point
    addfloat(&mut p, 1.0); // Z
    addfloat(&mut p, 1.0); // 1/W
    addfloat(&mut p, 0.0); // Varying 0 (Red)
    addfloat(&mut p, 1.0); // Varying 1 (Green)
    addfloat(&mut p, 0.0); // Varying 2 (Blue)

    // Vertex: bottom right, Blue
    addshort(&mut p, 1360 << 4); // X in 12.4 fixed point
    addshort(&mut p, 800 << 4); // Y in 12.4 fixed point
    addfloat(&mut p, 1.0); // Z
    addfloat(&mut p, 1.0); // 1/W
    addfloat(&mut p, 0.0); // Varying 0 (Red)
    addfloat(&mut p, 0.0); // Varying 1 (Green)
    addfloat(&mut p, 1.0); // Varying 2 (Blue)

    // Vertex list
    p = list + 0x70;
    addbyte(&mut p, 0); // top
    addbyte(&mut p, 1); // bottom left
    addbyte(&mut p, 2); // bottom right

    // fragment shader
    p = list + 0xfe00;
    addword(&mut p, 0x958e0dbf);
    addword(&mut p, 0xd1724823); /* mov r0, vary; mov r3.8d, 1.0 */
    addword(&mut p, 0x818e7176); 
    addword(&mut p, 0x40024821); /* fadd r0, r0, r5; mov r1, vary */
    addword(&mut p, 0x818e7376); 
    addword(&mut p, 0x10024862); /* fadd r1, r1, r5; mov r2, vary */
    addword(&mut p, 0x819e7540); 
    addword(&mut p, 0x114248a3); /* fadd r2, r2, r5; mov r3.8a, r0 */
    addword(&mut p, 0x809e7009); 
    addword(&mut p, 0x115049e3); /* nop; mov r3.8b, r1 */
    addword(&mut p, 0x809e7012); 
    addword(&mut p, 0x116049e3); /* nop; mov r3.8c, r2 */
    addword(&mut p, 0x159e76c0); 
    addword(&mut p, 0x30020ba7); /* mov tlbc, r3; nop; thrend */
    addword(&mut p, 0x009e7000);
    addword(&mut p, 0x100009e7); /* nop; nop; nop */
    addword(&mut p, 0x009e7000);
    addword(&mut p, 0x500009e7); /* nop; nop; sbdone */

    // Render control list
    p = list + 0xe200;

    // Clear color
    addbyte(&mut p, 114);
    addword(&mut p, 0xff000000); // Opaque Black
    addword(&mut p, 0xff000000); // 32 bit clear colours need to be repeated twice
    addword(&mut p, 0);
    addbyte(&mut p, 0);

    // Tile Rendering Mode Configuration
    addbyte(&mut p, 113);
    addword(&mut p, bus_addr + 0x10000); // framebuffer addresss
    addshort(&mut p, 1920); // width
    addshort(&mut p, 1080); // height
    addbyte(&mut p, 0x04); // framebuffer mode (linear rgba8888)
    addbyte(&mut p, 0x00);

    // Do a store of the first tile to force the tile buffer to be cleared
    // Tile Coordinates
    addbyte(&mut p, 115);
    addbyte(&mut p, 0);
    addbyte(&mut p, 0);
    // Store Tile Buffer General
    addbyte(&mut p, 28);
    addshort(&mut p, 0); // Store nothing (just clear)
    addword(&mut p, 0); // no address is needed

    // Link all binned lists together
    for x in 0..30 {
        for y in 0..17 {

            // Tile Coordinates
            addbyte(&mut p, 115);
            addbyte(&mut p, x);
            addbyte(&mut p, y);
            
            // Call Tile sublist
            addbyte(&mut p, 17);
            addword(&mut p, bus_addr + 0x6200 + (y as u32 * 30 + x as u32) * 32);

            // Last tile needs a special store instruction
            if(x == 29 && y == 16) {
            // Store resolved tile color buffer and signal end of frame
            addbyte(&mut p, 25);
            } else {
            // Store resolved tile color buffer
            addbyte(&mut p, 24);
            }
        }
    }

    let render_length = p - (list + 0xe200);

    // Run our control list
    println!("Binner control list constructed");
    println!("Start Address: {:#x}, length: {:#x}", bus_addr, length);

    setV3D(v3d, V3D_CT0CA, bus_addr);
    setV3D(v3d, V3D_CT0EA, bus_addr + length as u32);
    println!("V3D_CT0CS: {:#x}, Address: {:#x}", getV3D(v3d, V3D_CT0CS), getV3D(v3d, V3D_CT0CA));

    // v3d[V3D_CT0CA] = bus_addr;
    // v3d[V3D_CT0EA] = bus_addr + length;
    // printf("V3D_CT0CS: 0x%08x, Address: 0x%08x\n", v3d[V3D_CT0CS], v3d[V3D_CT0CA]);
    
    // Wait for control list to execute
    while (getV3D(v3d, V3D_CT0CS) & 0x20) > 0 {}
    //while(v3d[V3D_CT0CS] & 0x20);
  
    println!("V3D_CT0CS: {:#x}, Address: {:#x}", getV3D(v3d, V3D_CT0CS), getV3D(v3d, V3D_CT1CA));
    println!("V3D_CT1CS: {:#x}, Address: {:#x}", getV3D(v3d, V3D_CT1CS), getV3D(v3d, V3D_CT1CA));
    //printf("V3D_CT0CS: 0x%08x, Address: 0x%08x\n", v3d[V3D_CT0CS], v3d[V3D_CT0CA]);
    //printf("V3D_CT1CS: 0x%08x, Address: 0x%08x\n", v3d[V3D_CT1CS], v3d[V3D_CT1CA]);

    setV3D(v3d, V3D_CT1CA, bus_addr + 0xe200);
    setV3D(v3d, V3D_CT1EA, bus_addr + 0xe200 + render_length as u32);

    //v3d[V3D_CT1CA] = bus_addr + 0xe200;
    //v3d[V3D_CT1EA] = bus_addr + 0xe200 + render_length;

    while (getV3D(v3d, V3D_CT1CS) & 0x20) > 0 {}
    //while(v3d[V3D_CT1CS] & 0x20);
    

    println!("V3D_CT1CS`: {:#x}, address: {:#x}", getV3D(v3d, V3D_CT1CS), getV3D(v3d, V3D_CT1CA));
    setV3D(v3d, V3D_CT1CS, 0x20);
    //pintf("V3D_CT1CS: 0x%08x, Address: 0x%08x\n", v3d[V3D_CT1CS], v3d[V3D_CT1CA]);
    //v3d[V3D_CT1CS] = 0x20;

    // // just dump the frame to a file
    // FILE *f = fopen("frame.data", "w");
    // fwrite(list + 0x10000, (1920*1080*4), 1, f);
    // fclose(f);
    // printf("frame buffer memory dumpped to frame.data\n");
    
    let begin = list + 0x10000;
    let mut lock = fb::FRAME_BUFFER.lock();
    if let Some(fbr) = lock.as_mut() {
        for x in 0..fbr.fb_info.xres {
            for y in 0..fbr.fb_info.yres {
                let mut color: u32 = 0;
                let pos = begin + 4 * x as usize + 1920 * 4 * y as usize;
                color = *(pos as *mut u32);
                fbr.write(x, y, color);
            }
        }
    }

    // Release resources
    // super::memory::
    // unmapmem((void *) list, 0x800000);
    // mem_unlock(mbox, handle);
    // mem_free(mbox, handle);
    println!("gpu calculate over!");
}