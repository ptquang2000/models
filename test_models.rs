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
