use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::card::Card;
use crate::player::Player;
use crate::action::Action;
use crate::game_type::GameType;
use crate::lobby::LobbyStatus;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginAttempt {
    pub uuid: String
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameState {
    pub community_cards: Vec<Card>,
    pub players: Vec<Player>,
    pub active_player: Uuid,
    pub pot_amount: u32,
    pub dealer_position: u32,
    pub bet_amount: u32,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerAction {
    pub acting_player_id: Uuid,
    pub action: Action,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LobbyListItem {
    pub lobby_id: u32,
    pub status: LobbyStatus,
    pub user_count: u32,
    pub game_type: GameType,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LobbyActionType {
    Create,
    Join,
    Leave,
    Start
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LobbyAction {
    pub lobby_id: u32,
    pub action_type: LobbyActionType,
    pub user_id: String,
    pub game_type: GameType,
}