use std::io;
use crate::database::db_structs::GameType;

use crate::{action_option::ActionOption, card::Card};

pub mod cli_input;
pub mod test_input;

/// trait for input handling
pub trait Input {
    /// input handling for players, 
    /// returns a list of gamer names
    fn input_player() -> Vec<String>;

    /// for user input to pick which poker variation to play. 
    /// will return a usize from 1-3, which correspond to different poker variations
    fn input_variation() -> GameType;

    /// input a list of available actions for the user to choose from
    /// and output a action option that the user has chosen
    fn input_action_options(possible_actions: Vec<ActionOption>) -> ActionOption;

    /// ask the user to pick an amount to raise by,
    /// returns the amount that the user chose, after validation
    fn request_raise_amount(limit: u32) -> u32;

    /// ask the user to choose any number of cards from the provided cards
    /// to be replaced, and return the cards chosen by the user (to be replaced)
    fn request_replace_cards(cards: Vec<&Card>) -> Vec<&Card>;

    /// show the user their cards
    fn display_cards(cards: Vec<&Card>);

    /// display which player's turn it is
    fn display_current_player_index(player_index: u32);
}
