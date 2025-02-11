use serde::{ Deserialize, Serialize };

use std::cmp::Ordering;

mod rank;
pub use rank::Rank;
mod suit;
pub use suit::Suit;

/// Card class, containing a rank and a suit.
/// Create a new card with Card::new(),
/// Example:
/// ```
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// ```
#[derive(Serialize, Deserialize, Debug)]
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

    /// true if Card's Rank is a number (not including Ace)
    pub fn is_number(&self) -> bool {
        return self.rank.is_number();
    }

    /// true if Card's Rank is a face (Jack, Queen or King)
    pub fn is_face(&self) -> bool {
        return self.rank.is_face();
    }

    /// true if Card's Suit is Clubs or Spades
    pub fn is_black(&self) -> bool {
        return self.suit.is_black();
    }

    /// true if Card's Suit is Hearts or Diamonds
    pub fn is_red(&self) -> bool {
        return self.suit.is_red();
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        return self.rank == other.rank && self.suit == other.suit;
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

// converted rank to number because of rank iterator error....
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().to_u8().cmp(&other.rank().to_u8())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_constructor() {
        let card = Card::new(Rank::Ace, Suit::Clubs);
        assert_eq!(*card.rank(), Rank::Ace);
        assert_eq!(*card.suit(), Suit::Clubs);
        assert_ne!(*card.rank(), Rank::King);
        assert_ne!(*card.suit(), Suit::Diamonds);
    }

    #[test]
    fn is_number() {
        let card = Card::new(Rank::Queen, Suit::Clubs);
        assert!(!card.is_number());
        let card = Card::new(Rank::Two, Suit::Clubs);
        assert!(card.is_number());
        let card = Card::new(Rank::Ace, Suit::Diamonds);
        assert!(card.is_number());
        let card = Card::new(Rank::Ten, Suit::Spades);
        assert!(card.is_number());
        let card = Card::new(Rank::Jack, Suit::Hearts);
        assert!(!card.is_number());
    }

    #[test]
    fn is_face() {
        let card = Card::new(Rank::Queen, Suit::Clubs);
        assert!(card.is_face());
        let card = Card::new(Rank::Two, Suit::Clubs);
        assert!(!card.is_face());
        let card = Card::new(Rank::Ace, Suit::Diamonds);
        assert!(!card.is_face());
        let card = Card::new(Rank::Ten, Suit::Spades);
        assert!(!card.is_face());
        let card = Card::new(Rank::Jack, Suit::Hearts);
        assert!(card.is_face());
    }

    #[test]
    fn is_equal() {
        let ace_of_clubs = Card::new(Rank::Ace, Suit::Clubs);
        let ace_of_clubs_2 = Card::new(Rank::Ace, Suit::Clubs);
        assert_eq!(ace_of_clubs, ace_of_clubs_2);
        let ace_of_spades = Card::new(Rank::Ace, Suit::Spades);
        assert_ne!(ace_of_clubs, ace_of_spades);
        let two_of_clubs = Card::new(Rank::Two, Suit::Clubs);
        assert_ne!(ace_of_clubs, two_of_clubs);
    }
}
