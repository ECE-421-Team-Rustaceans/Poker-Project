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
    suit: Suit,
    is_face_up: bool
}

impl Card {
    /// Constructor for Card.
    /// Example:
    /// ```
    /// let card = Card::new(Rank::Ace, Suit::Spades);
    /// ```
    pub fn new(rank: Rank, suit: Suit, is_face_up: bool) -> Card {
        let card = Card {
            rank,
            suit,
            is_face_up
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

    /// true if Card is face up
    pub fn is_face_up(&self) -> bool {
        return self.is_face_up;
    }

    /// allow the dealer to flip cards over as necessary
    pub fn set_face_up(&mut self, is_face_up: bool) {
        self.is_face_up = is_face_up;
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

impl Clone for Card {
    fn clone(&self) -> Self {
        Self { rank: self.rank.clone(), suit: self.suit.clone(), is_face_up: self.is_face_up.clone() }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let face_up_string = match self.is_face_up {
            true => "face up",
            false => "face down",
        };
        write!(f, "{} of {} ({})", self.rank, self.suit, face_up_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_constructor() {
        let card = Card::new(Rank::Ace, Suit::Clubs, false);
        assert_eq!(*card.rank(), Rank::Ace);
        assert_eq!(*card.suit(), Suit::Clubs);
        assert_ne!(*card.rank(), Rank::King);
        assert_ne!(*card.suit(), Suit::Diamonds);
    }

    #[test]
    fn is_number() {
        let card = Card::new(Rank::Queen, Suit::Clubs, false);
        assert!(!card.is_number());
        let card = Card::new(Rank::Two, Suit::Clubs, false);
        assert!(card.is_number());
        let card = Card::new(Rank::Ace, Suit::Diamonds, false);
        assert!(card.is_number());
        let card = Card::new(Rank::Ten, Suit::Spades, false);
        assert!(card.is_number());
        let card = Card::new(Rank::Jack, Suit::Hearts, false);
        assert!(!card.is_number());
    }

    #[test]
    fn is_face() {
        let card = Card::new(Rank::Queen, Suit::Clubs, false);
        assert!(card.is_face());
        let card = Card::new(Rank::Two, Suit::Clubs, false);
        assert!(!card.is_face());
        let card = Card::new(Rank::Ace, Suit::Diamonds, false);
        assert!(!card.is_face());
        let card = Card::new(Rank::Ten, Suit::Spades, false);
        assert!(!card.is_face());
        let card = Card::new(Rank::Jack, Suit::Hearts, false);
        assert!(card.is_face());
    }

    #[test]
    fn is_equal() {
        let ace_of_clubs = Card::new(Rank::Ace, Suit::Clubs, false);
        let ace_of_clubs_2 = Card::new(Rank::Ace, Suit::Clubs, false);
        assert_eq!(ace_of_clubs, ace_of_clubs_2);
        let ace_of_spades = Card::new(Rank::Ace, Suit::Spades, false);
        assert_ne!(ace_of_clubs, ace_of_spades);
        let two_of_clubs = Card::new(Rank::Two, Suit::Clubs, false);
        assert_ne!(ace_of_clubs, two_of_clubs);
    }
}
