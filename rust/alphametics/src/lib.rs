use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (values, leading_char_count) = parse(input);
    for perm in (0..10u8).permutations(values.len()) {
        if perm[..leading_char_count].iter().any(|&digit| digit == 0) {
            continue;
        }
        let sum = values
            .iter()
            .zip(perm.iter())
            .map(|(&(_, _, value), &digit)| value * (digit as i64))
            .sum::<i64>();
        if sum == 0 {
            return Some(
                values
                    .iter()
                    .zip(perm.iter())
                    .map(|(&(_, ch, _), &digit)| (ch, digit))
                    .collect(),
            );
        }
    }
    None
}

fn parse(input: &str) -> (Vec<(bool, char, i64)>, usize) {
    // EXAMPLE:
    // PARSE "I + BB == ILL" to integer expression: 11*B-99*I-11*L.
    // The expression is represented as follows (psudo code):
    // vec![('B',11,leading=true), ('I',-99,leading=true),('L',-11,false)]
    //
    // Later, we test if it is equal to zero for value (B,I,L).
    //
    // from right to left
    // prev = ' ', value = -1
    // 'L', map={(L, -1)}, prev='L', value*=10 => value=-10
    // 'L', map={(L, -1+value=-11)}, prev='L', value*=10 => value=-100
    // 'I', map={(I, 0+value=-100)}, prev='I', value*=10 => value=-1000
    // ' ', prev=='I' is alphabetic => leading_set={I}, value=1, prev=' '
    // '=', prev='='
    // '=', prev='='
    // ' ', prev=' '
    // 'B', map={(L,-11),(I,-100),(B, 0+value=1)}, prev='B', value*=10 => value=10
    // 'B', map={(L,-11),(I,-100),(B, 1+value=11)}, prev='B', value*=10 => value=100
    // ' ', prev == 'B' is alphabetic => leading_set={I, B}, value=1, prev = ' '
    // 'I', map={(L,-11),(I,-100+value=-99),(B,11)}, prev='I', value*=10 => value=10
    // END
    let mut map = HashMap::new();
    let mut prev = ' ';
    let mut value = -1;
    let mut leadings = HashSet::new();
    for ch in input.chars().rev() {
        match ch {
            'A'..='Z' => {
                *map.entry(ch).or_insert(0) += value;
                prev = ch;
                value *= 10;
            }
            _ => {
                if prev.is_alphabetic() {
                    leadings.insert(prev);
                    value = 1;
                    prev = ch;
                }
            }
        }
    }
    leadings.insert(prev);
    let mut result = Vec::new();
    for elem in leadings.iter() {
        let (&k, &v) = map.get_key_value(elem).unwrap();
        result.push((true, k, v));
    }
    for elem in HashSet::from_iter(map.keys().cloned()).difference(&leadings) {
        let (&k, &v) = map.get_key_value(elem).unwrap();
        result.push((false, k, v));
    }
    (result, leadings.len())
}

#[test]
fn test_parse() {
    let a = parse("SEND + MORE == MONEY");
    println!("{:?}", a);
}
