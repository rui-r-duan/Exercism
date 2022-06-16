use std::collections::BTreeMap;

pub fn transform_imperative(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut a = BTreeMap::new();
    for (&score, letters) in h {
        for c in letters {
            a.insert(c.to_ascii_lowercase(), score);
        }
    }
    a
}

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&score, letters)| letters.iter().map(move |c| (c.to_ascii_lowercase(), score)))
        .collect()
}
