/// translate translates a series of words to pig-latin words.
/// The words must consist of [a-z].
pub fn translate(input: &str) -> String {
    let mut result = String::new();
    for word in input.split_ascii_whitespace() {
        if result.len() > 0 {
            result.push(' ');
        }
        result.push_str(&trans_word(word));
    }
    result
}

fn trans_word(word: &str) -> String {
    let bytes = word.as_bytes();
    if starts_with_vowel(bytes) || starts_with_special_vowel(bytes) {
        // rule 1
        transform_1(word)
    } else {
        let end = prefix_end_idx(bytes);
        transform_2(word, 0..end)
    }
}

fn is_vowel(b: u8) -> bool {
    const VOWELS: &[u8] = &[b'a', b'e', b'i', b'o', b'u'];
    VOWELS.iter().any(|&v| v == b)
}

fn starts_with_vowel(bytes: &[u8]) -> bool {
    is_vowel(bytes[0])
}

fn starts_with_special_vowel(bytes: &[u8]) -> bool {
    match &bytes[0..2] {
        [b'x', b'r'] | [b'y', b't'] => true,
        _ => false,
    }
}

/// prefix_end_idx returns the index of the end of the prefix that should be
/// moved to the end of the pig-latin string.  The index equals to 1 plus
/// the index of the last character of the prefix.
///
/// @pre-condition: the input does not start with a vowel character.
///
/// There are three rules. In this document, they are represented in
/// patterns.
/// In the following patterns, `?` represents a consonant character,
/// `@` represents a vowel character.
///
/// rule 2: `?+@.*$`
///            ^ return this position
///
/// rule 3: `?+qu.*$`
///              ^ return this position
///
/// rule 4: `?+y.*$' OR `?y$`
///            ^          ^ return this position
fn prefix_end_idx(bytes: &[u8]) -> usize {
    assert!(!is_vowel(bytes[0]));

    if bytes.len() == 2 && bytes[1] == b'y' {
        // rule 4
        return 1;
    }

    // Try to find the first vowel char.
    let mut i = 0;
    while !is_vowel(bytes[i]) {
        if bytes[i] == b'y' && i >= 1 {
            // rule 4
            return i;
        }
        i += 1;
    }
    if i == bytes.len() {
        // It does not found any vowel.
        i
    } else {
        // A vowel is found.
        if bytes[i] == b'u' && i >= 1 && bytes[i - 1] == b'q' {
            // rule 3
            i + 1
        } else {
            i
        }
    }
}

/// transform_1 appends "ay" at the end of the input string.
fn transform_1(input: &str) -> String {
    let mut s = String::from(input);
    s += "ay";
    s
}

// transform_2
fn transform_2(input: &str, prefix_range: std::ops::Range<usize>) -> String {
    let mut s = String::from(&input[prefix_range.end..]);
    s += &input[prefix_range];
    s += "ay";
    s
}
pub fn translate2(input: &str) -> String {
    input
        .split_whitespace()
        .map(|s| rotate(s, consonant(s)).chain("ay".chars()).collect())
        .collect::<Vec<String>>()
        .join(" ")
}

fn rotate(s: &str, n: usize) -> impl Iterator<Item = char> + '_ {
    s.chars().cycle().skip(n).take(s.len())
}
fn consonant(s: &str) -> usize {
    let vowel = |c| matches!(c as char, 'a' | 'e' | 'i' | 'o' | 'u');
    match s.as_bytes() {
        [c, ..] if vowel(*c) => 0,
        [b'x', b'r', ..] | [b'y', b't', ..] => 0,
        _ => match s
            .as_bytes()
            .iter()
            .skip(1)
            .position(|&c| vowel(c) || c == b'y')
        {
            Some(i) => match &s.as_bytes()[..i + 2] {
                [.., b'q', b'u'] => i + 2,
                _ => i + 1,
            },
            None => 0,
        },
    }
}
