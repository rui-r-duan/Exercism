pub fn encode(source: &str) -> String {
    let mut ans = String::new();
    let mut chars = source.chars().peekable();
    let mut count = 0;
    while let Some(curr) = chars.next() {
        count += 1;
        if chars.peek() != Some(&curr) {
            if count > 1 {
                ans.push_str(&count.to_string());
            }
            ans.push(curr);
            count = 0;
        }
    }

    ans
}

pub fn decode(source: &str) -> String {
    let mut ans = String::new();
    let mut digits = String::new();
    for curr in source.chars() {
        if curr.is_ascii_digit() {
            digits.push(curr);
        } else {
            let n = digits.parse::<usize>().unwrap_or(1);
            ans.push_str(&curr.to_string().repeat(n));
            digits.clear();
        }
    }

    ans
}
