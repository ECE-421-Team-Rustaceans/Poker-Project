/// trait containing necessary methods for each set of poker Rules
pub trait Rules {
    /// the play_round method takes care of all of the logic required the entire game, for a given variant of poker
    fn play_round(&mut self);
}

pub mod five_card_draw;
pub mod seven_card_draw;
pub mod kansas_city_lowball;
