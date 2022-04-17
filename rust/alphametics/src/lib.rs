use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let tokens: Vec<&str> = input.split_ascii_whitespace().collect();
    let equal_sign_pos = tokens
        .iter()
        .position(|&e| e == "==")
        .expect("input does not have `==`");
    let left = &tokens[0..equal_sign_pos];
    let right = &tokens[equal_sign_pos + 1..];
    let left_terms: Vec<String> = left
        .split(|&t| t.trim() == "+")
        .flat_map(|t| t.to_vec())
        .map(|t| t.chars().collect::<String>())
        .collect::<Vec<_>>();
    let right_term: String = right[0].chars().collect::<String>();

    let characters: Vec<char> = Vec::from_iter(
        input
            .chars()
            .filter(|t| t.is_alphabetic())
            .collect::<HashSet<char>>()
            .iter()
            .cloned(),
    );

    let permutations = "0123456789".chars().permutations(characters.len());
    for perm in permutations {
        let dict = std::iter::zip(characters.clone(), perm).collect::<Vec<_>>();

        let mut found_err = false;
        let mut addends: Vec<u64> = Vec::new();
        for term in left_terms.iter() {
            match term_to_num(term, &dict) {
                Ok(num) => addends.push(num),
                Err(_) => {
                    found_err = true;
                }
            }
        }
        if found_err {
            continue;
        }

        match term_to_num(&right_term, &dict) {
            Ok(num) => {
                let sum = num;
                if addends.iter().sum::<u64>() == sum {
                    let mut map: HashMap<char, u8> = HashMap::new();
                    for (c, v) in dict {
                        let nv = v.to_digit(10).unwrap() as u8;
                        map.insert(c, nv);
                    }
                    return Some(map);
                }
            }
            Err(_) => {
                found_err = true;
            }
        }
        if found_err {
            continue;
        }
    }

    None
}

fn term_to_num(term: &str, dict: &[(char, char)]) -> Result<u64, String> {
    let num_str = term
        .chars()
        .map(|c| dict.iter().find(|pair| pair.0 == c).unwrap().1)
        .collect::<String>();
    if num_str.chars().nth(0).unwrap() == '0' {
        return Err(format!("Leading zero is not valid: {}", num_str));
    }
    match num_str.parse() {
        Ok(v) => Ok(v),
        Err(err) => Err(format!(
            "Failed conversion to u32: {:?} for {}",
            err.kind(),
            num_str
        )),
    }
}
