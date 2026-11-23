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

pub struct MemoryArena {
    pub size: MemoryIndex,
    pub base: *mut u8,
    pub used: MemoryIndex,
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

pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    pub data: core::ptr::NonNull<f64>,
}

impl Mat {
    pub fn new(arena: &mut MemoryArena, rows: usize, cols: usize) -> core::ptr::NonNull<Mat> {
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

    pub fn get(&self, i: usize, j: usize) -> f64 {
        debug_assert!(i < self.rows && j < self.cols);
        unsafe { self.data.add(i * self.cols + j).read() }
    }

    pub fn set(&self, i: usize, j: usize, v: f64) {
        debug_assert!(i < self.rows && j < self.cols);
        unsafe { self.data.add(i * self.cols + j).write(v) };
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

#[cfg(test)]
#[path = "test_models.rs"]
mod test_models;
