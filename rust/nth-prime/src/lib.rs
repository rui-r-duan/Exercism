pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}

fn smallest_divisor(n: u32) -> u32 {
    find_divisor(n, 2)
}

fn find_divisor(n: u32, test_divisor: u32) -> u32 {
    let mut d = test_divisor;

    while d * d <= n {
        if n % d == 0 {
            return d;
        } else {
            d += 1;
        }
    }

    n
}

fn is_prime(n: u32) -> bool {
    smallest_divisor(n) == n
}
