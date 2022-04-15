pub struct Allergies {
    data: Vec<(Allergen, u32)>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs = 0x01,
    Peanuts = 0x02,
    Shellfish = 0x04,
    Strawberries = 0x08,
    Tomatoes = 0x10,
    Chocolate = 0x20,
    Pollen = 0x40,
    Cats = 0x80,
}

use Allergen::*;
impl Allergies {
    pub fn new(score: u32) -> Self {
        let table = [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ];
        const N: usize = 8;
        let marks: [u32; N] = table.map(|allergen| (allergen as u32) & score);
        let mut data: Vec<(Allergen, u32)> = Vec::new();
        for i in 0..N {
            data.push((table[i], marks[i]));
        }
        Allergies { data }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.data
            .iter()
            .find(|(a, v)| a == allergen && *v != 0)
            .is_some()
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.data
            .iter()
            .filter(|(_a, v)| *v != 0)
            .map(|&(a, _v)| a)
            .collect()
    }
}
