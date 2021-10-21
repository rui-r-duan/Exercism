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

    let is_anagram_of = |possible: &str| {
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
        if is_anagram_of(*w) {
            result.insert(*w);
        }
    }

    result
}
