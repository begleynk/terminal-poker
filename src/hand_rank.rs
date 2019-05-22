use crate::lookup_tables;
use crate::hand::Hand;

pub(crate) type NumericHandRank = u32;

#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(dead_code)]
pub(crate) enum HandRank {
    HighCard(NumericHandRank),
    OnePair(NumericHandRank),
    TwoPair(NumericHandRank),
    ThreeOfAKind(NumericHandRank),
    Straight(NumericHandRank),
    Flush(NumericHandRank),
    FullHouse(NumericHandRank),
    FourOfAKind(NumericHandRank),
    StraightFlush(NumericHandRank),
}

impl HandRank {
    pub(crate) fn compute(hand: &Hand) -> Self {
        let card0 = hand.cards()[0].as_int();
        let card1 = hand.cards()[1].as_int();
        let card2 = hand.cards()[2].as_int();
        let card3 = hand.cards()[3].as_int();
        let card4 = hand.cards()[4].as_int();

        let lookup_index = (card0 | card1 | card2 | card3 | card4) >> 16;

        if Self::all_same_suit(&hand) {
            (lookup_tables::FLUSHES[lookup_index as usize] as NumericHandRank).into()
        } else {
            let rank = lookup_tables::UNIQUES[lookup_index as usize] as NumericHandRank;

            if rank != 0 {
                rank.into()
            } else {
                let q = (hand.cards()[0].as_int() & 0xFF)
                    * (hand.cards()[1].as_int() & 0xFF)
                    * (hand.cards()[2].as_int() & 0xFF)
                    * (hand.cards()[3].as_int() & 0xFF)
                    * (hand.cards()[4].as_int() & 0xFF);

                (lookup_tables::VALUES[Self::find_value_index(q)] as NumericHandRank).into()
            }
        }
    }

    fn all_same_suit(hand: &Hand) -> bool {
        let card0 = hand.cards()[0].as_int();
        let card1 = hand.cards()[1].as_int();
        let card2 = hand.cards()[2].as_int();
        let card3 = hand.cards()[3].as_int();
        let card4 = hand.cards()[4].as_int();

        card0 & card1 & card2 & card3 & card4 & 0xf000 != 0
    }

    fn find_value_index(q: u32) -> usize {
        let mut low: usize = 0;
        let mut high: usize = 4888;
        let mut mid;

        while low <= high {
            mid = (high + low) >> 1; // Divide by two
            if q < lookup_tables::PRODUCTS[mid] {
                high = mid - 1;
            } else if q > lookup_tables::PRODUCTS[mid] {
                low = mid + 1;
            } else {
                return mid as usize;
            }
        }
        panic!("No match found for key {}", q);
    }

    pub(crate) fn numeric(&self) -> NumericHandRank {
        match *self {
            HandRank::HighCard(i) => i,
            HandRank::OnePair(i) => i,
            HandRank::TwoPair(i) => i,
            HandRank::ThreeOfAKind(i) => i,
            HandRank::Straight(i) => i,
            HandRank::Flush(i) => i,
            HandRank::FullHouse(i) => i,
            HandRank::FourOfAKind(i) => i,
            HandRank::StraightFlush(i) => i,
        }
    }
}

impl From<NumericHandRank> for HandRank {
    fn from(i: NumericHandRank) -> Self {
        if i > 6185 {
            HandRank::HighCard(i)
        } else if i > 3325 {
            HandRank::OnePair(i)
        } else if i > 2467 {
            HandRank::TwoPair(i)
        } else if i > 1609 {
            HandRank::ThreeOfAKind(i)
        } else if i > 1599 {
            HandRank::Straight(i)
        } else if i > 322 {
            HandRank::Flush(i)
        } else if i > 166 {
            HandRank::FullHouse(i)
        } else if i > 10 {
            HandRank::FourOfAKind(i)
        } else {
            HandRank::StraightFlush(i)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use crate::card::{Card, Suit, Rank};

    #[test]
    fn calculates_value_of_a_hand() {
        let hand = Hand::new([
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
        ]);

        assert_eq!(hand.rank(), HandRank::StraightFlush(9));

        let hand = Hand::new([
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Hearts),
        ]);

        assert_eq!(hand.rank(), HandRank::Straight(1601));

        let hand = Hand::new([
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Hearts),
        ]);

        assert_eq!(hand.rank(), HandRank::FourOfAKind(166));
    }

    #[test]
    fn can_rank_hands() {
        let straight_flush = Hand::new([
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
        ]);

        let straight = Hand::new([
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Hearts),
        ]);

        let one_pair = Hand::new([
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
        ]);

        assert!(straight_flush > straight);
        assert!(straight > one_pair);
        assert!(straight_flush > one_pair);
    }

    #[test]
    fn can_detect_identical_value_hands() {
        let hand1 = Hand::new([
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
        ]);

        let hand2 = Hand::new([
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Diamonds),
        ]);

        assert!(hand1 == hand2);
    }
}