use uuid::Uuid;
use std::vec::Vec;
use crate::{player::Player, rules::Rules};


pub struct Game {
    players: Vec<Player>,
    rules: Box<dyn Rules>,
    min_bet: usize,
}


impl Game {
    pub fn new() -> Game {
        todo!()
    }

    pub fn play_game(&mut self) {
        loop {
            let mut active_players: Vec<&mut Player> = Vec::new();
            for player in self.players.iter_mut() {
                if player.balance() >= self.min_bet {
                    active_players.push(player);
                }
            }

            if active_players.len() > 0 {
                self.rules.play_round(active_players);
            } else {
                break;
            }
        }
    }


    pub fn find_player_by_id(&self, player_id: Uuid) -> Result<usize, ()> {
        for (i, player) in self.players.iter().enumerate() {
            if player_id == player.account_id() {
                return Ok(i);
            }
        }
        return Err(());
    }


    pub fn add_player(&mut self, new_player: Player) -> Result<(), String> {
        let player_index = self.find_player_by_id(new_player.account_id());
        return match player_index {
            Ok(_) => Err("Player already in players for this game".to_string()),
            Err(_) => {
                self.players.push(new_player);
                return Ok(());
            },
        }
    }


    pub fn remove_player(&mut self, player_id: Uuid) -> Result<(), String> {
        let player_index = self.find_player_by_id(player_id);
        return match player_index {
            Ok(i) => {
                self.players.swap_remove(i);
                return Ok(());
            },
            Err(_) => Err("Could not remove player from game with that ID.".to_string()),
        };
    }
}
