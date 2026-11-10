use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

use hollyhock_sys::symbols::{free, malloc};

#[global_allocator]
static ALLOC: LibcAlloc = LibcAlloc;

const MALLOC_ALIGN: usize = 4;

struct LibcAlloc;

unsafe impl GlobalAlloc for LibcAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            if layout.align() <= MALLOC_ALIGN {
                return malloc(layout.size()) as *mut u8;
            }
            let raw = malloc(layout.size() + layout.align() + size_of::<usize>()) as *mut u8;
            if raw.is_null() {
                return raw;
            }
            let start = raw as usize + size_of::<usize>();
            let aligned = (start + layout.align() - 1) & !(layout.align() - 1);
            *((aligned - size_of::<usize>()) as *mut *mut u8) = raw;
            aligned as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            if layout.align() <= MALLOC_ALIGN {
                free(ptr as *mut c_void);
            } else {
                free(*((ptr as usize - size_of::<usize>()) as *mut *mut c_void));
            }
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        unsafe {
            if new_size <= layout.size() {
                return ptr;
            }
            let new_layout = Layout::from_size_align_unchecked(new_size, layout.align());
            let new = self.alloc(new_layout);
            if !new.is_null() {
                core::ptr::copy_nonoverlapping(ptr, new, layout.size());
                self.dealloc(ptr, layout);
            }
            new
        }
    }
}
