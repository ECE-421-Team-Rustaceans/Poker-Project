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
