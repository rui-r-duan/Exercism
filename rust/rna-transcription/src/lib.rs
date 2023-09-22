use std::ptr;

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
        match dna.bytes().position(|x| MAP[x as usize] == 0) {
            Some(i) => return Err(i),
            None => {}
        }

        let mut d = Dna(Vec::with_capacity(dna.len()));
        let dst_ptr = d.0.as_mut_ptr();
        let src_ptr = dna.as_ptr();
        unsafe {
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, dna.len());

            // Notify the destination vector that it now holds the content
            // os the source.
            d.0.set_len(dna.len());
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
        rna.as_bytes().clone_into(&mut r.0);
        // for c in rna.bytes() {
        //     r.0.push(c);
        // }

        Ok(r)
    }
}
