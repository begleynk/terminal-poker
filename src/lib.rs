mod lookup_tables;

#[derive(Eq, Debug, Copy, Clone)]
struct Hand([Card; 5]);

impl Hand {
    /// Implementation of the scheme described in
    /// http://suffe.cool/poker/evaluator.html
    fn rank(&self) -> HandRank {
        let card0 = self.0[0].as_int();
        let card1 = self.0[1].as_int();
        let card2 = self.0[2].as_int();
        let card3 = self.0[3].as_int();
        let card4 = self.0[4].as_int();

        let lookup_index = (card0 | card1 | card2 | card3 | card4) >> 16;

        if self.all_same_suit() {
            (lookup_tables::FLUSHES[lookup_index as usize] as NumericHandRank).into()
        } else {
            let rank = lookup_tables::UNIQUES[lookup_index as usize] as NumericHandRank;

            if rank != 0 {
                rank.into()
            } else {
                let q = (self.0[0].as_int() & 0xFF)
                    * (self.0[1].as_int() & 0xFF)
                    * (self.0[2].as_int() & 0xFF)
                    * (self.0[3].as_int() & 0xFF)
                    * (self.0[4].as_int() & 0xFF);

                (lookup_tables::VALUES[Self::find_value_index(q)] as NumericHandRank).into()
            }
        }
    }

    fn all_same_suit(&self) -> bool {
        let card0 = self.0[0].as_int();
        let card1 = self.0[1].as_int();
        let card2 = self.0[2].as_int();
        let card3 = self.0[3].as_int();
        let card4 = self.0[4].as_int();

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
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        let ordering = self.rank().numeric().cmp(&other.rank().numeric());

        ordering
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        // NOTE(Nik): Reverse here since smaller NumericHandRanks are greater value
        Some(self.cmp(other).reverse())
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.rank().numeric() == other.rank().numeric()
    }
}

type NumericHandRank = u32;

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

#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(dead_code)]
enum HandRank {
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
    fn numeric(&self) -> NumericHandRank {
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

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Rank {
    fn prime_encoding(&self) -> u32 {
        match *self {
            Rank::Ace => 41,
            Rank::King => 37,
            Rank::Queen => 31,
            Rank::Jack => 29,
            Rank::Ten => 23,
            Rank::Nine => 19,
            Rank::Eight => 17,
            Rank::Seven => 13,
            Rank::Six => 11,
            Rank::Five => 7,
            Rank::Four => 5,
            Rank::Three => 3,
            Rank::Two => 2,
        }
    }

    fn order_encoding(&self) -> u32 {
        match *self {
            Rank::Ace => 12,
            Rank::King => 11,
            Rank::Queen => 10,
            Rank::Jack => 9,
            Rank::Ten => 8,
            Rank::Nine => 7,
            Rank::Eight => 6,
            Rank::Seven => 5,
            Rank::Six => 4,
            Rank::Five => 3,
            Rank::Four => 2,
            Rank::Three => 1,
            Rank::Two => 0,
        }
    }
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Rank::Ace => write!(f, "A"),
            Rank::King => write!(f, "K"),
            Rank::Queen => write!(f, "Q"),
            Rank::Jack => write!(f, "J"),
            Rank::Ten => write!(f, "10"),
            Rank::Nine => write!(f, "9"),
            Rank::Eight => write!(f, "8"),
            Rank::Seven => write!(f, "7"),
            Rank::Six => write!(f, "6"),
            Rank::Five => write!(f, "5"),
            Rank::Four => write!(f, "4"),
            Rank::Three => write!(f, "3"),
            Rank::Two => write!(f, "2"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl std::fmt::Debug for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Suit::Hearts => write!(f, "♥"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Spades => write!(f, "♠"),
            Suit::Diamonds => write!(f, "♦"),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Card(u32);

impl Card {
    /// Packs a rank and suite pair into a packed bit representation
    ///
    /// +--------+--------+--------+--------+
    /// |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
    /// +--------+--------+--------+--------+
    ///
    /// p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
    /// r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
    /// cdhs = suit of card (bit turned on based on suit of card)
    /// b = bit turned on depending on rank of card
    ///
    /// xxxAKQJT 98765432 CDHSrrrr xxPPPPPP
    /// 00001000 00000000 01001011 00100101    King of Diamonds
    /// 00000000 00001000 00010011 00000111    Five of Spades
    /// 00000010 00000000 10001001 00011101    Jack of Clubs
    ///
    fn new(rank: Rank, suit: Suit) -> Card {
        let prime = rank.prime_encoding();
        let order = rank.order_encoding();
        let base = 1 << (16 + order);

        let suit_bits = match suit {
            Suit::Spades => 0x1000,
            Suit::Hearts => 0x2000,
            Suit::Diamonds => 0x4000,
            Suit::Clubs => 0x8000,
        };

        Card(base | order << 8 | suit_bits | prime)
    }

    fn as_int(&self) -> u32 {
        self.0
    }

    fn suit(&self) -> Suit {
        // Apply a mask to extract the suite
        let bits = self.0 & (0b00000000_00000000_11110000_00000000 as u32);

        match bits {
            0x1000 => Suit::Spades,
            0x2000 => Suit::Hearts,
            0x4000 => Suit::Diamonds,
            0x8000 => Suit::Clubs,
            _ => unreachable!("Bad suite encoding"),
        }
    }

    fn rank(&self) -> Rank {
        // Apply a mask and shift to extract the rank
        let bits = (self.0 & (0b00000000_00000000_00001111_00000000 as u32)) >> 8;

        match bits {
            0 => Rank::Two,
            1 => Rank::Three,
            2 => Rank::Four,
            3 => Rank::Five,
            4 => Rank::Six,
            5 => Rank::Seven,
            6 => Rank::Eight,
            7 => Rank::Nine,
            8 => Rank::Ten,
            9 => Rank::Jack,
            10 => Rank::Queen,
            11 => Rank::King,
            12 => Rank::Ace,
            _ => unreachable!("Bad rank encoding"),
        }
    }
}

impl From<u32> for Card {
    fn from(i: u32) -> Self {
        Card(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_cards() {
        assert_eq!(
            Card::new(Rank::King, Suit::Diamonds),
            (0b00001000_00000000_01001011_00100101 as u32).into(),
            "Should be a King of Diamonds: "
        );
        assert_eq!(
            Card::new(Rank::Five, Suit::Spades),
            (0b00000000_00001000_00010011_00000111 as u32).into(),
            "Should be a Five of Spades"
        );
        assert_eq!(
            Card::new(Rank::Jack, Suit::Clubs),
            (0b00000010_00000000_10001001_00011101 as u32).into(),
            "Should be a Jack of Clubs"
        );
    }

    #[test]
    fn gets_the_rank_and_suit_of_a_card() {
        let card = Card::new(Rank::Ace, Suit::Clubs);

        assert_eq!(card.rank(), Rank::Ace);
        assert_eq!(card.suit(), Suit::Clubs);

        let card = Card::new(Rank::Three, Suit::Spades);

        assert_eq!(card.rank(), Rank::Three);
        assert_eq!(card.suit(), Suit::Spades);
    }

    #[test]
    fn calculates_value_of_a_hand() {
        let hand = Hand([
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
        ]);

        assert_eq!(hand.rank(), HandRank::StraightFlush(9));

        let hand = Hand([
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Hearts),
        ]);

        assert_eq!(hand.rank(), HandRank::Straight(1601));

        let hand = Hand([
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
        let straight_flush = Hand([
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
        ]);

        let straight = Hand([
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Clubs),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Hearts),
        ]);

        let one_pair = Hand([
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
        let hand1 = Hand([
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
        ]);

        let hand2 = Hand([
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Diamonds),
        ]);

        assert!(hand1 == hand2);
    }
}
