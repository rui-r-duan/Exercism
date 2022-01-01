use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples: HashSet<u32> = HashSet::new();
    for &factor in factors {
        if factor == 0 {
            continue;
        }
        let mut i: u32 = 1;
        loop {
            let prod = factor * i;
            if prod < limit {
                multiples.insert(prod);
                i += 1;
            } else {
                break;
            }
        }
    }

    multiples.iter().sum()
}
