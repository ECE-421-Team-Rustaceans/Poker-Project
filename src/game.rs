use uuid::Uuid;
use std::vec::Vec;
use crate::{database::db_handler::DbHandler, player::Player, rules::Rules};


pub struct Game<T: Rules> {
    players: Vec<Player>,
    rules: T,
    minimum_bet: u32,
}


impl<T: Rules> Game<T> {
    /// create a new game with the rules set by the generic parameter
    pub fn new(raise_limit: u32, minimum_bet: u32, db_handler: DbHandler) -> Game<T> {
        let game_id = Uuid::now_v7();
        let players = Vec::new();
        return Game {
            players,
            rules: T::new(raise_limit, minimum_bet, db_handler, game_id),
            minimum_bet
        };
    }

    /// play a round of the game using the rules defined by the generic parameter
    pub async fn play_game(&mut self) {
        let mut player_indices_to_remove: Vec<usize> = self.players.iter().enumerate().filter(|(_, player)| player.balance() < self.minimum_bet as usize).map(|(player_index, _)| player_index).collect();
        player_indices_to_remove.reverse();
        player_indices_to_remove.iter().for_each(|player_index| {self.players.remove(*player_index);});

        if self.players.len() > 0 {
            match self.rules.play_round(self.players.drain(..).collect()).await {
                Ok(players) => self.players = players,
                Err((err, players)) => {
                    println!("Error: {err}");
                    self.players = players;
                },
            };
        } else {
            println!("Not enough players to start a game!");
        }
    }

    /// find whether a player is in this game or not.
    /// returns Ok(i) iff the player with that ID is in this game,
    pub fn find_player_by_id(&self, player_id: Uuid) -> Result<usize, ()> {
        for (i, player) in self.players.iter().enumerate() {
            if player_id == player.account_id() {
                return Ok(i);
            }
        }
        return Err(());
    }

    /// add a player to this game.
    /// returns Ok(()) if the player was successfully added,
    /// and Err(message) if the player is already in this game
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

    /// remove a player from this game.
    /// returns Ok(()) if the player was successfully removed,
    /// and Err(message) if the player was not in the game in the first place
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

    /// get a list of all the players in the game
    pub fn players(&self) -> Vec<&Player> {
        return self.players.iter().collect();
    }
}
