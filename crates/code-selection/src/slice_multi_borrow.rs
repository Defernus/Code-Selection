/// Allows for mutable borrowing of 2 elements of the slice at the same time.
pub fn get_pair_mut<T>(slice: &mut [T], index0: usize, index1: usize) -> (&mut T, &mut T) {
    assert_ne!(index0, index1, "Indices must be different");

    let ptr = slice.as_mut_ptr();

    unsafe {
        let a = &mut *ptr.add(index0);
        let b = &mut *ptr.add(index1);
        (a, b)
    }
}
