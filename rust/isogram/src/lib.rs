use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut chars: HashMap<char, bool> = HashMap::new();
    for c in candidate.chars() {
        if c != '-' && c != ' ' {
            let c = c.to_ascii_lowercase();
            // if chars.contains_key(&c) {
            //     return false;
            // } else {
            //     chars.insert(c, true);
            // }
            if let Entry::Vacant(e) = chars.entry(c) {
                e.insert(true);
            } else {
                return false;
            }
        }
    }
    true
}
