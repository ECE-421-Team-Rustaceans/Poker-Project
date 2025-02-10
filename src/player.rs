use uuid::Uuid;

pub struct Player {
    account_id: Uuid,
    balance: usize,
}

impl Player {
    pub fn new() -> Player {
        let account_id = Uuid::now_v7();
        let balance: usize = 0;
        return Player {
            account_id,
            balance
        };
    }

    pub fn balance(&self) -> usize {
        return self.balance;
    }

    pub fn account_id(&self) -> Uuid {
        return self.account_id;
    }
}

impl Clone for Player {
    fn clone(&self) -> Player {
        Player {
            account_id: self.account_id,
            balance: self.balance,
        }
    }
}

