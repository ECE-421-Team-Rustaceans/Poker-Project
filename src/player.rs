use uuid::Uuid;

use crate::card::Card;

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
}
