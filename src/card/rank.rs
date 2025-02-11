use serde::{ Serialize, Deserialize };

use std::cmp::Ordering;

use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, EnumIter, Serialize, Deserialize)]
/// Rank class, representing the rank of a Card (the number / face)
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Rank {
    /// true if Rank is a number (not including Ace)
    pub fn is_number(&self) -> bool {
        let numbers = vec![
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten
        ];
        return numbers.contains(self);
    }

    /// true if Rank is a face (Jack, Queen or King)
    pub fn is_face(&self) -> bool {
        let faces = vec![
            Rank::Jack,
            Rank::Queen,
            Rank::King
        ];
        return faces.contains(self);
    }

    // convert ranks to numbers for easy comparing
    pub fn to_u8(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }

    pub fn to_rank(value: u8) -> Rank {
        match value {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => panic!("invalid card rank {}", value),
        }
    }


}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        return core::mem::discriminant(self) == core::mem::discriminant(other);
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        else if (
            *self == Rank::Ace
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
                || *other == Rank::Five
                || *other == Rank::Six
                || *other == Rank::Seven
                || *other == Rank::Eight
                || *other == Rank::Nine
                || *other == Rank::Ten
                || *other == Rank::Jack
                || *other == Rank::Queen
                || *other == Rank::King
            )
        ) || (
            *self == Rank::King
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
                || *other == Rank::Five
                || *other == Rank::Six
                || *other == Rank::Seven
                || *other == Rank::Eight
                || *other == Rank::Nine
                || *other == Rank::Ten
                || *other == Rank::Jack
                || *other == Rank::Queen
            )
        ) || (
            *self == Rank::Queen
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
                || *other == Rank::Five
                || *other == Rank::Six
                || *other == Rank::Seven
                || *other == Rank::Eight
                || *other == Rank::Nine
                || *other == Rank::Ten
                || *other == Rank::Jack
            )
        ) || (
            *self == Rank::Jack
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
                || *other == Rank::Five
                || *other == Rank::Six
                || *other == Rank::Seven
                || *other == Rank::Eight
                || *other == Rank::Nine
                || *other == Rank::Ten
            )
        ) || (
            *self == Rank::Ten
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
                || *other == Rank::Five
                || *other == Rank::Six
                || *other == Rank::Seven
                || *other == Rank::Eight
                || *other == Rank::Nine
            )
        ) || (
            *self == Rank::Nine
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
                || *other == Rank::Five
                || *other == Rank::Six
                || *other == Rank::Seven
                || *other == Rank::Eight
            )
        ) || (
            *self == Rank::Eight
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
                || *other == Rank::Five
                || *other == Rank::Six
                || *other == Rank::Seven
            )
        ) || (
            *self == Rank::Seven
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
                || *other == Rank::Five
                || *other == Rank::Six
            )
        ) || (
            *self == Rank::Six
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
                || *other == Rank::Five
            )
        ) || (
            *self == Rank::Five
            && (
                *other == Rank::Two
                || *other == Rank::Three
                || *other == Rank::Four
            )
        ) || (
            *self == Rank::Four
            && (
                *other == Rank::Two
                || *other == Rank::Three
            )
        ) || (
            *self == Rank::Three
            && (
                *other == Rank::Two
            )
        ) {
            return Some(std::cmp::Ordering::Greater);
        }
        else {
            return Some(std::cmp::Ordering::Less);
        }
    }
}

impl Clone for Rank {
    fn clone(&self) -> Self {
        match self {
            Self::Ace => Self::Ace,
            Self::Two => Self::Two,
            Self::Three => Self::Three,
            Self::Four => Self::Four,
            Self::Five => Self::Five,
            Self::Six => Self::Six,
            Self::Seven => Self::Seven,
            Self::Eight => Self::Eight,
            Self::Nine => Self::Nine,
            Self::Ten => Self::Ten,
            Self::Jack => Self::Jack,
            Self::Queen => Self::Queen,
            Self::King => Self::King,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordering() {
        let ace = Rank::Ace;
        let king = Rank::King;
        assert!(ace > king);
        let two = Rank::Two;
        assert!(two < king);
        assert!(two < ace);
        let ten = Rank::Ten;
        assert!(ten > two);
        assert!(ten < king);
        assert!(ten < ace);
        let ace_2 = Rank::Ace;
        assert_eq!(ace, ace_2);
        assert_ne!(ace, king);
    }
}
