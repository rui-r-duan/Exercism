use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::mem;
use type_layout::TypeLayout;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    println!("{}", mem::size_of::<PokerHand>());
    println!("{}", PokerHand::type_layout());
    let mut hands_vec = vec![];
    for &hand in hands {
        let ph = PokerHand::new(hand);
        hands_vec.push(ph);
    }
    hands_vec[..].sort_by(|a, b| b.partial_cmp(a).unwrap());
    let winner = &hands_vec[0];
    hands_vec
        .iter()
        .filter(|&a| a == winner)
        .map(|a| a.card_str_ref)
        .collect()
}

#[derive(TypeLayout, Debug)]
#[repr(C)]
struct PokerHand<'a> {
    card_str_ref: &'a str,
    cards: [Card; 5],
    category_rank: u8,
    category: HandRankingCategory,
}

#[derive(Debug)]
enum HandRankingCategory {
    FiveOfAKind(CardRank),
    /// sorted continuous sequence (descending)
    StraightFlush(CardRank),
    /// 4 + 1(kicker)
    FourOfAKind(CardRank, CardRank),
    /// 3 + 2
    FullHouse(CardRank, CardRank),
    /// sorted sequence (descending)
    Flush(Vec<CardRank>),
    /// sorted continuous sequence (descending)
    Straight(CardRank),
    /// 3 + 1 + 1
    ThreeOfAKind(CardRank, CardRank, CardRank),
    /// 2 + 2 + 1
    TwoPair(CardRank, CardRank, CardRank),
    /// 2 + 1 + 1 + 1  
    OnePair(CardRank, CardRank, CardRank, CardRank),
    /// sorted sequence (descending)
    HighCard(Vec<CardRank>),
}

impl<'a> PokerHand<'a> {
    fn new(hand_str: &'a str) -> Self {
        let mut cards: [Card; 5] = [Card::new(); 5];
        let cards_str: Vec<&str> = hand_str.split(' ').collect();
        let mut i = 0;
        for c in cards_str {
            let card = Card::from(c);
            cards[i] = card;
            i += 1;
        }
        let (category, category_rank) = PokerHand::calc_category(&cards);
        PokerHand {
            card_str_ref: hand_str,
            cards,
            category_rank,
            category,
        }
    }

    fn calc_category(cards: &[Card]) -> (HandRankingCategory, u8) {
        if PokerHand::is_five_of_a_kind(cards) {
            (HandRankingCategory::FiveOfAKind(cards[0].rank), 10)
        } else if let (true, r) = PokerHand::is_straight_flush(cards) {
            (HandRankingCategory::StraightFlush(r), 9)
        } else if let (true, r1, r2) = PokerHand::is_four_of_a_kind(cards) {
            (HandRankingCategory::FourOfAKind(r1, r2), 8)
        } else if let (true, r1, r2) = PokerHand::is_full_house(cards) {
            (HandRankingCategory::FullHouse(r1, r2), 7)
        } else if PokerHand::is_flush(cards) {
            let mut ranks_sorted: Vec<CardRank> = cards.iter().map(|&x| x.rank).collect();
            ranks_sorted.sort();
            ranks_sorted.reverse();
            (HandRankingCategory::Flush(ranks_sorted), 6)
        } else if let (true, r) = PokerHand::is_straight(cards) {
            (HandRankingCategory::Straight(r), 5)
        } else if let (true, r1, r2, r3) = PokerHand::is_three_of_a_kind(cards) {
            (HandRankingCategory::ThreeOfAKind(r1, r2, r3), 4)
        } else if let (true, r1, r2, r3) = PokerHand::is_two_pair(cards) {
            (HandRankingCategory::TwoPair(r1, r2, r3), 3)
        } else if let (true, r1, r2, r3, r4) = PokerHand::is_one_pair(cards) {
            (HandRankingCategory::OnePair(r1, r2, r3, r4), 2)
        } else {
            let mut ranks_sorted: Vec<CardRank> = cards.iter().map(|&x| x.rank).collect();
            ranks_sorted.sort();
            ranks_sorted.reverse();
            (HandRankingCategory::HighCard(ranks_sorted), 1)
        }
    }

    fn rank_count(cards: &[Card]) -> Vec<(CardRank, u8)> {
        let mut rc: [u8; 15] = [0; 15];
        for &card in cards.iter() {
            rc[card.rank as usize] += 1;
        }
        rc.iter()
            .enumerate()
            .filter(|&(_, &v)| v > 0)
            .map(|(i, &v)| (i as CardRank, v))
            .collect()
    }

    fn is_five_of_a_kind(cards: &[Card]) -> bool {
        for &card in cards.iter() {
            if card.rank != cards[0].rank {
                return false;
            }
        }
        true
    }

    fn is_straight_flush(cards: &[Card]) -> (bool, CardRank) {
        for &card in cards.iter() {
            if card.suit != cards[0].suit {
                return (false, cards[0].rank);
            }
        }
        PokerHand::is_straight(cards)
    }

    fn is_four_of_a_kind(cards: &[Card]) -> (bool, CardRank, CardRank) {
        let rank_count = PokerHand::rank_count(cards);
        if rank_count.len() != 2 {
            return (false, cards[0].rank, cards[1].rank);
        }
        let (major, kicker) = if rank_count[0].1 > rank_count[1].1 {
            (rank_count[0], rank_count[1])
        } else {
            (rank_count[1], rank_count[0])
        };
        if major.1 != 4 {
            return (false, cards[0].rank, cards[1].rank);
        }
        (true, major.0, kicker.0)
    }

    fn is_full_house(cards: &[Card]) -> (bool, CardRank, CardRank) {
        let rank_count = PokerHand::rank_count(cards);
        if rank_count.len() != 2 {
            return (false, cards[0].rank, cards[1].rank);
        }
        let (major, minor) = if rank_count[0].1 > rank_count[1].1 {
            (rank_count[0], rank_count[1])
        } else {
            (rank_count[1], rank_count[0])
        };
        if major.1 != 3 || minor.1 != 2 {
            return (false, cards[0].rank, cards[1].rank);
        }
        (true, major.0, minor.0)
    }

    fn is_flush(cards: &[Card]) -> bool {
        for &card in cards.iter() {
            if card.suit != cards[0].suit {
                return false;
            }
        }
        true
    }

    fn is_straight(cards: &[Card]) -> (bool, CardRank) {
        let mut ranks_sorted: Vec<CardRank> = cards.iter().map(|&x| x.rank).collect();
        ranks_sorted.sort();
        ranks_sorted.reverse();
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

    fn is_three_of_a_kind(cards: &[Card]) -> (bool, CardRank, CardRank, CardRank) {
        let mut rank_count = PokerHand::rank_count(cards);
        if rank_count.len() != 3 {
            return (false, cards[0].rank, cards[1].rank, cards[2].rank);
        }
        rank_count.sort_by(|&(ar, ac), &(br, bc)| {
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
        if rank_count[0].1 != 3 || rank_count[1].1 != 1 || rank_count[2].1 != 1 {
            return (false, cards[0].rank, cards[1].rank, cards[2].rank);
        }
        (true, rank_count[0].0, rank_count[1].0, rank_count[2].0)
    }

    fn is_two_pair(cards: &[Card]) -> (bool, CardRank, CardRank, CardRank) {
        let mut rank_count = PokerHand::rank_count(cards);
        if rank_count.len() != 3 {
            return (false, cards[0].rank, cards[1].rank, cards[2].rank);
        }
        rank_count.sort_by(|&(ar, ac), &(br, bc)| {
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
        if rank_count[0].1 != 2 || rank_count[1].1 != 2 || rank_count[2].1 != 1 {
            return (false, cards[0].rank, cards[1].rank, cards[2].rank);
        }
        (true, rank_count[0].0, rank_count[1].0, rank_count[2].0)
    }

    fn is_one_pair(cards: &[Card]) -> (bool, CardRank, CardRank, CardRank, CardRank) {
        let mut result = PokerHand::rank_count(cards);
        if result.len() != 4 {
            return (
                false,
                cards[0].rank,
                cards[1].rank,
                cards[2].rank,
                cards[3].rank,
            );
        }
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
        if result[0].1 != 2 || result[1].1 != 1 || result[2].1 != 1 || result[3].1 != 1 {
            return (
                false,
                cards[0].rank,
                cards[1].rank,
                cards[2].rank,
                cards[3].rank,
            );
        }
        (true, result[0].0, result[1].0, result[2].0, result[3].0)
    }
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        let a = self.cards.iter().fold(0_u8, |acc, item| acc + item.rank);
        let b = other.cards.iter().fold(0_u8, |acc, item| acc + item.rank);
        a == b
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.category_rank > other.category_rank {
            Some(Ordering::Greater)
        } else if self.category_rank < other.category_rank {
            Some(Ordering::Less)
        } else {
            // same category
            match &self.category {
                HandRankingCategory::FiveOfAKind(rank1) => match &other.category {
                    HandRankingCategory::FiveOfAKind(rank2) => {
                        if rank1 > rank2 {
                            Some(Ordering::Greater)
                        } else if rank1 < rank2 {
                            Some(Ordering::Less)
                        } else {
                            Some(Ordering::Equal)
                        }
                    }
                    _ => None,
                },
                HandRankingCategory::StraightFlush(rank1) => match &other.category {
                    HandRankingCategory::StraightFlush(rank2) => {
                        if rank1 > rank2 {
                            Some(Ordering::Greater)
                        } else if rank1 < rank2 {
                            Some(Ordering::Less)
                        } else {
                            Some(Ordering::Equal)
                        }
                    }
                    _ => None,
                },
                HandRankingCategory::FourOfAKind(major1, kicker1) => match &other.category {
                    HandRankingCategory::FourOfAKind(major2, kicker2) => {
                        if major1 > major2 {
                            Some(Ordering::Greater)
                        } else if major1 < major2 {
                            Some(Ordering::Less)
                        } else {
                            if kicker1 > kicker2 {
                                Some(Ordering::Greater)
                            } else if kicker1 < kicker2 {
                                Some(Ordering::Less)
                            } else {
                                Some(Ordering::Equal)
                            }
                        }
                    }
                    _ => None,
                },
                HandRankingCategory::FullHouse(major1, kicker1) => match &other.category {
                    HandRankingCategory::FullHouse(major2, kicker2) => {
                        if major1 > major2 {
                            Some(Ordering::Greater)
                        } else if major1 < major2 {
                            Some(Ordering::Less)
                        } else {
                            if kicker1 > kicker2 {
                                Some(Ordering::Greater)
                            } else if kicker1 < kicker2 {
                                Some(Ordering::Less)
                            } else {
                                Some(Ordering::Equal)
                            }
                        }
                    }
                    _ => None,
                },
                HandRankingCategory::Flush(ranks1) => match &other.category {
                    HandRankingCategory::Flush(ranks2) => {
                        let mut result = Some(Ordering::Equal);
                        for i in 0..5 {
                            if ranks1[i] > ranks2[i] {
                                result = Some(Ordering::Greater);
                                break;
                            } else if ranks1[i] < ranks2[i] {
                                result = Some(Ordering::Less);
                                break;
                            } else {
                                continue;
                            }
                        }
                        result
                    }
                    _ => None,
                },
                HandRankingCategory::Straight(rank1) => match &other.category {
                    HandRankingCategory::Straight(rank2) => {
                        if rank1 > rank2 {
                            Some(Ordering::Greater)
                        } else if rank1 < rank2 {
                            Some(Ordering::Less)
                        } else {
                            Some(Ordering::Equal)
                        }
                    }
                    _ => None,
                },
                HandRankingCategory::ThreeOfAKind(a1, a2, a3) => match &other.category {
                    HandRankingCategory::ThreeOfAKind(b1, b2, b3) => {
                        let a = [a1, a2, a3];
                        let b = [b1, b2, b3];
                        let mut result = Some(Ordering::Equal);
                        for i in 0..3 {
                            if a[i] > b[i] {
                                result = Some(Ordering::Greater);
                                break;
                            } else if a[i] < b[i] {
                                result = Some(Ordering::Less);
                                break;
                            } else {
                                continue;
                            }
                        }
                        result
                    }
                    _ => None,
                },
                HandRankingCategory::TwoPair(a1, a2, a3) => match &other.category {
                    HandRankingCategory::TwoPair(b1, b2, b3) => {
                        let a = [a1, a2, a3];
                        let b = [b1, b2, b3];
                        let mut result = Some(Ordering::Equal);
                        for i in 0..3 {
                            if a[i] > b[i] {
                                result = Some(Ordering::Greater);
                                break;
                            } else if a[i] < b[i] {
                                result = Some(Ordering::Less);
                                break;
                            } else {
                                continue;
                            }
                        }
                        result
                    }
                    _ => None,
                },
                HandRankingCategory::OnePair(a1, a2, a3, a4) => match &other.category {
                    HandRankingCategory::OnePair(b1, b2, b3, b4) => {
                        let a = [a1, a2, a3, a4];
                        let b = [b1, b2, b3, b4];
                        let mut result = Some(Ordering::Equal);
                        for i in 0..4 {
                            if a[i] > b[i] {
                                result = Some(Ordering::Greater);
                                break;
                            } else if a[i] < b[i] {
                                result = Some(Ordering::Less);
                                break;
                            } else {
                                continue;
                            }
                        }
                        result
                    }
                    _ => None,
                },
                HandRankingCategory::HighCard(ranks1) => match &other.category {
                    HandRankingCategory::HighCard(ranks2) => {
                        let mut result = Some(Ordering::Equal);
                        for i in 0..5 {
                            if ranks1[i] > ranks2[i] {
                                result = Some(Ordering::Greater);
                                break;
                            } else if ranks1[i] < ranks2[i] {
                                result = Some(Ordering::Less);
                                break;
                            } else {
                                continue;
                            }
                        }
                        result
                    }
                    _ => None,
                },
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Card {
    rank: u8,
    suit: u8,
}

type CardRank = u8;
type CardSuit = u8;

impl Card {
    const R: [(&'static str, CardRank); 13] = [
        ("A", 14),
        ("K", 13),
        ("Q", 12),
        ("J", 11),
        ("10", 10),
        ("9", 9),
        ("8", 8),
        ("7", 7),
        ("6", 6),
        ("5", 5),
        ("4", 4),
        ("3", 3),
        ("2", 2),
    ];

    const S: [(&'static str, CardSuit); 4] = [("C", 0), ("D", 1), ("H", 2), ("S", 3)];

    fn new() -> Self {
        Card { rank: 2, suit: 0 }
    }

    fn from(card_str: &str) -> Self {
        let mut rank_map: HashMap<String, CardRank> = HashMap::new();
        for r in Card::R {
            rank_map.insert(r.0.to_string(), r.1);
        }
        let mut suit_map: HashMap<String, CardSuit> = HashMap::new();
        for s in Card::S {
            suit_map.insert(s.0.to_string(), s.1);
        }
        match Card::read(card_str) {
            Some((r, s)) => Card {
                rank: *rank_map.get(&r).expect("invalid card rank"),
                suit: *suit_map.get(&s).expect("invalid card suit"),
            },
            None => panic!("invalid card string"),
        }
    }

    fn read(card_str: &str) -> Option<(String, String)> {
        let chars: Vec<char> = card_str.chars().collect();
        if chars.len() == 3 {
            match chars[..] {
                ['1', '0', c] if Card::is_valid_card_suit(c) => {
                    return Some((String::from("10"), c.to_string()));
                }
                _ => return None,
            }
        } else if chars.len() == 2 {
            match chars[..] {
                [a, b] if Card::is_valid_card_rank(a) && Card::is_valid_card_suit(b) => {
                    return Some((a.to_string(), b.to_string()));
                }
                _ => return None,
            }
        } else {
            return None;
        }
    }

    fn is_valid_card_suit(s: char) -> bool {
        if s == 'C' || s == 'D' || s == 'H' || s == 'S' {
            true
        } else {
            false
        }
    }

    fn is_valid_card_rank(s: char) -> bool {
        let lst = ['2', '3', '4', '5', '6', '7', '8', '9', 'J', 'Q', 'K', 'A'];
        for c in lst {
            if s == c {
                return true;
            }
        }
        false
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rankstr = String::new();
        let mut suitstr = String::new();
        for (rs, rv) in Card::R {
            if self.rank == rv {
                rankstr = String::from(rs);
            }
        }
        for (ss, sv) in Card::S {
            if self.suit == sv {
                suitstr = String::from(ss);
            }
        }
        write!(f, "{}{}", rankstr, suitstr)
    }
}

#[test]
fn test_hand_order() {
    let h1 = PokerHand::new("5H 5S 5D 9S 9D");
    let h2 = PokerHand::new("5H 9S 5S 4S 9D");
    // assert!(h1 == h2);
    assert!(h1 > h2);
}

#[test]
fn test_categories() {
    let h1 = PokerHand::new("AS AC AH AD AH");
    assert!(matches!(h1.category, HandRankingCategory::FiveOfAKind(..)));

    let h2 = PokerHand::new("JC 10C 9C 7C 8C");
    assert!(matches!(
        h2.category,
        HandRankingCategory::StraightFlush(..)
    ));

    let h3 = PokerHand::new("5C 5D 5H 5S 2D");
    assert!(matches!(h3.category, HandRankingCategory::FourOfAKind(..)));

    let h4 = PokerHand::new("6S 6H 6D KC KH");
    assert!(matches!(h4.category, HandRankingCategory::FullHouse(..)));

    let h5 = PokerHand::new("JD 9D 8D 4D 3D");
    assert!(matches!(h5.category, HandRankingCategory::Flush(..)));

    let h6 = PokerHand::new("10D 9S 8H 7D 6C");
    assert!(matches!(h6.category, HandRankingCategory::Straight(..)));

    let h7 = PokerHand::new("QC QS QH 9H 2S");
    assert!(matches!(h7.category, HandRankingCategory::ThreeOfAKind(..)));

    let h8 = PokerHand::new("JH JS 3C 3S 2H");
    assert!(matches!(h8.category, HandRankingCategory::TwoPair(..)));

    let h9 = PokerHand::new("10S 10H 8S 7H 4C");
    assert!(matches!(h9.category, HandRankingCategory::OnePair(..)));

    let h10 = PokerHand::new("KD QD 7S 4S 3H");
    assert!(matches!(h10.category, HandRankingCategory::HighCard(..)));

    let h11 = PokerHand::new("10D JH QS KD AC");
    assert!(matches!(h11.category, HandRankingCategory::Straight(..)));

    let h12 = PokerHand::new("4D AH 3S 2D 5C");
    assert!(matches!(h12.category, HandRankingCategory::Straight(..)));
}
