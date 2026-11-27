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

pub fn mean(xs: &[f64]) -> Option<f64> {
    let sum = xs.iter().copied().reduce(|acc, x| acc + x)?;
    let result = sum / xs.len() as f64;
    Some(result)
}

type DebugPlatformReadEntireFile = fn(&str) -> Option<(core::ptr::NonNull<()>, usize)>;
type DebugPlatformFreeFileMemory = fn(core::ptr::NonNull<()>, usize);
pub struct PlatformMemory {
    pub numbers_arena: MemoryArena,

    pub read_entire_file_stub: Option<DebugPlatformReadEntireFile>,
    pub free_file_memory_stub: Option<DebugPlatformFreeFileMemory>,
}

impl PlatformMemory {
    fn read_entire_file(&self, filename: &str) -> Option<(core::ptr::NonNull<()>, usize)> {
        if let Some(func) = self.read_entire_file_stub {
            func(filename)
        } else {
            None
        }
    }
    fn free_file_memory(&self, memory: core::ptr::NonNull<()>, size: usize) {
        if let Some(func) = self.free_file_memory_stub {
            func(memory, size)
        }
    }
}

pub struct FileNumbers {
    count: usize,
    numbers: core::ptr::NonNull<f64>,
}

impl FileNumbers {
    pub fn sum(&self) -> f64 {
        let xs = unsafe { core::slice::from_raw_parts(self.numbers.as_ptr(), self.count) };
        let result = xs.iter().copied().reduce(|acc, x| acc + x).unwrap();
        result
    }
}

pub fn debug_load_numbers(memory: &mut PlatformMemory, filename: &str) -> Option<FileNumbers> {
    let mut result = None;

    if let Some((content, file_size)) = memory.read_entire_file(filename) {
        let data = unsafe { core::slice::from_raw_parts(content.as_ptr() as *mut u8, file_size) };

        let mut file_numbers = FileNumbers {
            count: 0,
            numbers: core::ptr::NonNull::dangling(),
        };

        let mut sign = 1.;
        let mut value = 0.;
        let mut divisor = 10.;
        let mut in_fraction = false;
        let mut has_number = false;
        for (ch_index, &ch) in data.iter().enumerate() {
            if ch == b'-' {
                debug_assert!(ch_index == 0 || !data[ch_index - 1].is_ascii_digit());
                sign = -1.;
            } else if ch.is_ascii_digit() {
                let digit = (ch - b'0') as f64;
                if in_fraction {
                    value = value + digit / divisor;
                    divisor *= 10.;
                } else {
                    value = value * 10. + digit;
                }
                has_number = true;
            } else if ch == b'.' {
                in_fraction = true;
            } else if has_number {
                // TODO: numbers must point at count adjacent f64 values
                unsafe {
                    let number = push_struct!(&mut memory.numbers_arena, f64);
                    *number = value * sign;
                    if file_numbers.count == 0 {
                        file_numbers.numbers = core::ptr::NonNull::new(number).unwrap();
                    }
                };
                file_numbers.count += 1;

                sign = 1.;
                value = 0.;
                divisor = 10.;
                in_fraction = false;
                has_number = false;
            }
        }
        if has_number {
            unsafe {
                let number = push_struct!(&mut memory.numbers_arena, f64);
                *number = value * sign;
                if file_numbers.count == 0 {
                    file_numbers.numbers = core::ptr::NonNull::new(number).unwrap();
                }
            };
            file_numbers.count += 1;
        }

        memory.free_file_memory(content, file_size);

        if file_numbers.count > 0 {
            result = Some(file_numbers);
        }
    }

    result
}
