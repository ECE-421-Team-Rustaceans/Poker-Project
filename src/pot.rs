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
use crate::player::Player;
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
                let mut winner_winnings: i64 = winner_stake as i64;
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
                remaining_stakes.add(player_id, -pot_winnings.abs());
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

    pub fn player_has_folded(&self, player_id: &Uuid) -> bool {
        self.history.iter().fold(false, |acc, (acting_player_id, action, _, _)| {
            acc || (*acting_player_id == *player_id && *action == Action::Fold)
        })
    }


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
                assert!(amount > player_stake);
                self.stakes.set(*player_id, amount);
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
    /// 
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
                pot: Pot::new_uuids(&player_ids, DbHandler::new_dummy())
            };
        }
    }

    #[test_context(Context)]
    #[test]
    fn test_add_turn(ctx: &mut Context) {
        let bet_amount = 100;
        ctx.pot.add_turn(&ctx.player_ids[0], Action::Bet(bet_amount), 0, Vec::new());
        assert_eq!(ctx.pot.get_player_stake(&ctx.player_ids[0]), bet_amount, "Stake amount is not the same after bet turn!");
    }

    #[test_context(Context)]
    #[test]
    fn test_get_non_player_id(ctx: &mut Context) {
        assert_eq!(ctx.pot.get_player_stake(&Uuid::now_v7()), 0);
    }

    #[test_context(Context)]
    #[test]
    fn test_divide_pot(ctx: &mut Context) {
        let mut stakes = Stakes::new_uuids(&ctx.player_ids);
        stakes.set(ctx.player_ids[0], 100);
        stakes.set(ctx.player_ids[1], 400);
        let result = ctx.pot.divide_pot(&stakes, &ctx.player_ids);

        let player_1_winnings = match result.get(&ctx.player_ids[0]) {
            Some(x) => *x,
            None => -1,
        };

        let player_2_winnings= match result.get(&ctx.player_ids[1]) {
            Some(x) => *x,
            None => -1,
        };

        assert_eq!(player_1_winnings, 200);
        assert_eq!(player_2_winnings, -100);

        for i in 2..ctx.player_ids.len() {
            let winnings = match result.get(&ctx.player_ids[i]) {
                Some(x) => *x,
                None => -1,
            };
            assert_eq!(winnings, 0);
        }
    }
}


#[cfg(test)]
mod db_tests {
    use bson::doc;
    use test_context::{AsyncTestContext, test_context};

    use super::*;
    use crate::card::{Card, Rank, Suit};
    use futures::stream::{TryStreamExt};
    use mongodb::{bson::{Document}};

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