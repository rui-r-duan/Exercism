use std::str;
pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    let windows = digits.as_bytes().windows(len);

    let mut result = Vec::new();
    for x in windows {
        result.push(str::from_utf8(x).unwrap().to_string());
    }
    result
}
