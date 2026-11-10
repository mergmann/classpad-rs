use core::sync::atomic::{AtomicUsize, Ordering};

use crate::sdk::SYMS;

const HEAP_MAX: usize = 0x8CE80000;

static RESERVED: AtomicUsize = AtomicUsize::new(0);

fn reset_heap() -> ! {
    unsafe { (SYMS.Mem_SbrkMode)(0) };
    unsafe { (SYMS.Mem_Sbrk)(0x10000000) };
    loop {}
}

/// Reserve the end of the heap as our static image base.
/// (Port of heap_hack.c)
unsafe fn reserve_heap(base: usize) {
    use core::sync::atomic::Ordering;

    if base >= HEAP_MAX {
        return;
    }

    // sbrk returns the current end
    let end = unsafe { (SYMS.Mem_Sbrk)(0) };

    // Check if we have enough space at the end of the heap to allocate our region
    if end.addr() > base {
        reset_heap();
    }

    // Consume the full heap space, prevent sbrk from growing
    let granted = unsafe { (SYMS.Mem_Sbrk)((HEAP_MAX - end.addr()) as i32) };
    if granted.addr() == usize::MAX {
        reset_heap();
    }

    RESERVED.swap(HEAP_MAX - base, Ordering::Relaxed);

    // Size of the free block between the current heap end and our fixed region
    let free = base - granted.addr();

    // Only donate if the block is large enough to hold metadata
    if free >= 8 {
        let header = granted.cast::<usize>();
        unsafe { header.write(free - 4) };
        unsafe { (SYMS.Mem_Free)(header.add(1).cast()) }
    }
}

unsafe fn release_heap() {
    let size = RESERVED.swap(0, Ordering::Relaxed);
    if size != 0 {
        unsafe { (SYMS.Mem_Sbrk)(-(size as i32)) };
    }
}

/// This needs to run after the symbol table was loaded,
/// *before* the SDK mallocs a buffer
#[used]
#[unsafe(link_section = ".init_array.00002")]
static BIG_APP_INIT: unsafe extern "C" fn() = big_app_init;

#[cfg(feature = "big-app")]
unsafe extern "C" fn big_app_init() {
    unsafe { reserve_heap(0x8cc80000) };
}

#[cfg(feature = "big-app")]
core::arch::global_asm!(
    ".globl _sh4_rust_code_size",
    ".set _sh4_rust_code_size, 0x200000"
);

#[cfg(feature = "big-app-1MB")]
unsafe extern "C" fn big_app_init() {
    unsafe { reserve_heap(0x8CD80000) };
}

#[cfg(feature = "big-app-1MB")]
core::arch::global_asm!(
    ".globl _sh4_rust_code_size",
    ".set _sh4_rust_code_size, 0x100000"
);

#[cfg(feature = "big-app-4MB")]
unsafe extern "C" fn big_app_init() {
    unsafe { reserve_heap(0x8CA80000) };
}

#[cfg(feature = "big-app-4MB")]
core::arch::global_asm!(
    ".globl _sh4_rust_code_size",
    ".set _sh4_rust_code_size, 0x400000"
);

#[used]
#[unsafe(link_section = ".fini_array.00001")]
static BIG_APP_FINI: unsafe extern "C" fn() = big_app_fini;

unsafe extern "C" fn big_app_fini() {
    unsafe { release_heap() };
}
