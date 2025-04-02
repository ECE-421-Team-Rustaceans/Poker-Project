use std::io;
use crate::game_type::GameType;

use crate::player::Player;
use crate::{action_option::ActionOption, card::Card};

pub mod cli_input;
pub mod test_input;

/// trait for input handling
pub trait Input {
    fn new() -> Self;
    /// input handling for players, 
    /// returns a list of gamer names
    fn input_player(&mut self) -> Vec<String>;

    /// for user input to pick which poker variation to play. 
    fn input_variation(&mut self) -> GameType;

    /// input a list of available actions for the user to choose from
    /// and output an action option that the user has chosen
    fn input_action_options(&mut self, possible_actions: Vec<ActionOption>) -> ActionOption;

    /// ask the user to pick an amount to raise by,
    /// returns the amount that the user chose, after validation
    fn request_raise_amount(&mut self, limit: u32) -> u32;

    /// ask the user to choose any number of cards from the provided cards
    /// to be replaced, and return the cards chosen by the user (to be replaced)
    fn request_replace_cards<'a>(&mut self, cards: Vec<&'a Card>) -> Vec<&'a Card>;

    /// show the player their cards (up and down)
    fn display_player_cards_to_player(&self, player: Player);

    /// Show the player the community cards
    fn display_community_cards_to_player(&self, community_cards: Vec<&Card>, player: Player);

    /// Show the player the other players' up cards.
    /// if other_players contains the "player", they will be ignored,
    /// that means that the player's up cards will not be shown to themselves,
    /// it is assumed that they will be shown using a different method.
    fn display_other_player_up_cards_to_player(&self, other_players: Vec<&Player>, player: Player);

    /// display which player's turn it is
    fn display_current_player_index(&self, player: Player);
}
