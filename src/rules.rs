use uuid::Uuid;

use crate::{database::db_handler::DbHandler, player::Player};

/// trait containing necessary methods for each set of poker Rules
pub trait Rules {
    fn new(raise_limit: u32, minimum_bet: u32, db_handler: DbHandler, game_id: Uuid) -> Self;
    /// the play_round method takes care of all of the logic required the entire game, for a given variant of poker
    async fn play_round(&mut self, players: Vec<Player>) -> Result<(), &'static str>;
}

pub mod five_card_draw;
pub mod seven_card_stud;
pub mod texas_holdem;
