use std::cmp::Ordering;
use std::fmt;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands_vec = hands.iter().map(|&h| PokerHand::new(h)).collect::<Vec<_>>();
    hands_vec.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let winner = &hands_vec[0];
    hands_vec
        .iter()
        .filter(|&a| a == winner)
        .map(|a| a.card_str_ref)
        .collect()
}

#[derive(Debug)]
struct HandRankingCategory {
    rank: u32,
    data: Vec<CardRank>, // sorted in descending order
}

#[derive(Debug)]
struct PokerHand<'a> {
    card_str_ref: &'a str,
    rank_sum: u32,
    category: HandRankingCategory,
}

impl<'a> PokerHand<'a> {
    fn new(hand_str: &'a str) -> Self {
        let cards: Vec<Card> = hand_str.split(' ').map(|c| Card::from(c)).collect();
        PokerHand {
            card_str_ref: hand_str,
            rank_sum: cards.iter().map(|c| c.rank).sum(),
            category: PokerHand::calc_category(&cards),
        }
    }

    fn calc_category(cards: &[Card]) -> HandRankingCategory {
        let rc = PokerHand::rank_count_sorted(cards);
        let result = match rc.as_slice() {
            [(a, 4), (b, 1)] => HandRankingCategory {
                rank: 8, // four of a kind
                data: vec![*a, *b],
            },
            [(a, 3), (b, 2)] => HandRankingCategory {
                rank: 7, // full house
                data: vec![*a, *b],
            },
            [(a, 3), (b, 1), (c, 1)] => HandRankingCategory {
                rank: 4, // three of a kind
                data: vec![*a, *b, *c],
            },
            [(a, 2), (b, 2), (c, 1)] => HandRankingCategory {
                rank: 3, // two pair
                data: vec![*a, *b, *c],
            },
            [(a, 2), (b, 1), (c, 1), (d, 1)] => HandRankingCategory {
                rank: 2, // one pair
                data: vec![*a, *b, *c, *d],
            },
            _ => {
                let mut ranks_sorted: Vec<CardRank> = cards.iter().map(|&x| x.rank).collect();
                ranks_sorted.sort();
                ranks_sorted.reverse();
                let is_same_suit = cards.windows(2).all(|c| c[0].suit == c[1].suit);
                let (is_straight, r) = PokerHand::is_straight(&ranks_sorted);
                match (is_same_suit, is_straight) {
                    (true, true) => HandRankingCategory {
                        rank: 9, // straight flush
                        data: vec![r],
                    },
                    (true, false) => HandRankingCategory {
                        rank: 6, // flush
                        data: ranks_sorted,
                    },
                    (false, true) => HandRankingCategory {
                        rank: 5, // straight
                        data: vec![r],
                    },
                    _ => HandRankingCategory {
                        rank: 1,
                        data: ranks_sorted,
                    },
                }
            }
        };
        return result;
    }

    /// Returns rank counts sorted in descending order.
    fn rank_count_sorted(cards: &[Card]) -> Vec<(CardRank, u8)> {
        let mut rc: [u8; 15] = [0; 15];
        for &card in cards.iter() {
            rc[card.rank as usize] += 1;
        }
        let mut result: Vec<(CardRank, u8)> = rc
            .iter()
            .enumerate()
            .filter(|&(_, &v)| v > 0)
            .map(|(i, &v)| (i as CardRank, v))
            .collect();
        result.sort_by(|&(ar, ac), &(br, bc)| {
            if bc > ac {
                Ordering::Greater
            } else if bc < ac {
                Ordering::Less
            } else {
                if br > ar {
                    Ordering::Greater
                } else if br < ar {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
        });
        result
    }

    fn is_straight(ranks_sorted: &[CardRank]) -> (bool, CardRank) {
        match ranks_sorted[..] {
            [14, 5, 4, 3, 2] => return (true, 5),
            _ => (),
        }
        for i in 1..5 {
            if ranks_sorted[i] != ranks_sorted[i - 1] - 1 {
                return (false, ranks_sorted[0]);
            }
        }
        (true, ranks_sorted[0])
    }
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.rank_sum == other.rank_sum
    }
}

fn ranks_cmp(ranks1: &[CardRank], ranks2: &[CardRank]) -> Ordering {
    assert_eq!(ranks1.len(), ranks2.len());
    let mut result = Ordering::Equal;
    for i in 0..ranks1.len() {
        if ranks1[i] > ranks2[i] {
            result = Ordering::Greater;
            break;
        } else if ranks1[i] < ranks2[i] {
            result = Ordering::Less;
            break;
        } else {
            continue;
        }
    }
    result
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.category.rank > other.category.rank {
            Some(Ordering::Greater)
        } else if self.category.rank < other.category.rank {
            Some(Ordering::Less)
        } else {
            // same category
            Some(ranks_cmp(&self.category.data, &other.category.data))
        }
    }
}

type CardRank = u32;
type CardSuit = char;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Card {
    rank: CardRank,
    suit: CardSuit,
}

impl Card {
    fn from(card_str: &str) -> Self {
        let n = card_str.len();
        let rank = match &card_str[..n - 1] {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            m => m.parse().unwrap(),
        };
        let suit = card_str.chars().nth(n - 1).unwrap();
        Card { rank, suit }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit as char)
    }
}

#[test]
fn test_print_card() {
    let h = Card {
        rank: 11,
        suit: 'H',
    };
    let s = format!("{:?}", h);
    assert_eq!(s, "11H");
}

#[test]
fn test_hand_eq() {
    let h1 = PokerHand::new("5H 5S 5D 9S 9D");
    let h2 = PokerHand::new("5H 9S 5S 5S 9D");
    assert!(h1 == h2);
}

#[test]
fn test_hand_order() {
    let h1 = PokerHand::new("5H 5S 5D 9S 9D");
    let h2 = PokerHand::new("5H 9S 5S 4S 9D");
    assert!(h1 > h2);
}

#[test]
fn test_categories() {
    let h2 = PokerHand::new("JC 10C 9C 7C 8C");
    assert_eq!(h2.category.rank, 9);

    let h3 = PokerHand::new("5C 5D 5H 5S 2D");
    assert_eq!(h3.category.rank, 8);

    let h4 = PokerHand::new("6S 6H 6D KC KH");
    assert_eq!(h4.category.rank, 7);

    let h5 = PokerHand::new("JD 9D 8D 4D 3D");
    assert_eq!(h5.category.rank, 6);

    let h6 = PokerHand::new("10D 9S 8H 7D 6C");
    assert_eq!(h6.category.rank, 5);

    let h7 = PokerHand::new("QC QS QH 9H 2S");
    assert_eq!(h7.category.rank, 4);

    let h8 = PokerHand::new("JH JS 3C 3S 2H");
    assert_eq!(h8.category.rank, 3);

    let h9 = PokerHand::new("10S 10H 8S 7H 4C");
    assert_eq!(h9.category.rank, 2);

    let h10 = PokerHand::new("KD QD 7S 4S 3H");
    assert_eq!(h10.category.rank, 1);

    let h11 = PokerHand::new("10D JH QS KD AC");
    assert_eq!(h11.category.rank, 5);

    let h12 = PokerHand::new("4D AH 3S 2D 5C");
    assert_eq!(h12.category.rank, 5);
}
