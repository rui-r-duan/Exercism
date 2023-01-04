#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    } else if num == 1 {
        return Some(Classification::Deficient);
    }

    let s: u64 = factors_without_self(num).iter().sum();
    if s == num {
        Some(Classification::Perfect)
    } else if s > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}

fn factors_without_self(num: u64) -> Vec<u64> {
    let mut result = vec![1];
    let n: u64 = (num as f64).sqrt() as u64;
    for i in 2..=n {
        let q = num / i;
        let r = num % i;
        if r == 0 {
            result.push(i);
            if q > i {
                result.push(q);
            }
        }
    }

    result
}

use std::cmp::Ordering;
pub fn classify2(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else {
        let n: u64 = (num as f64).sqrt() as u64;
        match (1..n).filter(|&f| num % f == 0).sum::<u64>().cmp(&num) {
            Ordering::Less => Some(Classification::Deficient),
            Ordering::Equal => Some(Classification::Perfect),
            Ordering::Greater => Some(Classification::Abundant),
        }
    }
}
