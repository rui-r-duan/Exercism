from allergies import (Allergies, Allergen)


def contain_same_elements(expected, actual):
    if len(expected) != len(actual):
        return False
    for x in expected:
        if not x in actual:
            return False
    return True


def test_is_not_allergic_to_anything():
    allergies = Allergies(0)
    assert not allergies.is_allergic_to(Allergen.Peanuts)
    assert not allergies.is_allergic_to(Allergen.Cats)
    assert not allergies.is_allergic_to(Allergen.Strawberries)


def test_is_allergic_to_egg_shellfish_and_strawberries():
    allergies = Allergies(5)
    assert allergies.is_allergic_to(Allergen.Eggs)
    assert allergies.is_allergic_to(Allergen.Shellfish)
    assert not allergies.is_allergic_to(Allergen.Strawberries)


def test_allergic_to_eggs_and_peanuts():
    allergies = Allergies(3)
    assert allergies.is_allergic_to(Allergen.Eggs)
    assert allergies.is_allergic_to(Allergen.Peanuts)
    allergens = allergies.allergies()
    assert contain_same_elements(allergens, [Allergen.Peanuts, Allergen.Eggs])