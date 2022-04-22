pub fn find<C: AsRef<[T]>, T: Ord>(array: C, key: T) -> Option<usize> {
    let slice = array.as_ref();
    let mut lo = 0;
    let mut hi = slice.len() as i32 - 1;
    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if key < slice[mid as usize] {
            hi = mid - 1;
        } else if key > slice[mid as usize] {
            lo = mid + 1;
        } else {
            return Some(mid as usize);
        }
    }
    None
}
