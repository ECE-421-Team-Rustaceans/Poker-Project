use std::vec::Vec;
use std::collections::HashMap;
use uuid::Uuid;

use crate::database::db_structs::Turn;
use crate::player::Player;


pub struct Pot {
    history: Vec<Turn>,
    players: Vec<Box<Player>>,
}

impl Pot {
    pub fn calc_bet(&self, phase_num: usize) -> i64 {
        let mut bets: HashMap<Uuid, usize> = HashMap::new();
        let mut current_bet = 0;
        for turn in self.history.iter() {
            if turn.phase_num == phase_num {
                match turn.action {
                    _ => ()
                    // Call => {
                    //     if bets.contains_key(turn.acting_player_id) {

                    //     }
                    //     bets.insert()
                    // },
                    // Bet(amount) => ,
                    // Raise(amount) => ,
                    // AllIn => ,
                    // _ => ,
                }
            }
        }
        return 0;
    }


    pub fn divide_winnings(&self) -> Vec<(Uuid, i64)> { 
        return Vec::new();
    }


    pub fn add_turn(&mut self, new_turn: Turn) {
        self.history.push(new_turn);
    }
}