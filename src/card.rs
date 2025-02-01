mod rank;
pub use rank::Rank;
mod suit;
pub use suit::Suit;

#[derive(Debug)]
/// Card class, containing a rank and a suit.
/// Create a new card with Card::new(),
/// Example:
/// ```
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// ```
pub struct Card {
    rank: Rank,
    suit: Suit
}

impl Card {
    /// Constructor for Card.
    /// Example:
    /// ```
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// ```
    pub fn new(rank: Rank, suit: Suit) -> Card {
        let card = Card {
            rank,
            suit
        };
        return card;
    }

    /// Get the Rank of this Card
    pub fn rank(&self) -> &Rank {
        return &self.rank;
    }

    /// Get the Suit of this Card
    pub fn suit(&self) -> &Suit {
        return &self.suit;
    }

    pub fn is_number(&self) -> bool {
        return self.rank.is_number();
    }
}
