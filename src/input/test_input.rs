use super::*;
use std::cmp::min;

pub struct TestInput;

impl Input for TestInput {
    fn input_player() -> Vec<String> {
        return vec!["p1".to_string(), "p2".to_string()];
    }

    fn input_variation() -> usize {
        return 1;
    }

    fn input_action_options(possible_actions: Vec<ActionOption>) -> ActionOption {
        return possible_actions[0];
    }

    fn input_action_option(action_option: ActionOption, limit: u32) -> u32 {
        return 1;
    }
    
    fn request_raise_amount(limit: u32) -> u32 {
        return min(5, limit);
    }
    
    fn request_replace_cards(cards: Vec<&Card>) -> Vec<&Card> {
        return vec![cards[0]];
    }
    
    fn display_cards(cards: Vec<&Card>) {
        // do nothing at all
    }
    
    fn display_current_player_index(player_index: u32) {
        // do nothing at all
    }
}
