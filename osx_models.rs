#![cfg_attr(not(test), no_main)]

#[macro_use]
#[path = "models.rs"]
mod models;
use models::*;

const PROT_READ: core::ffi::c_int = 0x1;
const PROT_WRITE: core::ffi::c_int = 0x2;

const MAP_PRIVATE: core::ffi::c_int = 0x02;
const MAP_ANON: core::ffi::c_int = 0x1000;
const MAP_FAILED: *mut core::ffi::c_void = (-1isize) as *mut core::ffi::c_void;

#[link_name = "c"]
extern "C" {
    fn mmap(
        addr: *mut core::ffi::c_void,
        length: usize,
        prot: core::ffi::c_int,
        flags: core::ffi::c_int,
        fd: core::ffi::c_int,
        offset: core::ffi::c_long,
    ) -> *mut core::ffi::c_void;
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main(
    argc: core::ffi::c_int,
    argv: *const *const core::ffi::c_char,
) -> core::ffi::c_int {
    0
}
