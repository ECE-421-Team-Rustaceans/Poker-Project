use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
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
