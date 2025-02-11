use uuid::Uuid;

use crate::card::Card;

#[derive(Debug)]
pub struct Player {
    account_id: Uuid,
    balance: usize,
    cards: Vec<Card>
}

impl Player {
    pub fn new() -> Player {
        let account_id = Uuid::now_v7();
        let balance: usize = 0;
        let cards: Vec<Card> = Vec::new();
        return Player {
            account_id,
            balance,
            cards
        };
    }

    pub fn balance(&self) -> usize {
        return self.balance;
    }

    pub fn account_id(&self) -> Uuid {
        return self.account_id;
    }

    pub fn obtain_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn return_cards(&mut self) -> Vec<Card> {
        let mut cards: Vec<Card> = Vec::new();
        for _ in 0..self.cards.len() {
            cards.push(self.cards.pop().expect( "Failed to return a card from the player"));
        }
        assert!(self.cards.len() == 0);
        return cards;
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        return self.account_id == other.account_id;
    }
}
