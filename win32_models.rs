#![cfg_attr(not(test), no_main)]

#[macro_use]
#[path = "models.rs"]
mod models;
use models::*;

type BOOL = core::ffi::c_int;
type CHAR = core::ffi::c_char;
type HANDLE = *mut core::ffi::c_void;
type DWORD = core::ffi::c_ulong;
type VOID = core::ffi::c_void;
type WORD = core::ffi::c_ushort;
type WCHAR = u16;
type UINT = core::ffi::c_uint;

const INVALID_HANDLE_VALUE: *mut VOID = -1isize as _;

#[link(name = "kernel32")]
extern "system" {
    fn VirtualAlloc(
        address: *mut VOID,
        size: usize,
        allocation_type: DWORD,
        protect: DWORD,
    ) -> *mut VOID;
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main(
    argc: core::ffi::c_int,
    argv: *const *const core::ffi::c_char,
) -> core::ffi::c_int {
    0
}
