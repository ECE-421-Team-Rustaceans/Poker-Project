mod rank;
pub use rank::Rank;
mod suit;
pub use suit::Suit;

#[derive(Debug)]
pub struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        let card = Card {
            rank,
            suit
        };
        return card;
    }

    pub fn rank(&self) -> &Rank {
        return &self.rank;
    }

    pub fn suit(&self) -> &Suit {
        return &self.suit;
    }
}
