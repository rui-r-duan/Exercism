#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match (first.len(), second.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (m, n) if m < n => {
            if second.windows(m).any(|x| x == first) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (m, n) if m > n => {
            if first.windows(n).any(|x| x == second) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (_, _) => {
            if first == second {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}
