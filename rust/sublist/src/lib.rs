#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    let (m, n) = (first.len(), second.len());
    if m == 0 && n == 0 {
        Comparison::Equal
    } else if m == 0 {
        // n > 0
        Comparison::Sublist
    } else if n == 0 {
        // m > 0
        Comparison::Superlist
    } else if m == n {
        // 0 < m == n
        if first == second {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if m < n {
        // 0 < m < n
        if second.windows(m).any(|x| x == first) {
            Comparison::Sublist
        } else {
            Comparison::Unequal
        }
    } else {
        // 0 < n < m
        if first.windows(n).any(|x| x == second) {
            Comparison::Superlist
        } else {
            Comparison::Unequal
        }
    }
}
