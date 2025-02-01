#[derive(Debug)]
/// Rank class, representing the rank of a Card (the number / face)
pub enum Rank {
    Ace,
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
    King
}

impl Rank {
    /// true if Rank is a number (not including Ace)
    pub fn is_number(&self) -> bool {
        let numbers = vec![
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
