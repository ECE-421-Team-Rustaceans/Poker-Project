use strum::IntoEnumIterator;
use rand::prelude::*;

pub use super::card::{Card, Rank, Suit};

#[derive(Debug)]
/// Deck class, representing a normal deck of 52 cards
/// except that there are no jokers in this deck
/// Create a new deck with Deck::new().
/// There should only be one deck per game.
/// Cards can be dealt (at random), but they must later be returned to the deck.
/// Example:
/// ```
/// let deck = Deck::new();
/// let card = deck.deal().unwrap();
/// deck.return_card(card).expect("Card was already in deck...");
/// ```
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    /// Constructor for Deck.
    /// Example:
    /// ```
    /// let deck = Deck::new();
    /// ```
    pub fn new() -> Deck {
        let mut deck = Deck {
            cards: Vec::new()
        };

        for rank in Rank::iter() {
            for suit in Suit::iter() {
                deck.cards.push(Card::new(rank.clone(), suit));
            }
        }

        return deck;
    }

    /// Deals a card from the deck at random.
    /// Err(String) if the deck no longer contains any cards,
    /// otherwise Ok(Card)
    pub fn deal(&mut self) -> Result<Card, String> {
        if self.cards.is_empty() {
            return Err("There are no cards remaining in the deck, so no card can be dealt".to_string());
        }
        let mut rng = rand::rng();
        let index = match (0..self.cards.len()).choose(&mut rng) {
            Some(card) => card,
            None => panic!("There was a problem picking a card to deal, even though there were cards in the deck...")
        };
        let card = self.cards.swap_remove(index);

        return Ok(card);
    }

    /// Return a card to the deck so that it can be dealt.
    /// If cards are not returned to the deck, they will never
    /// be able to be dealt again by this deck, and the deck will
    /// run out of cards.
    /// 
    /// panics if the returned card already exists in the deck.
    pub fn return_card(&mut self, card: Card) {
        if self.cards.contains(&card) {
            panic!("Card that was returned to Deck already existed in Deck, it is a duplicate Card");
        }
        self.cards.push(card);
    }
}
