use std::collections::HashSet;
use std::hash::Hash;

use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::database::db_handler::DbHandler;
use crate::database::db_structs::Game;
use crate::game_type::GameType;
use crate::input::Input;
use crate::rules::five_card_draw::FiveCardDraw;
use crate::rules::seven_card_stud::SevenCardStud;
use crate::rules::texas_holdem::TexasHoldem;
use crate::rules::{Rules, RulesEnum};
use crate::player::Player;
use crate::input::cli_input::CliInput;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LobbyStatus {
    InLobby,
    InGame,
}


pub struct Lobby<I: Input> {
    id: u32,
    status: LobbyStatus,
    users: HashSet<Uuid>,
    active_players: Vec<Player>,
    rules: RulesEnum<I>,
}


impl<I: Input> Lobby<I> {
    pub async fn new(id: u32, game_type: GameType) -> Self {
        let db_handler = match DbHandler::new("mongodb://localhost:27017/".to_string(), "poker".to_string()).await {
            Ok(handler) => handler,
            Err(e) => {
                println!("Using dummy DbHandler due to error: {}", e);
                DbHandler::new_dummy()
            }
        };
        Self { 
            id: id, 
            status: LobbyStatus::InLobby, 
            users: HashSet::new(), 
            active_players: Vec::new(), 
            rules: match game_type {
                GameType::FiveCardDraw => RulesEnum::FiveCardDraw(FiveCardDraw::new(1000, 1, db_handler, Uuid::now_v7())),
                GameType::SevenCardStud => RulesEnum::SevenCardStud(SevenCardStud::new(1000, 1, db_handler, Uuid::now_v7())),
                GameType::TexasHoldem => RulesEnum::TexasHoldem(TexasHoldem::new(1000, 1, db_handler, Uuid::now_v7())),
            }
        }
    }

    // Starts for a specific lobby.
    pub async fn start_game(&mut self) {
        self.active_players.clear();
        for user in self.users.iter() {
            self.active_players.push(Player::new(*user, user.simple().to_string(), 1000));
        }
        self.status = LobbyStatus::InGame;
        let _ = match &mut self.rules {
            RulesEnum::FiveCardDraw(ref mut rules) => rules.play_round(self.active_players.clone()).await,
            RulesEnum::SevenCardStud(ref mut rules) => rules.play_round(self.active_players.clone()).await,
            RulesEnum::TexasHoldem(ref mut rules) => rules.play_round(self.active_players.clone()).await,
        };
    }

    pub fn status(&self) -> LobbyStatus {
        self.status.clone()
    }

    // Counts the number of users.
    pub fn count_users(&self) -> u32 {
        self.users.len() as u32
    }


    pub fn rules(&self) -> &RulesEnum<I> {
        &self.rules
    }

    // Adds user to user list.
    pub fn join_user(&mut self, user_id: Uuid) -> Result<(), ()> {
        match self.users.get(&user_id) {
            Some(_) => Err(()),
            None => {
                self.users.insert(user_id);
                Ok(())
            },
        }
    }

    // Removes user from users list.
    pub fn leave_user(&mut self, user_id: Uuid) -> Result<(), ()> {
        match self.get_user(user_id) {
            None => Err(()),
            Some(_) => {
                self.users.remove(&user_id);
                Ok(())
            },
        }
    }


    pub fn id(&self) -> u32 {
        self.id
    } 


    pub fn get_user(&self, user_id: Uuid) -> Option<&Uuid> {
        self.users.get(&user_id)
    }


    pub fn users(&self) -> &HashSet<Uuid> {
        &self.users
    }


    pub fn active_players(&self) -> &Vec<Player> {
        &self.active_players
    }


    pub fn game_type(&self) -> GameType {
        match self.rules {
            RulesEnum::FiveCardDraw(_) => GameType::FiveCardDraw,
            RulesEnum::SevenCardStud(_) => GameType::SevenCardStud,
            RulesEnum::TexasHoldem(_) => GameType::TexasHoldem,
        }
    }
}