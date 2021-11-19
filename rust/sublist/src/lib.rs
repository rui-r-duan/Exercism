#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    let len_a = first.len();
    let len_b = second.len();
    if len_a == len_b {
        if sublist_recur(first, second) {
            Comparison::Equal
        } else {
            Comparison::Unequal
        }
    } else if len_a < len_b {
        if len_a == 0 {
            Comparison::Sublist
        } else {
            for (i, e) in second.iter().enumerate() {
                if e == &first[0] {
                    if sublist_recur(first, &second[i..]) {
                        return Comparison::Sublist;
                    }
                }
            }

            Comparison::Unequal
        }
    } else {
        match sublist(second, first) {
            Comparison::Equal => Comparison::Equal,
            Comparison::Sublist => Comparison::Superlist,
            Comparison::Superlist => Comparison::Sublist,
            Comparison::Unequal => Comparison::Unequal,
        }
    }
}

fn sublist_recur<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    let len_a = a.len();
    let len_b = b.len();
    if len_a > len_b {
        return false;
    }

    if len_a == 0 {
        true
    } else if a[0] == b[0] {
        sublist_recur(&a[1..], &b[1..])
    } else {
        false
    }
}
