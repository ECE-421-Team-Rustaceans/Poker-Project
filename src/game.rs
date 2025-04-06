use uuid::Uuid;
use std::vec::Vec;
use crate::{database::db_handler::DbHandler, player::Player, rules::Rules};


pub struct Game<T: Rules> {
    players: Vec<Player>,
    rules: T,
    minimum_bet: u32,
}


impl<T: Rules> Game<T> {
    pub fn new(raise_limit: u32, minimum_bet: u32, db_handler: DbHandler) -> Game<T> {
        let game_id = Uuid::now_v7();
        let players = Vec::new();
        return Game {
            players,
            rules: T::new(raise_limit, minimum_bet, db_handler, game_id),
            minimum_bet
        };
    }

    pub async fn play_game(&mut self) {
        loop {
            let mut player_indices_to_remove: Vec<usize> = self.players.iter().enumerate().filter(|(_, player)| player.balance() < self.minimum_bet as usize).map(|(player_index, _)| player_index).collect();
            player_indices_to_remove.reverse();
            player_indices_to_remove.iter().for_each(|player_index| {self.players.remove(*player_index);});

            if self.players.len() > 0 {
                self.rules.play_round(self.players.drain(..).collect()).await.unwrap();
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
