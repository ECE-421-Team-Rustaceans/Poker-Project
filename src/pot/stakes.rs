use std::collections::HashMap;
use uuid::Uuid;
use std::cmp::max;

use crate::player::Player;

/// Stakes Struct
/// 
/// Private struct to help manage stakes in Pot class. It is mostly
/// a wrapper over a HashMap.
/// 
/// This is not intended to be used elsewhere, that is why is private.
/// It is possible to make this a general hashmap-like data structure
/// for numbers if its usefulness is necessary elsewere.
pub struct Stakes {
    stakes: HashMap<Uuid, usize>,
}


impl Stakes {
    /// Constructor with list of players.
    pub fn new(players: &Vec<&Player>) -> Stakes {
        let mut new_stakes= Stakes {
            stakes: HashMap::new(),
        };
        for player in players {
            new_stakes.set(player.account_id(), 0);
        }
        return new_stakes
    }

    /// Constructor with list of uuids.
    pub fn new_uuids(players: &Vec<Uuid>) -> Stakes {
        let mut new_stakes= Stakes {
            stakes: HashMap::new(),
        };
        for id in players{
            new_stakes.set(*id, 0);
        }
        return new_stakes
    }

    /// Adds the amount onto the player's stakes. 
    /// The sum should be non-negative otherwise it will panic!
    pub fn add(&mut self, player_id: Uuid, amount: i64) {
        let current_stake: i64 = match self.stakes.get(&player_id) {
            Some(stake) => (*stake as i64),
            None => 0,
        };

        assert!((current_stake as i64) + amount >= 0, "Adding to stake results in negative amount!");
        let new_stake: usize = (current_stake + amount) as usize;
        self.stakes.insert(player_id, new_stake);
    }

    /// HashMap set wrapper.
    pub fn set(&mut self, player_id: Uuid, amount: usize) {
        self.stakes.insert(player_id, amount);
    }

    /// HashMap get wrapper.
    pub fn get(&self, player_id: &Uuid) -> usize {
        match self.stakes.get(player_id) {
            Some(stake) => *stake,
            None => panic!("Cannot find player with stakes with ID {}", player_id),
        }
    }

    /// Gets the maximum stake.
    pub fn max(&self) -> usize {
        return self.stakes.iter().fold(0, |acc, (_, stake)| max(acc, *stake));
    }

    /// Calculates the sum of all stakes.
    pub fn sum(&self) -> usize {
        return self.stakes.iter().fold(0, |acc, (_, x)| acc + *x);
    }

    /// Gets player_ids with stakes.
    pub fn get_player_ids(&self) -> Vec<&Uuid> {
        let mut player_ids = Vec::new();
        for id in self.stakes.keys() {
            player_ids.push(id);
        }
        return player_ids;
    }
}


impl Clone for Stakes {
    fn clone(&self) -> Stakes {
        return Stakes {
            stakes: self.stakes.clone()
        };
    }
}
