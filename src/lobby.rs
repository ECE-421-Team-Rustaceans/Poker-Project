use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::database::db_handler::DbHandler;
use crate::game_type::GameType;
use crate::input::Input;
use crate::rules::five_card_draw::FiveCardDraw;
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
    users: Vec<Uuid>,
    active_players: Vec<Player>,
    game_type: GameType,
    rules: RulesEnum<I>,
}


impl<I: Input> Lobby<I> {
    // pub async fn new(id: u32, game_type: GameType) -> Lobby {
    //     let db_handler = match DbHandler::new("mongodb://localhost:27017/".to_string(), "poker".to_string()).await {
    //         Ok(handler) => handler,
    //         Err(e) => {
    //             println!("Using dummy DbHandler due to error: {}", e);
    //             DbHandler::new_dummy()
    //         }
    //     };
    //     Lobby { 
    //         id: id, 
    //         status: LobbyStatus::InLobby, 
    //         users: Vec::new(), 
    //         active_players: Vec::new(), 
    //         game_type: game_type, 
    //         rules: match game_type {
    //             GameType::FiveCardDraw => Box::new(FiveCardDraw::<CliInput>::new(1000, db_handler, Uuid::now_v7())),
    //             GameType::TexasHoldem => Box::new(FiveCardDraw::<CliInput>::new(1000, db_handler, Uuid::now_v7())),
    //             GameType::SevenCardStud => Box::new(FiveCardDraw::<CliInput>::new(1000, db_handler, Uuid::now_v7())),
    //         }
    //     }
    // }


    pub fn status(&self) -> LobbyStatus {
        self.status.clone()
    }


    pub fn count_users(&self) -> u32 {
        self.users.len() as u32
    }


    pub fn game_type(&self) -> GameType {
        self.game_type.clone()
    }
}