use super::*;
use crate::game_type::GameType;

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

    fn input_player(&mut self) -> Vec<String> {
        return self.player_names.drain(..).collect();
    }

    fn input_variation(&mut self) -> GameType {
        return self.game_variation.clone().unwrap();
    }

    fn input_action_options(&mut self, _possible_actions: Vec<ActionOption>) -> ActionOption {
        return self.action_option_selections.pop().unwrap();
    }

    fn request_raise_amount(&mut self, _limit: u32) -> u32 {
        return self.raise_amounts.pop().unwrap();
    }
    
    fn request_replace_cards<'a>(&mut self, cards: Vec<&'a Card>) -> Vec<&'a Card> {
        let card_indices = self.card_replace_selections.pop().unwrap();
        return card_indices.into_iter().map(|card_index| *cards.get(card_index).unwrap()).collect();
    }

    fn display_player_cards_to_player(&self, player: Player) {
        // do nothing at all
    }

    fn display_community_cards_to_player(&self, community_cards: Vec<&Card>, player: Player) {
        // do nothing at all
    }

    fn display_other_player_up_cards_to_player(&self, other_players: Vec<&Player>, player: Player) {
        // do nothing at all
    }

    fn display_current_player(&self, player: Player) {
        // do nothing at all
    }
}

impl TestInput {
    pub fn set_player_names(&mut self, player_names: Vec<String>) {
        self.player_names = player_names;
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
