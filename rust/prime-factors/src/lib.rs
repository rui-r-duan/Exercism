pub fn factors(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![];
    }

    let mut result = vec![];
    let mut quotient = n;
    for p in generate_primes(n) {
        while quotient != 1 {
            if quotient % p == 0 {
                result.push(p);
                quotient = quotient / p;
            } else {
                break;
            }
        }
        if quotient == 1 {
            break;
        }
    }

    result
}

fn generate_primes(n: u64) -> Vec<u64> {
    assert!(n > 1);

    let mut a = vec![true; (n + 1).try_into().unwrap()];

    let sqn = (n as f64).sqrt().floor() as usize;
    for i in 2..=sqn {
        if a[i] {
            let mut k = 0;
            loop {
                let j = i.pow(2) + k * i;
                if j <= (n as usize) {
                    a[j] = false;
                    k += 1;
                } else {
                    break;
                }
            }
        }
    }

    (2..=n).filter(|&i| a[i as usize] == true).collect()
}
