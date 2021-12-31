pub fn factors(n: u64) -> Vec<u64> {
    let mut i = n;
    let mut factors = Vec::new();
    let mut candidates = 2..;

    while i > 1 {
        let x = candidates.next().unwrap();

        while i % x == 0 {
            i /= x;
            factors.push(x);
        }
    }

    factors
}
