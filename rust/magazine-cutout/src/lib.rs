use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut word_count = HashMap::new();
    for word in note {
        let count = word_count.entry(*word).or_insert(0);
        *count += 1;
    }

    for word in magazine {
        if let Some(count) = word_count.get_mut(*word) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }

    word_count.values().all(|x| *x == 0)
}
