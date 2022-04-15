pub struct Allergies {
    allergens: u32,
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

const ALLERGENS: [Allergen; 8] = [
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { allergens: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen = *allergen as u32;
        self.allergens & allergen == allergen
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .into_iter()
            .filter(|x: &Allergen| self.is_allergic_to(x))
            .collect()
        // ALLERGENS
        //     .iter()
        //     .filter(|x: &&Allergen| self.is_allergic_to(x))
        //     .cloned()
        //     .collect()
    }
}
