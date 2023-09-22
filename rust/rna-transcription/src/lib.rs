#[derive(Debug, PartialEq, Eq)]
pub struct Dna(Vec<u8>);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(Vec<u8>);

const fn dna_to_rna_nucleotide_map() -> [u8; 127] {
    let mut map: [u8; 127] = [0; 127];
    map[b'G' as usize] = b'C';
    map[b'C' as usize] = b'G';
    map[b'T' as usize] = b'A';
    map[b'A' as usize] = b'U';

    map
}

const fn rna_nucleotide_map() -> [bool; 127] {
    let mut map: [bool; 127] = [false; 127];
    map[b'C' as usize] = true;
    map[b'G' as usize] = true;
    map[b'A' as usize] = true;
    map[b'U' as usize] = true;

    map
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
	const MAP: [u8; 127] = dna_to_rna_nucleotide_map();
        for (i, c) in dna.bytes().enumerate() {
            if MAP[c as usize] == 0 {
                return Err(i);
            }
        }
        let mut d = Dna(Vec::with_capacity(dna.len()));
        for c in dna.bytes() {
            d.0.push(c);
        }

        Ok(d)
    }

    pub fn into_rna(self) -> Rna {
	const MAP: [u8; 127] = dna_to_rna_nucleotide_map();
        let mut r = Rna(Vec::with_capacity(self.0.len()));
        for &c in self.0.iter() {
            r.0.push(MAP[c as usize]);
        }

        r
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
	const MAP: [bool; 127] = rna_nucleotide_map();
        for (i, c) in rna.bytes().enumerate() {
            if !MAP[c as usize] {
                return Err(i);
            }
        }
        let mut r = Rna(Vec::with_capacity(rna.len()));
        for c in rna.bytes() {
            r.0.push(c);
        }

        Ok(r)
    }
}
