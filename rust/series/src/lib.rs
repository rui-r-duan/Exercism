use std::str;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    digits
        .as_bytes()
        .windows(len)
        .map(|x| x.iter().map(|&c| c as char).collect::<String>())
        .collect()
}
