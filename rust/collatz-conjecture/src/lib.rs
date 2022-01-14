pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n;
    let mut count: u64 = 0;
    if n < 1 {
        None
    } else if n == 1 {
        Some(0)
    } else {
        loop {
            if n % 2 == 0 {
                n /= 2;
                count += 1;
                if n == 1 {
                    return Some(count);
                }
            } else {
                match n.checked_mul(3) {
                    Some(x) => {
                        n = x;
                        match n.checked_add(1) {
                            Some(x) => {
                                n = x;
                                count += 1;
                                if n == 1 {
                                    return Some(count);
                                }
                            }
                            None => {
                                return None;
                            }
                        }
                    }
                    None => {
                        return None;
                    }
                }
            }
        }
    }
}
