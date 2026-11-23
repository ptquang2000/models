#![cfg_attr(not(test), no_main)]

macro_rules! kilobytes {
    ($value:expr) => {
        $value * 1024
    };
}
macro_rules! megabytes {
    ($value:expr) => {
        kilobytes!($value) * 1024
    };
}
macro_rules! gigabytes {
    ($value:expr) => {
        megabytes!($value) * 1024
    };
}
macro_rules! terabytes {
    ($value:expr) => {
        gigabytes!($value) * 1024
    };
}

type MemoryIndex = usize;

struct MemoryArena {
    size: MemoryIndex,
    base: *mut u8,
    used: MemoryIndex,
}

macro_rules! push_struct {
    ($arena:expr, $type:ty) => {
        push_size_(
            $arena,
            core::mem::size_of::<$type>(),
            core::mem::align_of::<$type>(),
        )
        .cast::<$type>()
    };
}
macro_rules! push_array {
    ($arena:expr, $count:expr, $type:ty) => {
        push_size_(
            $arena,
            $count as usize * core::mem::size_of::<$type>(),
            core::mem::align_of::<$type>(),
        )
        .cast::<$type>()
    };
}
fn push_size_(arena: &mut MemoryArena, size: MemoryIndex, align: MemoryIndex) -> *mut u8 {
    arena.used = (arena.used + align - 1) & !(align - 1);
    debug_assert!(arena.used + size <= arena.size);

    let result = arena.base.wrapping_add(arena.used);
    arena.used += size;
    result
}

const PROT_READ: core::ffi::c_int = 0x1;
const PROT_WRITE: core::ffi::c_int = 0x2;

const MAP_PRIVATE: core::ffi::c_int = 0x02;
const MAP_ANONYMOUS: core::ffi::c_int = 0x20;
const MAP_POPULATE: core::ffi::c_int = 0x008000;
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
struct Mat {
    rows: usize,
    cols: usize,
    data: core::ptr::NonNull<f64>,
}

impl Mat {
    fn new(arena: &mut MemoryArena, rows: usize, cols: usize) -> core::ptr::NonNull<Mat> {
        let mat = core::ptr::NonNull::new(push_struct!(arena, Mat)).unwrap();
        unsafe {
            mat.write(Mat {
                rows: rows,
                cols: cols,
                data: core::ptr::NonNull::new(push_array!(arena, rows * cols, f64)).unwrap(),
            })
        };
        mat
    }

    fn get(&self, i: usize, j: usize) -> f64 {
        debug_assert!(i < self.rows && j < self.cols);
        unsafe { self.data.add(i * self.cols + j).read() }
    }

    fn set(&self, i: usize, j: usize, v: f64) {
        debug_assert!(i < self.rows && j < self.cols);
        unsafe { self.data.add(i * self.cols + j).write(v) };
    }

    fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main(
    argc: core::ffi::c_int,
    argv: *const *const core::ffi::c_char,
) -> core::ffi::c_int {
    0
}

#[cfg(test)]
#[path = "test_models.rs"]
mod test_models;
