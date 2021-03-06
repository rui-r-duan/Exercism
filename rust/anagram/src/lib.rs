// use std::collections::HashSet;

// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//     let word_lower = word.to_lowercase();
//     let word_sorted = get_sorted(&word_lower);
//     possible_anagrams
//         .iter()
//         .filter(|candidate| {
//             let candidate_lower = candidate.to_lowercase();
//             candidate_lower.len() == word_lower.len()
//                 && candidate_lower != word_lower
//                 && get_sorted(&candidate_lower) == word_sorted
//         })
//         .copied()
//         .collect()
// }

// fn get_sorted(word: &str) -> Vec<char> {
//     let mut word_sorted: Vec<char> = word.chars().collect();
//     word_sorted.sort_unstable();
//     word_sorted
// }

use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();
    let word_chars_count = word.chars().count();
    let word_lowercase = word.to_lowercase();
    let mut word_char_map = HashMap::new();
    for c in word.chars() {
        let lc = c.to_lowercase().to_string();
        let count = word_char_map.entry(lc).or_insert(0);
        *count += 1;
    }

    let is_anagram = |possible: &str| {
        if possible.chars().count() != word_chars_count {
            return false;
        }

        if possible.to_lowercase() == word_lowercase {
            return false;
        }

        let mut map = word_char_map.clone();

        for c in possible.chars() {
            let lc = c.to_lowercase().to_string();
            let count = map.get_mut(&lc);
            match count {
                Some(cnt) => {
                    *cnt -= 1;
                }
                None => {
                    return false;
                }
            }
        }

        map.values().all(|&x| x == 0)
    };

    for w in possible_anagrams {
        if is_anagram(*w) {
            result.insert(*w);
        }
    }

    result
}
