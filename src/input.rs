use std::io;

use crate::action_option::ActionOption;

pub mod cli_input;

/// trait for input handling
pub trait Input {
    /// input handling for players, 
    /// returns a list of gamer names
    fn input_player(&self) -> Vec<String>;

    /// for user input to pick which poker variation to play. 
    /// will return a usize from 1-3, which correspond to different poker variations
    fn input_variation(&self) -> usize;

    /// input a list of available actions for the user to choose from
    /// and output a action option that the user has chosen
    fn input_action_options(&self, possible_actions: Vec<ActionOption>) -> ActionOption;

    /// action option to action with the number
    fn input_action_option(&self, action_option: ActionOption, limit: u32) -> u32;
}
