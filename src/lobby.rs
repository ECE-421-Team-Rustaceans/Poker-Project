use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::rules::Rules;
use crate::player::Player;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LobbyStatus {
    InLobby,
    InGame,
}


pub struct Lobby<'a> {
    id: u32,
    status: LobbyStatus,
    users: Vec<Uuid>,
    active_players: Vec<Player>,
    rules: Box<dyn Rules<'a> + Send + Sync>,
}