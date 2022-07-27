use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if nucleotide == 'A' || nucleotide == 'C' || nucleotide == 'G' || nucleotide == 'T' {
        match nucleotide_counts(dna) {
            Ok(map) => Ok(*map.get(&nucleotide).unwrap()),
            Err(ch) => Err(ch),
        }
    } else {
        Err(nucleotide)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('C', 0);
    map.insert('G', 0);
    map.insert('T', 0);
    for ch in dna.chars() {
        if ch == 'A' || ch == 'C' || ch == 'G' || ch == 'T' {
            map.entry(ch).and_modify(|e| *e += 1);
        } else {
            return Err(ch);
        }
    }
    Ok(map)
}
