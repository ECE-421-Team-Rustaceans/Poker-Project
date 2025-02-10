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
            Some(stake) => *stake as i64,
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


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use test_context::{test_context, TestContext};

    struct Context {
        player_ids: Vec<Uuid>,
        stakes: Stakes,
    }

    impl TestContext for Context {
        fn setup() -> Self {
            let n = 10;
            let mut ids = Vec::new();
            for _ in 0..n {
                ids.push(Uuid::now_v7());
            }

            let stakes = Stakes::new_uuids(&ids);

            return Context {
                player_ids: ids,
                stakes: stakes,
            };
        }
    }

    #[test_context(Context)]
    #[test]
    fn test_basic_add(ctx: &mut Context) {
        ctx.stakes.set(ctx.player_ids[0], 10);
        ctx.stakes.add(ctx.player_ids[0], 20);
        assert_eq!(ctx.stakes.get(&ctx.player_ids[0]), 30);
    }

    #[test_context(Context)]
    #[test]
    fn test_random_add(ctx: &mut Context) {
        let mut rng = rand::rng();
        let x: usize = rng.random_range(0..1000);
        let y: i64 = rng.random_range(10..1000);

        ctx.stakes.set(ctx.player_ids[0], x);
        ctx.stakes.add(ctx.player_ids[0], y);
        assert_eq!(ctx.stakes.get(&ctx.player_ids[0]), x + y as usize);
    }

    #[test_context(Context)]
    #[test]
    #[should_panic]
    fn test_bad_add(ctx: &mut Context) {
        ctx.stakes.set(ctx.player_ids[0], 10);
        ctx.stakes.add(ctx.player_ids[0], -50);
    }

    #[test_context(Context)]
    #[test]
    fn test_max(ctx: &mut Context) {
        ctx.stakes.set(ctx.player_ids[0], 10);
        ctx.stakes.set(ctx.player_ids[1], 500);
        ctx.stakes.set(ctx.player_ids[2], 40);
        assert_eq!(ctx.stakes.max(), 500);
    }

    #[test_context(Context)]
    #[test]
    fn test_sum(ctx: &mut Context) {
        ctx.stakes.set(ctx.player_ids[0], 10);
        ctx.stakes.set(ctx.player_ids[1], 500);
        ctx.stakes.set(ctx.player_ids[2], 40);
        assert_eq!(ctx.stakes.sum(), 550);
    }

    #[test_context(Context)]
    #[test]
    fn test_get_player_ids(ctx: &mut Context) {
        let player_ids = ctx.stakes.get_player_ids();
        for id in player_ids {
            assert!(ctx.player_ids.contains(id));
        }
    }
}