use uuid::Uuid;

use crate::card::Card;

#[derive(Debug)]
pub struct Player {
    account_id: Uuid,
    name: String,
    balance: usize,
    cards: Vec<Card>
}

impl Player {
    pub fn new(account_id: Uuid, name: String, balance: usize) -> Player {
        let cards: Vec<Card> = Vec::new();
        return Player {
            account_id,
            name,
            balance,
            cards
        };
    }

    pub fn balance(&self) -> usize {
        return self.balance;
    }

    /// Removes the amount from the Player's wallet.
    /// Returns Ok(amount remaining in wallet) on success,
    /// but if the Player does not have enough funds to make the bet,
    /// Returns Err() and does not remove funds.
    pub fn bet(&mut self, amount: usize) -> Result<usize, &'static str> {
        if self.balance >= amount {
            self.balance = self.balance - amount;
            return Ok(self.balance);
        }
        else {
            return Err("Player does not have enough money remaining to make this bet");
        }
    }

    pub fn win(&mut self, amount: usize) {
        self.balance += amount;
    }

    pub fn account_id(&self) -> Uuid {
        return self.account_id;
    }

    pub fn name(&self) -> &str {
        return &self.name;
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

    pub fn peek_at_cards(&self) -> Vec<&Card> {
        return self.cards.iter().collect();
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        return self.account_id == other.account_id;
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Self { account_id: self.account_id.clone(), name: self.name.clone(), balance: self.balance.clone(), cards: self.cards.clone() }
    }
}
