use crate::player::Player;

/// trait containing necessary methods for each set of poker Rules
pub trait Rules<'a> {
    /// the play_round method takes care of all of the logic required the entire game, for a given variant of poker
    async fn play_round(&mut self, players: Vec<&'a mut Player>) -> Result<(), &'static str>;
}

pub mod five_card_draw;
pub mod seven_card_stud;
pub mod texas_holdem;
