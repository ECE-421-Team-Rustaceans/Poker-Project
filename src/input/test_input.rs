use super::*;
use crate::game_type::GameType;

/// TestInput is an implementation of the Input trait along with some additional specific methods.
/// It allows setting specific inputs that will be performed, ahead of time, for testing purposes.
/// Essentially, this is a driver for tests that is used in place of user input, and allows
/// testing game rules with a specific sequence of events.
/// The implementations of Input trait methods that simply display to the user do nothing,
/// and the methods that return something pop from a preset vector of inputs to return.
/// Setter methods (not part of the Input trait) are provided to set the actions that
/// will be performed in the order they occur.
/// This struct should only be used for testing purposes.
pub struct TestInput {
    player_names: Vec<String>,
    game_variation: Option<GameType>,
    action_option_selections: Vec<ActionOption>,
    raise_amounts: Vec<u32>,
    card_replace_selections: Vec<Vec<usize>>
}

impl Input for TestInput {
    fn new() -> Self {
        return TestInput {
            player_names: Vec::new(),
            game_variation: None,
            action_option_selections: Vec::new(),
            raise_amounts: Vec::new(),
            card_replace_selections: Vec::new()
        };
    }

    fn request_username(&mut self) -> String {
        return self.player_names.pop().unwrap();
    }

    fn input_variation(&mut self) -> GameType {
        return self.game_variation.clone().unwrap();
    }

    fn input_action_options(&mut self, _possible_actions: Vec<ActionOption>, _player: &Player) -> ActionOption {
        return self.action_option_selections.pop().unwrap();
    }

    fn request_raise_amount(&mut self, _limit: u32, _player: &Player) -> u32 {
        return self.raise_amounts.pop().unwrap();
    }

    fn request_replace_cards<'a>(&mut self, player: &'a Player) -> Vec<&'a Card> {
        let cards = player.peek_at_cards();
        let card_indices = self.card_replace_selections.pop().unwrap();
        return card_indices.into_iter().map(|card_index| *cards.get(card_index).unwrap()).collect();
    }

    fn display_player_cards_to_player(&self, _player: &Player) {
        // do nothing at all
    }

    fn display_community_cards_to_player(&self, _community_cards: Vec<&Card>, _player: &Player) {
        // do nothing at all
    }

    fn display_other_player_up_cards_to_player(&self, _other_players: Vec<&Player>, _player: &Player) {
        // do nothing at all
    }

    fn display_current_player(&self, _player: &Player) {
        // do nothing at all
    }

    fn announce_winner(&self, _winner: Vec<&Player>, _all_players: Vec<&Player>) {
        // do nothing at all
    }

    fn display_pot(&self, _pot_amount: u32, _all_players: Vec<&Player>) {
        // do nothing at all
    }
}

impl TestInput {
    pub fn set_player_names(&mut self, player_names: Vec<String>) {
        self.player_names = player_names;
        self.player_names.reverse();
    }

    pub fn set_game_variation(&mut self, game_variation: GameType) {
        self.game_variation = Some(game_variation);
    }

    pub fn set_action_option_selections(&mut self, action_option_selections: Vec<ActionOption>) {
        self.action_option_selections = action_option_selections;
        self.action_option_selections.reverse(); // reverse since we pop from the end for performance reasons
    }

    pub fn set_raise_amounts(&mut self, raise_amounts: Vec<u32>) {
        self.raise_amounts = raise_amounts;
        self.raise_amounts.reverse(); // reverse since we pop from the end for performance reasons
    }

    pub fn set_card_replace_selections(&mut self, card_replace_selections: Vec<Vec<usize>>) {
        self.card_replace_selections = card_replace_selections;
        self.card_replace_selections.reverse(); // reverse since we pop from the end for performance reasons
    }
}
