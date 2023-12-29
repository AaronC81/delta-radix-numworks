//! Provides a [GlobalAlloc] allocator which directly uses the underlying C `malloc`.

use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// An allocator which uses `malloc` and `free` from the linked C libraries.
pub struct Mallocator;

unsafe impl GlobalAlloc for Mallocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        free(ptr)
    }
}

// Required to link against `malloc` and `free`.
#[used] #[no_mangle] static __exidx_start: u32 = 0;
#[used] #[no_mangle] static __exidx_end: u32 = 0;

#[no_mangle] extern "C" fn _exit() { panic!("`exit` called"); }
#[no_mangle] extern "C" fn _kill() { panic!("`kill` called"); }
#[no_mangle] extern "C" fn _getpid() -> usize { 1 }
