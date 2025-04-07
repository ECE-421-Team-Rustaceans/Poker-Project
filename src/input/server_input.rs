use uuid::Uuid;

use super::*;

/// Implementation of the Input trait for server-client interaction
/// Each method that requires user input must first send the new data to the client,
/// and then wait until the client responds (or a timeout) before returning.
/// The display methods (the ones that don't return anything) don't need to
/// wait for any response from the client.
#[derive(Clone)]
pub struct ServerInput;

impl Input for ServerInput {
    fn new() -> Self {
        return Self;
    }

    fn request_username(&mut self) -> String {
        todo!()
    }

    fn input_variation(&mut self) -> GameType {
        todo!()
    }

    fn input_action_options(&mut self, possible_actions: Vec<ActionOption>, player: &Player) -> ActionOption {
        todo!()
    }

    fn request_raise_amount(&mut self, limit: u32, player: &Player) -> u32 {
        todo!()
    }

    fn request_replace_cards<'a>(&mut self, player: &'a Player) -> Vec<&'a Card> {
        todo!()
    }

    fn display_player_cards_to_player(&self, player: &Player) {
        todo!()
    }

    fn display_community_cards_to_player(&self, community_cards: Vec<&Card>, player: &Player) {
        todo!()
    }

    fn display_other_player_up_cards_to_player(&self, other_players: Vec<&Player>, player: &Player) {
        todo!()
    }

    fn display_current_player(&self, player: &Player) {
        todo!()
    }

    fn announce_winner(&self, winner: Vec<&Player>, all_players: Vec<&Player>) {
        todo!()
    }

    fn display_pot(&self, pot_amount: u32, all_players: Vec<&Player>) {
        todo!()
    }
}
