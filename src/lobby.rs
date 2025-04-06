use std::collections::HashSet;

use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::database::db_handler::DbHandler;
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


    pub fn status(&self) -> LobbyStatus {
        self.status.clone()
    }


    pub fn count_users(&self) -> u32 {
        self.users.len() as u32
    }


    pub fn rules(&self) -> &RulesEnum<I> {
        &self.rules
    }


    pub fn join_user(&mut self, user_id: Uuid) {
        self.users.insert(user_id);
    }


    pub fn leave_user(&mut self, user_id: Uuid) {
        self.users.remove(&user_id);
    }
}