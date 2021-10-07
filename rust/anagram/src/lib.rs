use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result = HashSet::new();

    for w in possible_anagrams {
        if (*w).is_anagram_of(word) {
            result.insert(*w);
        }
    }

    result
}

pub trait Anagram {
    fn is_anagram_of(&self, word: &str) -> bool;
}

impl Anagram for str {
    fn is_anagram_of(&self, word: &str) -> bool {
        if self.chars().count() != word.chars().count() {
            return false;
        }

        if self.to_lowercase() == word.to_lowercase() {
            return false;
        }

        let mut map = HashMap::new();

        for c in word.chars() {
            let lc = c.to_lowercase().to_string();
            let count = map.entry(lc).or_insert(0);
            *count += 1;
        }

        for c in self.chars() {
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
    }
}
