use five_card_draw::FiveCardDraw;
use seven_card_stud::SevenCardStud;
use texas_holdem::TexasHoldem;
use uuid::Uuid;

use crate::{database::db_handler::DbHandler, input::Input, player::Player};
use crate::game_type::GameType;

/// trait containing necessary methods for each set of poker Rules
pub trait Rules {
    /// create a new instance of the rules, with a certain raise limit, minimum bet, and game ID
    fn new(raise_limit: u32, minimum_bet: u32, db_handler: DbHandler, game_id: Uuid) -> Self where Self: Sized;
    /// the play_round method takes care of all of the logic required the entire game, for a given variant of poker,
    /// the players are assumed to stay in the game for the entire round (but may change between rounds),
    /// and if a player leaves, they will be automatically folded
    async fn play_round(&mut self, players: Vec<Player>) -> Result<Vec<Player>, (&'static str, Vec<Player>)>;
}

pub enum RulesEnum<I: Input> {
    FiveCardDraw(FiveCardDraw<I>),
    SevenCardStud(SevenCardStud<I>),
    TexasHoldem(TexasHoldem<I>)
}


impl<I: Input> RulesEnum<I> {
    pub fn to_game_type(&self) -> GameType {
        match self {
            RulesEnum::FiveCardDraw(_) => GameType::FiveCardDraw,
            RulesEnum::SevenCardStud(_) => GameType::SevenCardStud,
            RulesEnum::TexasHoldem(_) => GameType::TexasHoldem,
        }
    }
}


pub mod five_card_draw;
pub mod seven_card_stud;
pub mod texas_holdem;
