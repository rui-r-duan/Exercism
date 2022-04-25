use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let n = input.len();
    if n < worker_count {
        return letter_count(input);
    } else {
        let lines_per_chunk = if n % worker_count == 0 {
            n / worker_count
        } else {
            n / worker_count + 1
        };
        input
            .par_chunks(lines_per_chunk)
            .map(|chunk| letter_count(&chunk))
            .reduce(
                || HashMap::new(),
                |mut map, small_map| {
                    for (k, v) in small_map {
                        *map.entry(k).or_default() += v;
                    }
                    map
                },
            )
    }
}

fn letter_count(lines: &[&str]) -> HashMap<char, usize> {
    let mut ans = HashMap::new();
    for line in lines {
        for ch in line.chars().filter(|c| c.is_alphabetic()) {
            if let Some(c) = ch.to_lowercase().next() {
                *ans.entry(c).or_insert(0) += 1;
            }
        }
    }
    ans
}
