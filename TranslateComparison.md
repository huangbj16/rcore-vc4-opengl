## C2Rust Tranlation VS our Translation

C2Rust翻译流程

```
intercept-build make
跟踪make过程，生成compile_commands.json
​```
{
        "arguments": [
            "cc", 
            "-c", 
            "-std=c99", 
            "-o", 
            "test", 
            "test.c"
        ], 
        "directory": "/home/huangbj16/Documents/ostrain/buffer", 
        "file": "test.c"
    }, 
​```
c2rust transpile compile_commands.json
根据json完成翻译。
```



可以借鉴的部分

- 翻译的完整性，可直接运行的rust代码。
- 函数翻译满足FFI要求，
  - `#[no_mangle]
    pub extern "C" fn rust_function() {}`

- include".h"引用头文件的处理

  ```
  头文件中的函数声明，变为extern C
  original
      #include "vc4_cl.h"
      vc4_cl.h:
          void vc4_init_cl(struct vc4_cl *cl);
  C2Rust Translation
  	vc4_render_cl.rs
      extern "C" {
          #[no_mangle]
          fn vc4_bo_create(dev: *mut device, size: size_t,
                           type_0: vc4_kernel_bo_type) -> *mut vc4_bo;
      }
  ```



存在的问题

```
大量unsafe函数
大量的raw pointers使用和dereference使用。
(*bo).size = size;
(*bo).handle = handle;
(*bo).paddr = bus_addr;
(*bo).vaddr = bus_addr as *mut libc::c_void;
(*bo).type_0 = type_0;
```





```
头文件中的函数实现和Struct声明，直接复制粘贴，代码冗余。
original
	#include "vc4_cl.h"
    static inline void cl_u32(struct vc4_cl *cl, uint32_t n);
C2Rust translation
	vc4_render_cl.rs
	#[inline]
    unsafe extern "C" fn cl_u32(mut cl: *mut vc4_cl, mut n: uint32_t) {
        put_unaligned_32((*cl).next, n);
        cl_advance(cl, 4i32 as uint32_t);
    }
```







#### Pros

```
对goto语句做了处理。
逻辑上没有问题。

基于makefile，对文件之间的include关系非常清楚。

函数的翻译：参考官方描述：https://rust-embedded.github.io/book/interoperability/rust-with-c.html#no_mangle
保证满足FFI的要求。
```

#### Cons

```
语法级而非语义级的翻译，安全性完全无法保证。

函数整体unsafe
大量的raw pointers使用和dereference使用。
(*bo).size = size;
(*bo).handle = handle;
(*bo).paddr = bus_addr;
(*bo).vaddr = bus_addr as *mut libc::c_void;
(*bo).type_0 = type_0;
```

#### Details

```rust
include".h"引用头文件的处理：

头文件中的函数声明，变为extern C
original
    #include "vc4_cl.h"
    vc4_cl.h:
        void vc4_init_cl(struct vc4_cl *cl);
C2Rust Translation
	vc4_render_cl.rs
    extern "C" {
        #[no_mangle]
        fn vc4_bo_create(dev: *mut device, size: size_t,
                         type_0: vc4_kernel_bo_type) -> *mut vc4_bo;
    }

头文件中的函数实现和Struct声明，直接复制粘贴
original
	#include "vc4_cl.h"
    static inline void cl_u32(struct vc4_cl *cl, uint32_t n)
    {
        put_unaligned_32(cl->next, n);
        cl_advance(cl, 4);
    }
C2Rust translation
	vc4_render_cl.rs
	#[inline]
    unsafe extern "C" fn cl_u32(mut cl: *mut vc4_cl, mut n: uint32_t) {
        put_unaligned_32((*cl).next, n);
        cl_advance(cl, 4i32 as uint32_t);
    }
Our Translation
	vc4_render_cl.rs
	impl vc4_cl {//改写成OOP风格，作为vc4_cl的成员函数，不需要传*cl的参数。
        pub fn cl_u32(&mut self, data: u32) {
		unsafe {
			*(self.next as *mut u32) = data;
		}
		self.next += 4;
	}
```

**Function Translation**

```rust
//C2Rust Translation
#[no_mangle]
pub unsafe extern "C" fn vc4_bo_create(mut dev: *mut device, mut size: size_t,
                                       mut type_0: vc4_kernel_bo_type)
 -> *mut vc4_bo {
    let mut bus_addr: uint32_t = 0;
    let mut vc4: *mut vc4_dev = to_vc4_dev(dev);
    let mut bo: *mut vc4_bo = 0 as *mut vc4_bo;
    if type_0 as libc::c_uint == VC4_BO_TYPE_FB as libc::c_int as libc::c_uint
       {
        return (*vc4).fb_bo
    }
    // bo = (struct vc4_bo *)kmalloc(sizeof(struct vc4_bo));
	// if (!bo)
	// 	return NULL;
    if size == 0 { return 0 as *mut vc4_bo }
    size =
        ({
             let mut __n: size_t = 4096i32 as size_t;
             ({
                  let mut __a: size_t =
                      size.wrapping_add(__n).wrapping_sub(1i32 as
                                                              libc::c_ulonglong);
                  __a.wrapping_sub(__a.wrapping_rem(__n))
              })
         });
    let mut handle: uint32_t =
        mbox_mem_alloc(size, 4096i32 as size_t,
                       (MEM_FLAG_COHERENT as libc::c_int |
                            MEM_FLAG_ZERO as libc::c_int) as uint32_t);
    if handle == 0 {
        kprintf(b"VC4: unable to allocate memory with size %08x\n\x00" as
                    *const u8 as *const libc::c_char, size);
        return 0 as *mut vc4_bo
    }
    if handle as libc::c_ulong >=
           ((2i32 * 4096i32) as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<vc4_dev>()
                                                as
                                                libc::c_ulong).wrapping_div(::std::mem::size_of::<vc4_bo>()
                                                                                as
                                                                                libc::c_ulong)
       {
        kprintf(b"VC4: too many bo handles, VC4_DEV_BO_NENTRY = %d\n\x00" as
                    *const u8 as *const libc::c_char,
                ((2i32 * 4096i32) as
                     libc::c_ulong).wrapping_sub(::std::mem::size_of::<vc4_dev>()
                                                     as
                                                     libc::c_ulong).wrapping_div(::std::mem::size_of::<vc4_bo>()
                                                                                     as
                                                                                     libc::c_ulong));
    } else {
        bus_addr = mbox_mem_lock(handle);
        if bus_addr == 0 {
            kprintf(b"VC4: unable to lock memory at handle %08x\n\x00" as
                        *const u8 as *const libc::c_char, handle);
        } else {
            __boot_map_iomem(bus_addr, size, bus_addr);
            bo =
                &mut *(*vc4).handle_bo_map.offset(handle as isize) as
                    *mut vc4_bo;
            (*bo).size = size;
            (*bo).handle = handle;
            (*bo).paddr = bus_addr;
            (*bo).vaddr = bus_addr as *mut libc::c_void;
            (*bo).type_0 = type_0;
            list_init(&mut (*bo).unref_head);
            // kprintf("vc4_bo_create: %08x %08x %08x %08x\n", bo->size, bo->handle,
	// 	bo->paddr, bo->vaddr);
            return bo
        }
    }
    mbox_mem_free(handle);
    return 0 as *mut vc4_bo;
}
```

```rust
//our translation:
pub fn vc4_bo_create(&mut self, size: u32, bo_type: u32) -> Option<Arc<Mutex<gpu_bo>>>
	{
		// default frame buffer
		if (bo_type == VC4_BO_TYPE_FB) {
			if let Some(bo) = &self.fb_bo {
				return Some(bo.clone())
			} else {
				return None
			}
		}

		if size == 0 {
			return None
		}

		let size = roundUp(size, PAGE_SIZE as u32);

		if let Ok(handle) = mailbox::mem_alloc(size, PAGE_SIZE as u32, mailbox::MEM_FLAG_COHERENT | mailbox::MEM_FLAG_ZERO) {
			// we use Tree, so don't have to check this?
			// if handle >= VC4_DEV_BO_NENTRY {
			// 	println!("VC4: too many bo handles, VC4_DEV_BO_NENTRY = {%d}\n",
			// 		VC4_DEV_BO_NENTRY);
			// 	// goto free_mem;
			// 	mailbox::mem_free(handle);
			// 	return None
			// }

			if let Ok(bus_addr) = mailbox::mem_lock(handle) {
				let paddr = addr::bus_to_phys(bus_addr);
				let vaddr = memory::ioremap(paddr as usize, size as usize, "bo");
				let result = self.handle_bo_map.insert(handle, Arc::new(Mutex::new(gpu_bo {
																	size: size,
																	handle: handle,
																	paddr: paddr,
																	vaddr: vaddr,
																	bo_type: bo_type	
																})));
				if let Some(bo) = self.handle_bo_map.get(&handle) {
					Some(bo.clone())
				} else {
					None
				}
			} else {
				println!("VC4: unable to lock memory at handle {}", handle);
				if let Err(r) = mailbox::mem_free(handle) {
					println!("VC4: unable to free memory at handle {}", handle);
				}
				None
			}
		} else {
			println!("VC4: unable to allocate memory with size {:#x}\n", size);
			None
		}
	}
```

**宏定义**

```rust
//未实现宏定义，直接代码覆盖：
//猜测原因：C的宏定义没有限定类型，在rust里无法定义。
Case1:
宏定义ROUNDUP
#define ROUNDUP(a, n) ({                                            \
            size_t __n = (size_t)(n);                               \
            (typeof(a))(ROUNDDOWN((size_t)(a) + __n - 1, __n));     \
        })
size = ROUNDUP(size, PGSIZE);
let mut __a: size_t =
                      size.wrapping_add(__n).wrapping_sub(1i32 as
                                                              libc::c_ulonglong);
                  __a.wrapping_sub(__a.wrapping_rem(__n))
---
Case2:
mbox_mem_alloc(size, PGSIZE, MEM_FLAG_COHERENT | MEM_FLAG_ZERO);
mbox_mem_alloc(size, 4096i32 as size_t,
                       (MEM_FLAG_COHERENT as libc::c_int |
                            MEM_FLAG_ZERO as libc::c_int) as uint32_t);
```

**Enum**

```rust
//对Enum的处理
original
enum vc4_kernel_bo_type {
	VC4_BO_TYPE_FB,
	VC4_BO_TYPE_V3D,
	VC4_BO_TYPE_BIN,
	VC4_BO_TYPE_RCL,
	VC4_BO_TYPE_BCL,
};
our translation
enum vc4_kernel_bo_type {
	VC4_BO_TYPE_FB,
	VC4_BO_TYPE_V3D,
	VC4_BO_TYPE_BIN,
	VC4_BO_TYPE_RCL,
	VC4_BO_TYPE_BCL,
}
C2Rust
pub type vc4_kernel_bo_type = libc::c_uint;
pub const VC4_BO_TYPE_BCL: vc4_kernel_bo_type = 4;
pub const VC4_BO_TYPE_RCL: vc4_kernel_bo_type = 3;
pub const VC4_BO_TYPE_BIN: vc4_kernel_bo_type = 2;
pub const VC4_BO_TYPE_V3D: vc4_kernel_bo_type = 1;
pub const VC4_BO_TYPE_FB: vc4_kernel_bo_type = 0;
```

**类型转换**

```rust
//类型转换
pub type bool_0 = libc::c_int;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type uintptr_t = uint64_t;
pub type size_t = uintptr_t;
pub type __u32 = uint32_t;
```

