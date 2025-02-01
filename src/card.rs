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
}
