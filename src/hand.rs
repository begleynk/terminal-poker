use crate::card::Card;
use crate::hand_rank::HandRank;

#[derive(Eq, Debug, Copy, Clone)]
pub(crate) struct Hand([Card; 5]);

impl Hand {
    pub(crate) fn new(cards: [Card; 5]) -> Self {
        Hand(cards)
    }

    /// Implementation of the scheme described in
    /// http://suffe.cool/poker/evaluator.html
    pub(crate) fn rank(&self) -> HandRank {
        HandRank::compute(self)
    }

    pub(crate) fn cards(&self) -> &[Card; 5] {
        &self.0
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        self.rank().numeric().cmp(&other.rank().numeric())
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
