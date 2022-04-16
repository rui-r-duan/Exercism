from enum import IntEnum
from typing import List


class Allergen(IntEnum):
    Eggs = 1
    Peanuts = 2
    Shellfish = 4
    Strawberries = 8
    Tomatoes = 16
    Chocolate = 32
    Pollen = 64
    Cats = 128


class Allergies:

    def __init__(self, score):
        self.allergens = score

    def is_allergic_to(self, allergen: Allergen) -> bool:
        return self.allergens & int(allergen) == allergen

    def allergies(self) -> List[Allergen]:
        return [x for x in Allergen if self.is_allergic_to(x)]
