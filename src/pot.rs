use std::vec::Vec;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::clone::Clone;

use uuid::Uuid;
use mongodb::results::InsertOneResult;
use bson::de::from_bson;

use crate::database::db_handler::DbHandler;
use crate::database::db_structs::{Round, Turn};
use crate::action::Action;
use crate::player::{self, Player};
use crate::card::Card;

mod stakes;
use stakes::Stakes;

/// Pot struct
/// 
/// Intended to keep track of what moves player made during a game as well
/// as the current stakes for players. The stakes are updated each time a
/// turn played and added to the pot's history.
/// 
/// NOTE: No checks for correctness are implemented in Pot. This must be
/// done when Turns are being created.
pub struct Pot {
    history: Vec<(Uuid, Action, usize, Vec<Card>)>,
    stakes: Stakes,
    db_handler: DbHandler,
}

impl Pot {
    /// Initialize pot with list of Uuids.
    pub fn new_uuids(players: &Vec<Uuid>, db_handler: DbHandler) -> Pot {
        return Pot {
            history: Vec::new(),
            stakes: Stakes::new_uuids(players),
            db_handler: db_handler,
        };
    }

    /// Initialize pot with list of Player structs.
    pub fn new(players: &Vec<&Player>, db_handler: DbHandler) -> Pot {
        return Pot {
            history: Vec::new(),
            stakes: Stakes::new(players),
            db_handler: db_handler,
        };
    }

    /// Gets the current call amount.
    pub fn get_call_amount(&self) -> i64 {
        let amount = self.stakes.max();
        assert!(amount >= 0, "Found negative call amount!");
        return amount;
    }


    /// Divides winnings of the current pot, this includes division of winnings over side pots.
    /// 
    /// winning_order is a collection of player IDs in order of most winning (at first index) 
    /// and least winning (last index). Only IDs of players who have played during a pot should
    /// be in winning_order.
    /// 
    /// This function will modify pot's history and add additional turns that specify winnings/losings
    /// of each player at the end of the round.
    /// 
    /// A HashMap of player winnings is returned from this method so balance fields in Player structs 
    /// can be updated based on their wins and losses.
    pub fn divide_winnings(&mut self, winning_order: Vec<Vec<Uuid>>) -> Stakes { 
        let mut remaining_stakes = self.stakes.clone();
        let mut net_balance_changes  = Stakes::new_uuids(&self.stakes.get_player_ids().iter().map(|x| **x).collect());
        let mut winnings = Stakes::new_uuids(&self.get_player_ids());
        loop {
            let remaining_amount = remaining_stakes.sum();
            if remaining_amount == 0 { break; }
            
            // Find minimum non-zero player stakes (this will determine pot amount).
            let min_stakes: i64 = remaining_stakes.iter().fold(10000000000, |acc, (_, stake)| {
                if *stake != 0 && *stake < acc {
                    return *stake;
                }
                acc
            });
            assert!(0 < min_stakes && min_stakes <= 10000000000, "Illegal min stakes");

            // Find elligible winners.
            let mut highest_non_folding_players = Vec::new();
            let mut pot_winners = Vec::new();
            let mut winners_with_stakes = false;
            let mut highest_non_folded = false;
            for winners in winning_order.iter() {
                for player in winners {
                    if !self.player_has_folded(&player) {
                        if !highest_non_folded {
                            highest_non_folding_players.push(player);
                            highest_non_folded = true;
                        }

                        if remaining_stakes.get(&player) >= min_stakes {
                            pot_winners.push(player);
                            winners_with_stakes = true;
                        }
                    }
                }
                if winners_with_stakes { break; }
            }

            // Gather pot money from players.
            let mut pot_amount = 0;
            for player in self.get_player_ids() {
                let stakes = remaining_stakes.get(&player);
                if  stakes != 0 {
                    assert!(stakes >= min_stakes, "Player {} has ${} while the minimum stakes are {}", player, stakes, min_stakes);
                    remaining_stakes.add(player, -(min_stakes as i64));
                    net_balance_changes.add(player, -(min_stakes as i64));
                    pot_amount += min_stakes;
                }
            }

            // Give pot money to winners.
            if pot_winners.len() > 0 {
                for winner in pot_winners.iter() {
                    net_balance_changes.add(**winner, pot_amount / pot_winners.len() as i64);
                    winnings.add(**winner, pot_amount / pot_winners.len() as i64);
                }
            } else {
                for player in highest_non_folding_players.iter() {
                    net_balance_changes.add(**player, pot_amount / highest_non_folding_players.len() as i64);
                    winnings.add(**player, pot_amount / highest_non_folding_players.len() as i64);
                }
            }
        }

        // Adds wins and losses to history.
        let next_phase_num = match self.history.last() {
            Some((_, _, last_phase_num, _)) => last_phase_num + 1,
            None => 0,
        };
        for (player_id, winnings) in net_balance_changes.iter(){
            if *winnings > 0 {
                self.add_turn(&player_id, Action::Win(*winnings as usize), next_phase_num, Vec::new());
            } else {
                self.add_turn(&player_id, Action::Lose(*winnings as usize), next_phase_num, Vec::new());
            }
        }

        assert_eq!(remaining_stakes.sum(), 0);

        winnings
    }

    /// Reset pot to be ready for a new round.
    pub fn clear(&mut self, players: &Vec<&Player>) {
        self.history = Vec::new();
        self.stakes = Stakes::new(players);
    }

    /// Reset pot to be ready for a new round.
    pub fn clear_uuids(&mut self, player_ids: &Vec<Uuid>) {
        self.history = Vec::new();
        self.stakes = Stakes::new_uuids(player_ids);
    }

    /// Get the stake for a particular player in the pot.
    pub fn get_player_stake(&self, player_id: &Uuid) -> i64 {
        let player_stakes = self.stakes.get(player_id);
        assert!(player_stakes >= 0, "Player {} cannot have negative stakes!", *player_id);
        return player_stakes;
    }

    /// Get the total stake from all players in the pot.
    pub fn get_total_stake(&self) -> u32 {
        let mut total_stake = 0;
        for player_id in self.get_player_ids() {
            total_stake += self.get_player_stake(&player_id);
        }
        return total_stake as u32;
    }

    /// Checks if a particular player has folded in the pot's history.
    pub fn player_has_folded(&self, player_id: &Uuid) -> bool {
        self.history.iter().fold(false, |acc, (acting_player_id, action, _, _)| {
            acc || (*acting_player_id == *player_id && *action == Action::Fold)
        })
    }

    /// Counts numbers of players who have folded based on pot's history.
    pub fn number_of_players_folded(&self) -> u32 {
        let mut count = 0;
        for player_id in self.get_player_ids() {
            if self.player_has_folded(&player_id) {
                count += 1;
            }
        }
        count
    }

    /// Returns player IDs in the current pot.
    pub fn get_player_ids(&self) -> Vec<Uuid> {
        let mut id_set= HashSet::new();
        self.history.iter().for_each(|(player_id, _, _, _)| {
            id_set.insert(*player_id);
        });
        id_set.into_iter().collect()
    }

    /// Adds a turn to the pot's history.
    /// This method does minimial checks and integrity of pot history has to
    /// be maintained by the owner of the pot instance.
    pub fn add_turn(&mut self, player_id: &Uuid, action: Action, phase_num: usize, hand: Vec<Card>) {
        let player_stake= self.stakes.get(&player_id);

        match action {
            Action::Ante(amount) | Action::Bet(amount) | Action::Raise(amount) | Action::AllIn(amount) => {
                assert!(amount > player_stake as usize);
                self.stakes.set(*player_id, amount as i64);
            },
            Action::Call => {
                let call_amount = self.get_call_amount();
                assert!(call_amount > player_stake);
                self.stakes.set(*player_id, call_amount);
            },
            _ => (),
        }
        self.history.push((*player_id, action, phase_num, hand));
    }

    /// Saves turns in DB and adds new round document to Rounds.
    /// This is intended to be used at the end of a round when no more turns will be played.
    pub async fn save(&self, game_id: Uuid) {
        let mut turn_ids = Vec::new();
        let round_id = Uuid::now_v7();
        for (player_id, action, phase_num, hand) in self.history.iter() {
            let insert_result = self.db_handler.add_document(Turn {
                _id: Uuid::now_v7(),
                round_id: round_id,
                phase_num: *phase_num,
                acting_player_id: *player_id,
                hand: hand.clone(),
                action: action.clone(),
            }, "Turns").await;

            match insert_result.unwrap() {
                Ok(res) => {
                    match from_bson::<Uuid>(res.inserted_id) {
                        Ok(id) => turn_ids.push(id),
                        Err(e) => println!("Error when deserializing BSON to UUID: {:?}", e),
                    }
                }
                Err(e) => println!("Error when adding turn to Turns collection: {:?}", e),
            }
        }

        let round = Round {
            _id: round_id,
            game_id: game_id,
            turn_ids: turn_ids,
            player_ids: self.get_player_ids(),
        };

        match self.db_handler.add_document(round, "Rounds").await.unwrap() {
            Ok(res) => println!("Successfully added round to Rounds with ID: {}", res.inserted_id),
            Err(e) => println!("Error when adding round to Rounds collection: {:?}", e),
        }
    }
}


#[cfg(test)]
mod tests {
    use futures::stream::Fold;
    use test_context::{TestContext, test_context};
    use std::ptr::swap;

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
                pot: Pot::new_uuids(&player_ids, DbHandler::new_dummy())
            };
        }
    }

    #[test_context(Context)]
    #[test]
    fn test_add_turn(ctx: &mut Context) {
        let bet_amount = 100;
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Bet(bet_amount), 0, Vec::new());
        assert_eq!(ctx.pot.get_player_stake(&ctx.player_ids[0]), bet_amount as i64, "Stake amount is not the same after bet turn!");
    }

    #[test_context(Context)]
    #[test]
    fn test_get_non_player_id(ctx: &mut Context) {
        assert_eq!(ctx.pot.get_player_stake(&Uuid::now_v7()), 0);
    }

    #[test_context(Context)]
    #[test]
    fn test_divide_winnings_auto_win(ctx: &mut Context) {
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[1], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[2], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[3], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[4], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[5], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[6], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[7], Action::Ante(5), 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[8], Action::Ante(5), 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[9], Action::Ante(5), 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[7], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[8], Action::Fold, 0, Vec::new());

        let mut players = ctx.player_ids.clone();
        players.swap(8, 9);
        players.reverse();
        let winning_order = players.iter().map(|x| vec![*x]).collect();
        let winnings = ctx.pot.divide_winnings(winning_order);
        assert_eq!(winnings.get(&ctx.player_ids[0]), 0, "Player 0 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[1]), 0, "Player 1 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[2]), 0, "Player 2 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[3]), 0, "Player 3 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[4]), 0, "Player 4 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[5]), 0, "Player 5 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[6]), 0, "Player 6 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[7]), 0, "Player 7 has incorrect winnings");
        assert_eq!(winnings.get(&ctx.player_ids[8]), 0, "Player 8 has incorrect winnings");
        assert_eq!(winnings.get(&ctx.player_ids[9]), 15, "Player 10 has incorrect winnings");
    }

    #[test_context(Context)]
    #[test]
    fn test_divide_winnings_ties(ctx: &mut Context) {
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[1], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[2], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[3], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[4], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[5], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[6], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[7], Action::Ante(5), 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[8], Action::Ante(5), 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[9], Action::Ante(5), 0, Vec::new());

        let mut players = ctx.player_ids.clone();
        players.reverse();
        let mut winning_order = vec![vec![players[0], players[1], players[2]]];
        winning_order.extend(players[3..].iter().map(|x| vec![*x]));
        println!("{:?}", winning_order);

        let pot_winnings = ctx.pot.divide_winnings(winning_order);
        for (&player, &winnings) in pot_winnings.iter() {
            if player == ctx.player_ids[9] || player == ctx.player_ids[8] || player == ctx.player_ids[7] {
                assert_eq!(winnings, 5);
            } else {
                assert_eq!(winnings, 0);
            }
        }
    }

    #[test_context(Context)]
    #[test]
    fn test_divide_winnings_only_main_pot(ctx: &mut Context) {
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[1], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[2], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[3], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[4], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[5], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[6], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[7], Action::Bet(5), 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[8], Action::Bet(5), 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[9], Action::Bet(5), 0, Vec::new());

        let mut players = ctx.player_ids.clone();
        players.reverse();
        let winning_order = players.iter().map(|x| vec![*x]).collect();
        let winnings = ctx.pot.divide_winnings(winning_order);
        assert_eq!(winnings.get(&ctx.player_ids[0]), 0, "Player 0 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[1]), 0, "Player 1 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[2]), 0, "Player 2 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[3]), 0, "Player 3 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[4]), 0, "Player 4 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[5]), 0, "Player 5 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[6]), 0, "Player 6 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[7]), 0, "Player 7 has incorrect winnings");
        assert_eq!(winnings.get(&ctx.player_ids[8]), 0, "Player 8 has incorrect winnings");
        assert_eq!(winnings.get(&ctx.player_ids[9]), 15, "Player 10 has incorrect winnings");
    }

    #[test_context(Context)]
    #[test]
    fn test_divide_winnings_side_pots(ctx: &mut Context) {
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[1], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[2], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[3], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[4], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[5], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[6], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[7], Action::Bet(15), 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[8], Action::Bet(10), 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[9], Action::Bet(5), 0, Vec::new());

        let mut players = ctx.player_ids.clone();
        players.reverse();
        let winning_order = players.iter().map(|x| vec![*x]).collect();
        let winnings = ctx.pot.divide_winnings(winning_order);
        assert_eq!(winnings.get(&ctx.player_ids[0]), 0, "Player 0 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[1]), 0, "Player 1 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[2]), 0, "Player 2 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[3]), 0, "Player 3 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[4]), 0, "Player 4 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[5]), 0, "Player 5 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[6]), 0, "Player 6 has non-zero winnings");
        assert_eq!(winnings.get(&ctx.player_ids[7]), 5, "Player 7 has incorrect winnings");
        assert_eq!(winnings.get(&ctx.player_ids[8]), 10, "Player 8 has incorrect winnings");
        assert_eq!(winnings.get(&ctx.player_ids[9]), 15, "Player 10 has incorrect winnings");
    }


    #[test_context(Context)]
    #[test]
    fn test_number_of_players_folded(ctx: &mut Context) {
        assert_eq!(ctx.pot.number_of_players_folded(), 0);

        ctx.pot.add_turn(&ctx.player_ids[0], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[1], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[2], Action::Fold, 0, Vec::new());
        ctx.pot.add_turn(&ctx.player_ids[3], Action::Fold, 0, Vec::new());

        assert_eq!(ctx.pot.number_of_players_folded(), 4);
    }
}


#[cfg(test)]
mod db_tests {
    use bson::doc;
    use test_context::{AsyncTestContext, test_context};

    use super::*;
    use crate::card::{Card, Rank, Suit};

    struct Context {
        player_ids: Vec<Uuid>,
        pot: Pot,
        test_conn: DbHandler,
    }

    impl AsyncTestContext for Context {
        async fn setup() -> Self {
            let n = 10;
            let mut player_ids = Vec::new();
            for _ in 0..n {
                player_ids.push(Uuid::now_v7());
            }

            let db_conn = DbHandler::new("mongodb://localhost:27017/".to_string(), "test".to_string()).await.unwrap();
            let test_conn = DbHandler::new("mongodb://localhost:27017/".to_string(), "test".to_string()).await.unwrap();
            Context {
                player_ids: player_ids.clone(),
                pot: Pot::new_uuids(&player_ids, db_conn),
                test_conn: test_conn,
            }
        }
    }

    fn gen_random_hand(card_num: u32) -> Vec<Card> {
        let mut ran_hand = Vec::new();
        for _ in 0..card_num {
            let rand_rank = Rank::to_rank(rand::random_range(2..=14));
            let rand_suit = match rand::random_range(0..4) {
                0 => Suit::Clubs,
                1 => Suit::Hearts,
                2 => Suit::Diamonds,
                3 => Suit::Spades,
                _ => panic!("Unexpected value when generating random hand.")
            };
            ran_hand.push(Card::new(rand_rank, rand_suit));
        }
        ran_hand
    }


    #[test_context(Context)]
    #[tokio::test]
    #[ignore]
    async fn test_save(ctx: &mut Context) {
        let game_id = Uuid::now_v7();
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Bet(10), 0, gen_random_hand(5));
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Bet(20), 0, gen_random_hand(5));
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Bet(30), 0, gen_random_hand(5));
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Bet(40), 0, gen_random_hand(5));
        ctx.pot.add_turn(&ctx.player_ids[1], Action::Bet(100), 0, gen_random_hand(5));
        ctx.pot.add_turn(&ctx.player_ids[2], Action::Bet(1000), 0, gen_random_hand(5));
        ctx.pot.add_turn(&ctx.player_ids[2], Action::Bet(2000), 0, gen_random_hand(5));
        ctx.pot.save(game_id).await;

        assert_eq!(ctx.test_conn.count_documents::<Turn>(doc! {"acting_player_id": &ctx.player_ids[0].simple().to_string()}, "Turns").await.unwrap().unwrap(), 4);
        assert_eq!(ctx.test_conn.count_documents::<Turn>(doc! {"acting_player_id": &ctx.player_ids[1].simple().to_string()}, "Turns").await.unwrap().unwrap(), 1);
        assert_eq!(ctx.test_conn.count_documents::<Turn>(doc! {"acting_player_id": &ctx.player_ids[2].simple().to_string()}, "Turns").await.unwrap().unwrap(), 2);
    }
}