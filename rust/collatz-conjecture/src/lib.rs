pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut count: u64 = 0;
    loop {
        match n {
            0 => return None,
            1 => return Some(count),
            x if (x % 2) == 0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
        count += 1;
    }
}
