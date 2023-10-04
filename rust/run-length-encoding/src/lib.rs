use std::iter;

pub fn encode(source: &str) -> String {
    let mut ans = String::new();
    let mut remainder = source;
    while let Some(curr) = remainder.chars().next() {
        let count = remainder.chars().take_while(|&c| c == curr).count();
        match count {
            1 => ans.push(curr),
            _ => ans.push_str(&format!("{}{}", count, curr)),
        }
        remainder = &remainder[count * curr.len_utf8()..];
    }

    ans
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .filter(|c: &char| !c.is_ascii_digit())
        .zip(
            source
                .split(|c: char| !c.is_ascii_digit())
                .map(|num| num.parse::<usize>().unwrap_or(1)),
        )
        .flat_map(|(c, count)| iter::repeat(c).take(count))
        .collect()
}
