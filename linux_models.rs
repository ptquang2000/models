#![cfg_attr(not(test), no_main)]

#[macro_use]
#[path = "models.rs"]
mod models;
use models::*;

#[cfg(test)]
#[path = "test_models.rs"]
mod test_models;

const PROT_READ: core::ffi::c_int = 0x1;
const PROT_WRITE: core::ffi::c_int = 0x2;

const O_RDONLY: i32 = 0o00;
const O_RDWR: i32 = 0o02;
const O_CREAT: i32 = 0o0100;
const O_TRUNC: i32 = 0o01000;
const O_NONBLOCK: i32 = 0o04000;

const S_IRUSR: i64 = 0o0400;
const S_IWUSR: i64 = 0o0200;
const S_IRGRP: i64 = S_IRUSR >> 3;
const S_IROTH: i64 = S_IRGRP >> 3;

const SEEK_SET: i32 = 0;
const SEEK_END: i32 = 2;

const MAP_PRIVATE: core::ffi::c_int = 0x02;
const MAP_ANONYMOUS: core::ffi::c_int = 0x20;
const MAP_POPULATE: core::ffi::c_int = 0x008000;
const MAP_FAILED: *mut core::ffi::c_void = (-1isize) as *mut core::ffi::c_void;

fn debug_platform_read_entire_file(filename: &str) -> Option<(core::ptr::NonNull<()>, usize)> {
    debug_assert!(filename.ends_with('\0'));
    let mut result = None;
    unsafe {
        let fd = open(
            filename.as_ptr() as *const _,
            O_RDONLY,
            S_IRUSR | S_IRGRP | S_IROTH,
        );
        if fd != -1 {
            let size = lseek(fd, 0, SEEK_END);
            if size != -1 {
                let addr = mmap(
                    core::ptr::null_mut(),
                    size as usize,
                    PROT_READ | PROT_WRITE,
                    MAP_PRIVATE,
                    fd,
                    0,
                );
                if addr != MAP_FAILED {
                    result = Some((core::ptr::NonNull::new(addr as _).unwrap(), size as usize));
                }
            }
            close(fd);
        }
    }
    result
}

fn debug_platform_free_file_memory(memory: core::ptr::NonNull<()>, size: usize) {
    unsafe { munmap(memory.as_ptr() as _, size) };
}

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
    fn munmap(addr: *mut core::ffi::c_void, len: usize) -> core::ffi::c_int;

    fn open(path: *const core::ffi::c_char, flag: core::ffi::c_int, ...) -> core::ffi::c_int;
    fn close(fd: core::ffi::c_int) -> core::ffi::c_int;
    fn lseek(
        fd: core::ffi::c_int,
        offset: core::ffi::c_long,
        whence: core::ffi::c_int,
    ) -> core::ffi::c_long;
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main(
    argc: core::ffi::c_int,
    argv: *const *const core::ffi::c_char,
) -> core::ffi::c_int {
    0
}
