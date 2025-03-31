use serde::{ Serialize, Deserialize };
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Serialize, Deserialize)]
/// Suit class, representing the suit of a Card (shape + colour)
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds
}

impl Suit {
    /// true if Suit is Clubs or Spades
    pub fn is_black(&self) -> bool {
        let blacks = vec![
            Suit::Clubs,
            Suit::Spades
        ];
        return blacks.contains(self);
    }

    /// true if Suit is Hearts or Diamonds
    pub fn is_red(&self) -> bool {
        let reds = vec![
            Suit::Hearts,
            Suit::Diamonds
        ];
        return reds.contains(self);
    }
}

impl PartialEq for Suit {
    fn eq(&self, other: &Self) -> bool {
        return core::mem::discriminant(self) == core::mem::discriminant(other);
    }
}

impl Clone for Suit {
    fn clone(&self) -> Self {
        match self {
            Self::Clubs => Self::Clubs,
            Self::Spades => Self::Spades,
            Self::Hearts => Self::Hearts,
            Self::Diamonds => Self::Diamonds,
        }
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Spades => write!(f, "Spades"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Diamonds => write!(f, "Diamonds"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_equal() {
        let clubs = Suit::Clubs;
        let spades = Suit::Spades;
        let clubs_2 = Suit::Clubs;
        assert_eq!(clubs, clubs_2);
        assert_ne!(clubs, spades);
        let diamonds = Suit::Diamonds;
        assert_ne!(clubs, diamonds);
        assert_ne!(spades, diamonds);
    }
}
