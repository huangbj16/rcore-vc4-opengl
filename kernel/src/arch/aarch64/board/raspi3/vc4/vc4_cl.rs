//#ifndef VC4_CL_H
//#define VC4_CL_H

struct vc4_bo;
struct vc4_context;

struct vc4_cl {
    base: usize;
    next: usize;
    reloc_next: usize;
    size: size_t;//what is size_t
}

//in C, parameter is a pointer vc4_cl* cl; in Rust, I use reference &mut vc4_cl to be safe.
pub fn vc4_init_cl(cl: &mut vc4_cl){
    cl.base = 0;
    cl.next = 0;
    cl.size = 0;
}
pub fn vc4_reset_cl(cl: &mut vc4_cl){
    cl.next = cl.base;
}
pub fn vc4_dump_cl(cl: &mut vc4_cl, size: &u32, cols: &u32, name: &str){
    println!("vc4_dump_cl {}:", name);

    let mut offset = 0;
    let mut ptr: u8 = cl as u8;

    while offset < size {
        print!("{08x}: ", ptr);
        let mut i = 0;
        while i < cols && offset < size {
            unsafe {
                print!("{02x} ", *ptr);
            }
            i += 1;
            ptr += 1;
            offset += 1;
        }
        println!();
    }
}

//struct __attribute__((__packed__)) unaligned_16 { uint16_t x; };
//struct __attribute__((__packed__)) unaligned_32 { uint32_t x; };

pub fn cl_offset(cl: &mut vc4_cl) -> u32
{
    (cl.next as u32) - (cl.base as u32)
}

pub fn cl_advance(cl: &mut vc4_cl, n: u32){
    cl.next += n;
}

pub fn put_unaligned_32(ptr: &mut u32, val: u32)
{
    ptr = val;
}

pub fn put_unaligned_16(ptr: &mut u16, val: u16)
{
    ptr = val;
}

//not needed any more?

// pub fn get_unaligned_32(ptr: usize) -> u32
// {
// struct unaligned_32 *p = (void *)ptr;
// return p->x;
// }

// pub fn get_unaligned_16(ptr: usize) -> u16
// {
// struct unaligned_16 *p = (void *)ptr;
// return p->x;
// }

pub fn cl_u8(cl: &mut vc4_cl, n: u8)
{
    unsafe{
        let *mut next = cl.next as u8;
        *next = n;
    }
    cl_advance(cl, 1);
}

//not very sure about how to convert uint8_t *?
// static inline void cl_u8(struct vc4_cl *cl, uint8_t n)
// {
// 	*(uint8_t *)cl->next = n;
// 	cl_advance(cl, 1);
// }

pub fn cl_u16(cl: *mut vc4_cl, n: u16)
{
put_unaligned_16(cl->next, n);
cl_advance(cl, 2);
}

pub fn cl_u32(cl: *mut vc4_cl, n: u32)
{
put_unaligned_32(cl->next, n);
cl_advance(cl, 4);
}

pub fn cl_aligned_u32(cl: *mut vc4_cl, n: u32)
{
*(uint32_t *)cl->next = n;
cl_advance(cl, 4);
}

pub fn cl_f(cl: *mut vc4_cl, f: f32)
{
cl_u32(cl, *((uint32_t *)&f));
}

pub fn cl_aligned_f(cl: *mut vc4_cl, f: f32)
{
cl_aligned_u32(cl, *((uint32_t *)&f));
}

//#endif /* VC4_CL_H */
