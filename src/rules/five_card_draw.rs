use crate::deck::Deck;
use crate::player::Player;
use super::Rules;
use crate::action_option::ActionOption;
use crate::action::Action;

pub struct FiveCardDraw<'a> {
    players: Vec<&'a Player>,
    deck: Deck,
    dealer_position: usize,
    current_player_index: usize
}

impl<'a> FiveCardDraw<'a> {
    fn new(players: Vec<&Player>) -> FiveCardDraw {
        let deck = Deck::new();
        let dealer_position = 0_usize;
        let current_player_index = 0_usize;
        return FiveCardDraw {
            players,
            deck,
            dealer_position,
            current_player_index
        };
    }

    fn increment_dealer_position(&mut self) {
        self.dealer_position += 1;
        if self.dealer_position == self.players.len() {
            self.dealer_position = 0;
        }
    }

    fn play_blinds(&mut self) {
        // the first and second players after the dealer must bet blind
        let mut first_blind_player = *self.players.get(self.dealer_position).expect("Expected a player at the dealer position, but there was None");
        let mut second_blind_player = match self.players.get(self.dealer_position+1) {
            Some(player) => *player,
            None => {
                self.players.get(0).expect("Expected a non-zero number of players")
            }
        };

        first_blind_player.play_turn(ActionOption::Ante);
        second_blind_player.play_turn(ActionOption::Ante);
    }

    fn play_phase_one(&mut self) {
        // betting on this phase starts with the first blind player (player at self.dealer_position)
        self.current_player_index = self.dealer_position;
        let mut all_bets_matched = false;
        loop {
            let mut player = *self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None");
            let action_options = [ActionOption::Raise, ActionOption::Check, ActionOption::Fold, ActionOption::Call, ActionOption::AllIn];
            let player_action: Action = player.play_turn(action_options); // TODO: pass possible actions to player
            // TODO: process player action

            self.current_player_index += 1;
            // wrap the player index around
            if self.current_player_index == self.players.len() {
                self.current_player_index = 0;
            }

            if all_bets_matched {
                break;
            }
        }
    }

    fn play_draw_phase(&mut self) {
        // house rules: players may discard as many cards as they wish to draw new replacements
        // the exception is if there are not enough cards left in the deck to do so
        let start_player_index = self.current_player_index;
        loop {
            let mut player = *self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None");
            let action_options = [ActionOption::Replace];
            let player_action: Action = player.play_turn(action_options); // TODO: pass possible action (draw) to player
            // TODO: process player action

            self.current_player_index += 1;
            // wrap the player index around
            if self.current_player_index == self.players.len() {
                self.current_player_index = 0;
            }

            if self.current_player_index == start_player_index {
                // one turn has been completed for each player,
                // this marks the end of the draw phase
                break;
            }
        }
    }

    fn play_phase_two(&mut self) {
        // betting on this phase starts with the player at the dealer position (or the next one that hasn't folded yet)
        // the second round does not have raises, only checks, bets and folds, so there is only one loop around the table
        let start_player_index = self.current_player_index;
        loop {
            let mut player = *self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None");
            let action_options = [ActionOption::Check, ActionOption::Bet, ActionOption::Fold];
            let player_action: Action = player.play_turn(action_options); // TODO: pass possible actions to player
            // TODO: process player action

            self.current_player_index += 1;
            // wrap the player index around
            if self.current_player_index == self.players.len() {
                self.current_player_index = 0;
            }

            if self.current_player_index == start_player_index {
                // one turn has been completed for each player,
                // this marks the end of the second phase of betting
                break;
            }
        }
    }

    fn deal_initial_cards(&mut self) -> Result<(), String> {
        for _ in 0..5 {
            // each player gets 5 cards
            for player in self.players {
                player.obtain_card(self.deck.deal()?);
            }
        }
        return Ok(());
    }
}

impl<'a> Rules for FiveCardDraw<'a> {
    fn play_round(&mut self, players: Vec<&Player>) {
        self.play_blinds();
        self.deal_initial_cards();
        self.play_phase_one();
        self.play_draw_phase();
        self.play_phase_two();
        self.increment_dealer_position();
    }
}

// FIXME: need to account for players folding... not really accounted for right now
