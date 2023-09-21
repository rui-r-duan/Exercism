#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strand: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: Vec<u8>,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
	let mut map: [u8; 127] = [0; 127];
	map[b'G' as usize] = b'C';
	map[b'C' as usize] = b'G';
	map[b'T' as usize] = b'A';
	map[b'A' as usize] = b'U';

	for (i, c) in dna.bytes().enumerate() {
	    if map[c as usize] == 0 {
		return Err(i);
	    }
	}
	let mut d = Dna {strand: Vec::with_capacity(dna.len())};
	for c in dna.bytes() {
	    d.strand.push(c);
	}

	Ok(d)
    }

    pub fn into_rna(self) -> Rna {
	let mut map: [u8; 127] = [0; 127];
	map[b'G' as usize] = b'C';
	map[b'C' as usize] = b'G';
	map[b'T' as usize] = b'A';
	map[b'A' as usize] = b'U';
	let mut r = Rna {strand: Vec::with_capacity(self.strand.len())};
	for &c in self.strand.iter() {
	    r.strand.push(map[c as usize]);
	}

	r
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
	let mut map: [bool; 127] = [false; 127];
	map[b'C' as usize] = true;
	map[b'G' as usize] = true;
	map[b'A' as usize] = true;
	map[b'U' as usize] = true;

	for (i, c) in rna.bytes().enumerate() {
	    if !map[c as usize] {
		return Err(i);
	    }
	}
	let mut r = Rna {strand: Vec::with_capacity(rna.len())};
	for c in rna.bytes() {
	    r.strand.push(c);
	}

	Ok(r)

    }
}
