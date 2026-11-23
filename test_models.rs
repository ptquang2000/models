use *;

#[test]
fn test_mat() {
    let permanent_storage_size = megabytes!(1);
    let permanent_storage_memory = unsafe {
        mmap(
            core::ptr::null_mut(),
            permanent_storage_size,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_POPULATE,
            -1,
            0,
        )
    };
    let mut arena = MemoryArena {
        base: permanent_storage_memory as _,
        size: permanent_storage_size,
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
