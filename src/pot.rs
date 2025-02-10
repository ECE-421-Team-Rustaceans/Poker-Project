use std::vec::Vec;
use std::cmp::{max, min};
use std::collections::HashMap;
use uuid::Uuid;
use std::clone::Clone;

use crate::database::db_structs::Turn;
use crate::database::db_structs::Action::*;
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

/// Pot struct
/// 
/// Intended to keep track of what moves player made during a game as well
/// as the current stakes for players. The stakes are updated each time a
/// turn played and added to the pot's history.
/// 
/// NOTE: No checks for correctness are implemented in Pot. This must be
/// done when Turns are being created.
pub struct Pot {
    history: Vec<Turn>,
    stakes: Stakes,
}

impl Pot {
    /// Initialize pot with list of Uuids.
    pub fn new_uuids(players: &Vec<Uuid>) -> Pot {
        return Pot {
            history: Vec::new(),
            stakes: Stakes::new_uuids(players),
        };
    }

    /// Initialize pot with list of Player structs.
    pub fn new(players: &Vec<&Player>) -> Pot {
        return Pot {
            history: Vec::new(),
            stakes: Stakes::new(players),
        };
    }

    /// Gets the current call amount.
    pub fn get_call_amount(&self) -> usize {
        return self.stakes.max();
    }

    /// Divide the winnings of a single pot. To divide winnings for all
    /// pots, use divide_winnings().
    fn divide_pot(&self, pot_stakes: &Stakes, winning_order: &Vec<Uuid>) -> HashMap<Uuid, i64> {
        let mut player_winnings: HashMap<Uuid, i64> = HashMap::new();
        for winner in winning_order {
            let winner_stake = pot_stakes.get(winner);
            if winner_stake > 0 {
                let mut winner_winnings: i64 = 0;
                for loser in pot_stakes.get_player_ids() {
                    if *winner != *loser {
                        let loser_stakes = pot_stakes.get(loser);
                        let delta = min(loser_stakes, winner_stake) as i64;
                        player_winnings.insert(*loser, -delta);
                        winner_winnings += delta;
                    }
                }
                player_winnings.insert(*winner, winner_winnings);
                break;
            }
        }
        return player_winnings;
    }

    /// Divides winnings of the current pot, this includes division of winnings over side pots.
    pub fn divide_winnings(&self, winning_order: Vec<Uuid>) -> HashMap<Uuid, i64> { 
        let mut remaining_stakes = self.stakes.clone();
        let mut total_player_winnings: HashMap<Uuid, i64> = HashMap::new();
        loop {
            let remaining_amount = remaining_stakes.sum();
            if remaining_amount == 0 { break; } 
            let side_pot_winnings =  self.divide_pot(&remaining_stakes, &winning_order);
            for (player_id, pot_winnings) in side_pot_winnings {
                remaining_stakes.add(player_id, pot_winnings);
                let player_curr_winnings = match total_player_winnings.get(&player_id) {
                    Some(winnings) => *winnings,
                    None => 0,
                };
                total_player_winnings.insert(player_id, player_curr_winnings + pot_winnings);
            }
        }
        return total_player_winnings;
    }

    /// Get the stake for a particular player in the pot.
    pub fn get_player_stake(&self, player_id: &Uuid) -> usize {
        return self.stakes.get(player_id);
    }

    /// Adds a turn to the pot's history.
    /// This method does minimial checks and integrity of pot history has to
    /// be maintained by the owner of the pot instance.
    pub fn add_turn(&mut self, new_turn: Turn) {
        let player_id = new_turn.acting_player_id;
        let player_stake= self.stakes.get(&new_turn.acting_player_id);

        match new_turn.action {
            Ante(amount) | Bet(amount) | Raise(amount) | AllIn(amount) => {
                assert!(amount > player_stake);
                self.stakes.set(player_id, amount);
            },
            Call => {
                let call_amount = self.get_call_amount();
                assert!(call_amount > player_stake);
                self.stakes.set(player_id, call_amount);
            },
            _ => (),
        }
    }
}


#[cfg(test)]
mod tests {
    use test_context::{TestContext, test_context};

    use super::*;

    struct Context {
        player_ids: Vec<Uuid>,
        pot: Pot,
    }

    impl TestContext for Context {
        fn setup() -> Self {
            let n = 10;
            let mut player_ids = Vec::new();
            for _ in 0..n {
                player_ids.push(Uuid::now_v7());
            }

            return Context {
                player_ids: player_ids.clone(),
                pot: Pot::new_uuids(&player_ids),
            };
        }
    }

    #[test_context(Context)]
    #[test]
    fn test_add_turn(ctx: &mut Context) {
        let bet_amount = 100;
        let turn = Turn {
            round_id: Uuid::now_v7(),
            turn_id: Uuid::now_v7(),
            phase_num: 1,
            acting_player_id: ctx.player_ids[0],
            hand: Vec::new(),
            action: Bet(bet_amount),
        };
        ctx.pot.add_turn(turn);
        assert_eq!(ctx.pot.get_player_stake(&ctx.player_ids[0]), bet_amount, "Stake amount is not the same after bet turn!");
    }


    #[test_context(Context)]
    #[test]
    #[should_panic]
    fn test_add_turn_panic(ctx: &mut Context) {
        let bet_amount = 100;
        let turn = Turn {
            round_id: Uuid::now_v7(),
            turn_id: Uuid::now_v7(),
            phase_num: 1,
            acting_player_id: ctx.player_ids[0],
            hand: Vec::new(),
            action: Bet(bet_amount),
        };
        ctx.pot.add_turn(turn);
        ctx.pot.get_player_stake(&Uuid::now_v7());
    }
}