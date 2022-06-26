use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for c in candidate.chars() {
        if c != '-' && c != ' ' {
            let c = c.to_ascii_lowercase();
            if !chars.insert(c) {
                return false;
            }
        }
    }
    true
}
