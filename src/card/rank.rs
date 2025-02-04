use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
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
