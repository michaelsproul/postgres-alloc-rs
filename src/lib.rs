#![feature(allocator)]

#![allocator]
#![no_std]

extern crate libc;

use libc::{size_t, c_void};

// Postgres functions for memory allocation.
extern {
    fn palloc(size: size_t) -> *mut c_void;
    fn repalloc(p: *mut c_void, size: size_t) -> *mut c_void;
    fn pfree(p: *mut c_void);
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, _align: usize) -> *mut u8 {
    unsafe { palloc(size as libc::size_t) as *mut u8 }
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, _old_size: usize, _align: usize) {
    unsafe { pfree(ptr as *mut libc::c_void) }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                _align: usize) -> *mut u8 {
    unsafe {
        repalloc(ptr as *mut libc::c_void, size as libc::size_t) as *mut u8
    }
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
