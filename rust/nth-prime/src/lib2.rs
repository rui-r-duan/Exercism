use rand::Rng;

pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| fast_prime(x, 5)).nth(n as usize).unwrap()
}

fn square(n: u32) -> u32 {
    n * n
}

/// power(base, exp) % m
/// CAUTION: may cause overflow
/// e.g.
/// ---- test_big_prime stdout ----
/// thread 'test_big_prime' panicked at 'attempt to multiply with overflow', src/lib.rs:8:5
pub fn expmod(base: u32, exp: u32, m: u32) -> u32 {
    if exp == 0 {
        1
    } else if exp % 2 == 0 {
        square(expmod(base, exp / 2, m)) % m
    } else {
        (base * expmod(base, exp - 1, m)) % m
    }
}

/// Test if power(a, n) % n ≣ a
pub fn fermat_test(n: u32) -> bool {
    let try_it = |a| expmod(a, n, n) == a;

    let random = rand::thread_rng().gen_range(0..n - 1);

    try_it(random + 1)
}

pub fn fast_prime(n: u32, times: u32) -> bool {
    if times == 0 {
        true
    } else if fermat_test(n) {
        fast_prime(n, times - 1)
    } else {
        false
    }
}
