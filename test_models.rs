use *;

#[test]
fn test_mat() {
    let mut buffer = [0.; 256];
    let mut arena = MemoryArena {
        base: buffer.as_ptr() as _,
        size: buffer.len() * core::mem::size_of_val(&buffer[0]),
        used: 0,
    };

    let mat = unsafe { Mat::new(&mut arena, 2, 3).as_mut() };
    mat.set(0, 0, 1.);
    mat.set(0, 1, 2.);
    mat.set(0, 2, 3.);
    mat.set(1, 0, 4.);
    mat.set(1, 1, 5.);
    mat.set(1, 2, 6.);
    assert_eq!(mat.get(0, 0), 1.);
    assert_eq!(mat.get(0, 1), 2.);
    assert_eq!(mat.get(0, 2), 3.);
    assert_eq!(mat.get(1, 0), 4.);
    assert_eq!(mat.get(1, 1), 5.);
    assert_eq!(mat.get(1, 2), 6.);
    assert_eq!(mat.shape(), (2, 3));
}

#[test]
fn test_mean() {
    assert_eq!(mean(&[1., 2., 3., 4., 5., 6.]).unwrap(), 3.5);
    assert_eq!(mean(&[-1., 1.]).unwrap(), 0.);
    assert_eq!(mean(&[7.]).unwrap(), 7.);
    assert_eq!(mean(&[]), None);
}

#[test]
fn test_load_numbers() {
    let mut buffer = [0.; 256];
    let mut numbers_arena = MemoryArena {
        base: buffer.as_ptr() as _,
        size: buffer.len() * core::mem::size_of_val(&buffer[0]),
        used: 0,
    };
    let mut memory = PlatformMemory {
        read_entire_file_stub: Some(debug_platform_read_entire_file),
        free_file_memory_stub: Some(debug_platform_free_file_memory),
        numbers_arena,
    };
    assert_eq!(
        debug_load_numbers(&mut memory, "./data/numbers_frac\0")
            .unwrap()
            .sum(),
        7.
    );
    assert_eq!(
        debug_load_numbers(&mut memory, "./data/numbers_int\0")
            .unwrap()
            .sum(),
        30.
    );
    assert_eq!(
        debug_load_numbers(&mut memory, "./data/numbers_neg\0")
            .unwrap()
            .sum(),
        -10.
    );
    assert_eq!(
        debug_load_numbers(&mut memory, "./data/numbers_notrail\0")
            .unwrap()
            .sum(),
        18.
    );
    assert_eq!(
        debug_load_numbers(&mut memory, "./data/numbers_empty\0").is_none(),
        true
    );
}
