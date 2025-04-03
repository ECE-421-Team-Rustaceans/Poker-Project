use std::io;
use crate::game_type::GameType;

use crate::player::Player;
use crate::{action_option::ActionOption, card::Card};

pub mod cli_input;
pub mod test_input;

/// trait for input handling
pub trait Input {
    fn new() -> Self;
    /// ask user to create a username
    fn request_username(&mut self) -> String;

    /// pick which poker variation to play. 
    fn input_variation(&mut self) -> GameType;

    /// input a list of available actions for the player to choose from
    /// and output an action option that the player has chosen
    fn input_action_options(&mut self, possible_actions: Vec<ActionOption>, player: &Player) -> ActionOption;

    /// ask player to pick an amount to raise by,
    /// returns the amount that the player chose, after validation
    fn request_raise_amount(&mut self, limit: u32, player: &Player) -> u32;

    /// ask player to choose any number of cards from their cards
    /// to be replaced, and return the cards chosen by the player (to be replaced)
    fn request_replace_cards<'a>(&mut self, player: &'a Player) -> Vec<&'a Card>;

    /// show the player their cards (up and down)
    fn display_player_cards_to_player(&self, player: &Player);

    /// Show the player the community cards
    fn display_community_cards_to_player(&self, community_cards: Vec<&Card>, player: &Player);

    /// Show the player the other players' up cards.
    /// if other_players contains the "player", they will be ignored,
    /// that means that the player's up cards will not be shown to themselves,
    /// it is assumed that they will be shown using a different method.
    fn display_other_player_up_cards_to_player(&self, other_players: Vec<&Player>, player: &Player);

    /// display which player's turn it is
    fn display_current_player(&self, player: &Player);

    /// display the winner of a round to all players
    fn announce_winner(&self, winner: &Player, all_players: Vec<&Player>);

    /// display the amount currently in the pot to all players
    fn display_pot(&self, pot_amount: u32, all_players: Vec<&Player>);
}
