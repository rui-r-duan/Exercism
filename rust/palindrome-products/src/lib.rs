/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let vs = value.to_string();
        let s = vs.as_bytes();
        let mut i = 0;
        let n = s.len();
        while i < n / 2 {
            if s[i] != s[n - i - 1] {
                return None;
            }
            i += 1;
        }
        Some(Palindrome(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut smallest = max * max;
    let mut largest = min * min;
    let mut found = false;
    for i in min..=max {
        for j in min..=max {
            let v = i * j;
            if Palindrome::new(v).is_some() {
                if !found {
                    found = true;
                }
                if v < smallest {
                    smallest = v;
                }
                if v > largest {
                    largest = v;
                }
            }
        }
    }
    if found {
        Some((Palindrome(smallest), Palindrome(largest)))
    } else {
        None
    }
}
