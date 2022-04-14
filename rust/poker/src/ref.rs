use std::cmp::Ordering;
use std::fmt;
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
struct Card<'a>(&'a str);
impl<'a> From<&'a str> for Card<'a> {
    fn from(s: &'a str) -> Card<'a> {
        Card(s)
    }
}
impl<'a> Card<'a> {
    fn value(&self) -> Option<u32> {
        match self.0.chars().next() {
            Some('A') => Some(14),
            Some('K') => Some(13),
            Some('Q') => Some(12),
            Some('J') => Some(11),
            Some('1') => Some(10),
            Some(x) => x.to_digit(10),
            _ => None,
        }
    }
    fn value_aces_low(&self) -> Option<u32> {
        match self.0.chars().next() {
            Some('A') => Some(1),
            _ => self.value(),
        }
    }
    fn suit(&self) -> Option<char> {
        self.0.chars().rev().next()
    }
    fn partial_cmp_aces_low(&self, other: &Self) -> Option<Ordering> {
        self.value_aces_low()
            .and_then(|x| other.value_aces_low().and_then(|y| x.partial_cmp(&y)))
    }
}
impl<'a> PartialOrd for Card<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value()
            .and_then(|x| other.value().and_then(|y| x.partial_cmp(&y)))
    }
}
impl<'a> Ord for Card<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}
#[derive(Debug, PartialEq)]
struct PokerHand<'a> {
    hand: &'a str,
    cards: Vec<Card<'a>>,
    hand_type: HandType,
}
impl<'a> fmt::Display for PokerHand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hand: {}, Type: {}, Cards: {}",
            self.hand,
            format!("{:?}", self.hand_type),
            self.cards.iter().map(|x| x.0).collect::<Vec<_>>().join(" ")
        )
    }
}
impl HandType {
    fn new<'a>(pairs: &[&[Card<'a>]]) -> HandType {
        // Each card is one higher than the following card
        let straight = pairs
            .concat()
            .windows(2)
            .all(|s| s[0].value().unwrap_or(0) == 1 + s[1].value().unwrap_or(0));
        // As a straight, but the last card may be a low ace
        let straight_aces_low = [pairs[1..].concat(), pairs[0..1].concat()]
            .concat()
            .windows(2)
            .all(|s| s[0].value().unwrap_or(0) == 1 + s[1].value_aces_low().unwrap_or(0));
        // All cards have the same suit
        let flush = pairs
            .concat()
            .windows(2)
            .all(|s| s[0].suit() == s[1].suit());

        // Ranking of poker hands
        if (straight || straight_aces_low) && flush {
            HandType::StraightFlush
        } else if !pairs.is_empty() && pairs[0].len() == 4 {
            HandType::FourOfAKind
        } else if pairs.len() == 2 {
            HandType::FullHouse
        } else if flush {
            HandType::Flush
        } else if straight || straight_aces_low {
            HandType::Straight
        } else if !pairs.is_empty() && pairs[0].len() == 3 {
            HandType::ThreeOfAKind
        } else if pairs.len() == 3 {
            HandType::TwoPair
        } else if pairs.len() == 4 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}
/// Implementation of group_by as currently this is in unstable
struct GroupBy<'a, T: 'a, F> {
    slice: &'a [T],
    thunk: F,
    start: usize,
}
impl<'a, T, F: FnMut(&T, &T) -> bool> Iterator for GroupBy<'a, T, F> {
    //! Group consecutive items into slices
    type Item = &'a [T];
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.slice.len() {
            return None;
        }
        let start = self.start;
        let mut end = self.start;
        while end < self.slice.len() && (self.thunk)(&self.slice[start], &self.slice[end]) {
            end += 1;
        }
        self.start = end;
        Some(&self.slice[start..end])
    }
}
trait GroupIt<'a, T, F> {
    fn group_by_l(&self, thunk: F) -> GroupBy<'a, T, F>;
}
impl<'a, T, F> GroupIt<'a, T, F> for &'a [T]
where
    T: 'a,
    F: FnMut(&T, &T) -> bool,
{
}
impl<'a, T: 'a, F: FnMut(&T, &T) -> bool> GroupIt<'a, T, F> for &'a [T] {
    fn group_by_l(&self, thunk: F) -> GroupBy<'a, T, F> {
        GroupBy {
            slice: self,
            thunk,
            start: 0,
        }
    }
}
impl<'a> From<&&'a str> for PokerHand<'a> {
    fn from(hand: &&'a str) -> PokerHand<'a> {
        // Split the hand up into a vector of cards
        let mut cards = hand
            .split_whitespace()
            .map(|x| x.into())
            .collect::<Vec<Card>>();
        // Group the cards into groups of equal value
        cards.sort_by(|b, a| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        let mut pairs = cards
            .as_slice()
            .group_by_l(|a, b| a.value() == b.value())
            .collect::<Vec<_>>();
        // Sort the groups first by length, then by value (aces high)
        pairs.sort_by(|b, a| {
            a.len().cmp(&b.len()).then(
                a.iter()
                    .next()
                    .partial_cmp(&b.iter().next())
                    .unwrap_or(Ordering::Equal),
            )
        });
        // Determine the type of hand we have
        let hand_type = HandType::new(pairs.as_slice());
        // Special case for straights:
        // if the first items are not in a straight then this is a
        // straight with aces low
        if hand_type == HandType::Straight
            && pairs[0][0].value() != pairs[1][0].value().map(|x| x + 1)
        {
            cards.sort_by(|b, a| a.partial_cmp_aces_low(b).unwrap_or(Ordering::Equal));
            PokerHand {
                hand,
                cards,
                hand_type,
            }
        } else {
            PokerHand {
                hand,
                cards: pairs.concat(),
                hand_type,
            }
        }
    }
}
impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.hand_type
                .cmp(&other.hand_type)
                .then(self.cards.cmp(&other.cards)),
        )
    }
}
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // Convert the strings into poker hands
    let mut vec = hands
        .iter()
        .map(|x| x.into())
        .collect::<Vec<PokerHand<'a>>>();
    // Sort the hands based on their partial ordering in reverse
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal).reverse());
    // Return the group of hands considered equal
    vec.as_slice()
        .group_by_l(|a, b| a.partial_cmp(b) == Some(Ordering::Equal))
        .next()
        .unwrap_or(&[])
        .iter()
        .map(|x| x.hand)
        .collect::<Vec<_>>()
}
