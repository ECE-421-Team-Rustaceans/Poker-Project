use super::*;
use std::cmp::min;
use crate::database::db_structs::GameType;

pub struct TestInput;

impl Input for TestInput {
    fn new() -> Self {
        return Self;
    }

    fn input_player(&mut self) -> Vec<String> {
        return vec!["p1".to_string(), "p2".to_string()];
    }

    fn input_variation(&mut self) -> GameType {
        return GameType::FiveCardDraw;
    }

    fn input_action_options(&mut self, possible_actions: Vec<ActionOption>) -> ActionOption {
        return possible_actions[0];
    }

    fn request_raise_amount(&mut self, limit: u32) -> u32 {
        return min(5, limit);
    }
    
    fn request_replace_cards<'a>(&mut self, cards: Vec<&'a Card>) -> Vec<&'a Card> {
        return vec![cards[0]];
    }
    
    fn display_cards(&self, _cards: Vec<&Card>) {
        // do nothing at all
    }
    
    fn display_current_player_index(&self, _player_index: u32) {
        // do nothing at all
    }
}
