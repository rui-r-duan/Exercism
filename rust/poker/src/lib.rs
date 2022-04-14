use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    if hands.len() == 0 {
        vec![]
    } else {
        let mut hands_max_heap = hands
            .iter()
            .map(|&h| PokerHand::new(h))
            .collect::<BinaryHeap<PokerHand>>();
        let winner = hands_max_heap.pop().unwrap();
        let mut result = vec![winner.card_str_ref];
        while let Some(hand) = hands_max_heap.pop() {
            if hand < winner {
                break;
            }
            result.push(hand.card_str_ref);
        }
        result
    }
}

// Eq and Ord are not needed in this program,
// but this enum has a total order in nature,
// so we derive Eq and Ord.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandCategory {
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

type CardRank = u32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HandRank {
    category_rank: HandCategory,
    card_ranks_sorted: Vec<CardRank>, // sorted by (card_rank_count, card_rank)
}

#[derive(Debug)]
struct PokerHand<'a> {
    card_str_ref: &'a str,
    rank: HandRank,
}

impl<'a> PokerHand<'a> {
    fn new(hand_str: &'a str) -> Self {
        PokerHand {
            card_str_ref: hand_str,
            rank: calc_rank(hand_str),
        }
    }
}

fn calc_rank(hand_str: &str) -> HandRank {
    let mut suits = vec![];
    let mut ranks = vec![];
    for c in hand_str.split(' ') {
        let n = c.len();
        let rank = match &c[..n - 1] {
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            m => m.parse().unwrap(),
        };
        let suit = c.chars().nth(n - 1).unwrap();
        suits.push(suit);
        ranks.push(rank);
    }
    let rc = rank_count_sorted(&ranks);
    let mut card_ranks_sorted = vec![];
    for &(v, c) in rc.iter() {
        for _ in 0..c {
            card_ranks_sorted.push(v);
        }
    }
    let is_lowest_ace = match card_ranks_sorted.as_slice() {
        [14, 5, 4, 3, 2] => true,
        _ => false,
    };
    let category_rank = match rc.as_slice() {
        [(_, 4), (_, 1)] => HandCategory::FourOfAKind,
        [(_, 3), (_, 2)] => HandCategory::FullHouse,
        [(_, 3), (_, 1), (_, 1)] => HandCategory::ThreeOfAKind,
        [(_, 2), (_, 2), (_, 1)] => HandCategory::TwoPair,
        [(_, 2), (_, 1), (_, 1), (_, 1)] => HandCategory::OnePair,
        _ => {
            // [(_, 1), (_, 1), (_, 1), (_, 1), (_, 1)]
            let is_same_suit = suits.windows(2).all(|s| s[0] == s[1]);
            let is_straight =
                // is_lowest_ace || card_ranks_sorted.windows(2).all(|w| w[0] - 1 == w[1]);
                is_lowest_ace || card_ranks_sorted[4] == card_ranks_sorted[0] - 4;
            match (is_same_suit, is_straight) {
                (true, true) => HandCategory::StraightFlush,
                (true, false) => HandCategory::Flush,
                (false, true) => HandCategory::Straight,
                _ => HandCategory::HighCard,
            }
        }
    };

    HandRank {
        category_rank,
        card_ranks_sorted: if is_lowest_ace {
            vec![5, 4, 3, 2, 1]
        } else {
            card_ranks_sorted
        },
    }
}

/// Returns rank counts sorted in descending order.
fn rank_count_sorted(ranks: &[CardRank]) -> Vec<(CardRank, u8)> {
    let mut rc: [u8; 15] = [0; 15];
    for rank in ranks {
        // equivalent to ranks.into_iter()
        rc[*rank as usize] += 1;
    }
    let mut result: Vec<(CardRank, u8)> = rc
        .iter()
        .enumerate()
        .filter(|&(_, &v)| v > 0)
        .map(|(i, &v)| (i as CardRank, v))
        .collect();
    result.sort_by(|&(ar, ac), &(br, bc)| bc.cmp(&ac).then(br.cmp(&ar)));
    result
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &Self) -> bool {
        PokerHand::partial_cmp(self, other) == Some(Ordering::Equal)
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl<'a> Eq for PokerHand<'a> {}

// Ord is needed for `collect::<BinaryHeap<PokerHand>>()`
//
// fn collect<B>(self) -> B
// where
//     B: FromIterator<Self::Item>,
//
//
// impl<T> FromIterator<T> for BinaryHeap<T>
// where
//     T: Ord,
impl<'a> Ord for PokerHand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[cfg(test)]
mod tests {
    use super::{HandCategory, PokerHand};

    #[test]
    fn test_hand_eq() {
        let h1 = PokerHand::new("5H 5S 5D 9S 9D");
        let h2 = PokerHand::new("5H 9S 5S 5S 9D");
        assert!(h1 == h2);
    }

    #[test]
    fn test_hand_order_2() {
        let h1 = PokerHand::new("3H 3D 4H 4D 5S"); // 4-high two-pair, sum = 19
        let h2 = PokerHand::new("2H 3D 5H 5D 4S"); // 5-high one-pair, sum = 19

        // assert_eq requires Debug trait
        assert_eq!(h1.rank.category_rank, HandCategory::TwoPair);
        assert_eq!(h2.rank.category_rank, HandCategory::OnePair);
        assert!(h1 != h2);
        assert!(h1 > h2);
        assert!(h1 >= h2);
    }

    #[test]
    fn test_hand_order() {
        let h1 = PokerHand::new("5H 5S 5D 9S 9D");
        let h2 = PokerHand::new("5H 9S 5S 4S 9D"); // two-pair
        assert_eq!(h1.rank.category_rank, HandCategory::FullHouse);
        assert_eq!(h2.rank.category_rank, HandCategory::TwoPair);
        assert!(h1 > h2);
    }

    #[test]
    fn test_categories() {
        let h2 = PokerHand::new("JC 10C 9C 7C 8C");
        assert_eq!(h2.rank.category_rank, HandCategory::StraightFlush);

        let h3 = PokerHand::new("5C 5D 5H 5S 2D");
        assert_eq!(h3.rank.category_rank, HandCategory::FourOfAKind);

        let h4 = PokerHand::new("6S 6H 6D KC KH");
        assert_eq!(h4.rank.category_rank, HandCategory::FullHouse);

        let h5 = PokerHand::new("JD 9D 8D 4D 3D");
        assert_eq!(h5.rank.category_rank, HandCategory::Flush);

        let h6 = PokerHand::new("10D 9S 8H 7D 6C");
        assert_eq!(h6.rank.category_rank, HandCategory::Straight);

        let h7 = PokerHand::new("QC QS QH 9H 2S");
        assert_eq!(h7.rank.category_rank, HandCategory::ThreeOfAKind);

        let h8 = PokerHand::new("JH JS 3C 3S 2H");
        assert_eq!(h8.rank.category_rank, HandCategory::TwoPair);

        let h9 = PokerHand::new("10S 10H 8S 7H 4C");
        assert_eq!(h9.rank.category_rank, HandCategory::OnePair);

        let h10 = PokerHand::new("KD QD 7S 4S 3H");
        assert_eq!(h10.rank.category_rank, HandCategory::HighCard);

        let h11 = PokerHand::new("10D JH QS KD AC");
        assert_eq!(h11.rank.category_rank, HandCategory::Straight);

        let h12 = PokerHand::new("4D AH 3S 2D 5C");
        assert_eq!(h12.rank.category_rank, HandCategory::Straight);
    }
}
