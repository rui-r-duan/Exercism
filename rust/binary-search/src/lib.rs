pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let n = array.len();
    if n == 0 {
        return None;
    }
    let mut lo = 0;
    let mut hi = n - 1;
    loop {
        if lo == hi {
            if array[lo] == key {
                return Some(lo);
            } else {
                return None;
            }
        } else if lo < hi {
            let mid = lo + (hi - lo) / 2;
            if array[mid] == key {
                return Some(mid);
            } else if array[mid] < key {
                lo = mid + 1;
            } else {
                let new_hi = mid as i32 - 1;
                if new_hi < 0 {
                    return None;
                }
                hi = new_hi as usize;
            }
        } else {
            return None;
        }
    }
}
