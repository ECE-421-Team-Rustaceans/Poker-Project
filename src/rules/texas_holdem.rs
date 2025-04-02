use crate::action_history::ActionHistory;
use crate::card::Card;
use crate::deck::Deck;
use crate::hand_rank::Hand;
use crate::input::Input;
use crate::player::Player;
use crate::player_action::PlayerAction;
use crate::pot::Pot;
use super::Rules;
use crate::action_option::ActionOption;
use crate::action::Action;

use std::cmp::min;

pub struct TexasHoldem<'a, I: Input> {
    players: Vec<&'a mut Player>,
    deck: Deck,
    dealer_position: usize,
    current_player_index: usize,
    action_history: ActionHistory,
    raise_limit: u32,
    bring_in: u32,
    input: I,
    pot: Pot,
    community_cards: Vec<Card>
}

impl<'a, I: Input> TexasHoldem<'a, I> {
    pub fn new(raise_limit: u32, bring_in: u32) -> TexasHoldem<'a, I> {
        let deck = Deck::new();
        let dealer_position = 0_usize;
        let current_player_index = 0_usize;
        let action_history = ActionHistory::new();
        let players = Vec::new();
        let pot = Pot::new(&Vec::new());
        let community_cards = Vec::new();
        return TexasHoldem {
            players,
            deck,
            dealer_position,
            current_player_index,
            action_history,
            raise_limit,
            bring_in,
            input: I::new(),
            pot,
            community_cards
        };
    }

    fn number_of_players_all_in(&self) -> usize {
        return self.players.iter().filter(|player| player.balance() == 0).count();
    }

    fn increment_dealer_position(&mut self) {
        self.dealer_position += 1;
        if self.dealer_position >= self.players.len() {
            self.dealer_position = 0;
        }
    }

    fn increment_player_index(&mut self) {
        self.current_player_index += 1;
        // wrap the player index around
        if self.current_player_index == self.players.len() {
            self.current_player_index = 0;
        }
    }

    fn play_blinds(&mut self) {
        // the first and second players after the dealer must bet blind
        let first_blind_player = self.players.get_mut(self.dealer_position).expect("Expected a player at the dealer position, but there was None");
        let player_action = PlayerAction::new(&first_blind_player, Action::Ante(1)); // consider not hardcoding in the future
        self.action_history.push(player_action);
        first_blind_player.bet(1).unwrap();
        self.increment_player_index();

        let second_blind_player = match self.players.get_mut(self.dealer_position+1) {
            Some(player) => player,
            None => {
                self.players.get_mut(0).expect("Expected a non-zero number of players")
            }
        };
        let player_action = PlayerAction::new(&second_blind_player, Action::Ante(2)); // consider not hardcoding in the future
        self.action_history.push(player_action);
        second_blind_player.bet(2).unwrap();
        self.increment_player_index();
    }

    fn play_bet_phase(&mut self, is_first_bet_phase: bool) {
        // for the first bet phase, the correct player to start at has been set by the bring in method.
        // for subsequent bet phases, the starting player is the one with the up cards that make the best poker hand.
        if !is_first_bet_phase {
            self.current_player_index = self.find_player_with_best_up_card_hand();
        }
        let mut last_raise_player_index = self.current_player_index;
        let mut raise_has_occurred = false;
        loop {
            if self.action_history.number_of_players_folded()+1 == (self.players.len() as u32) {
                // all players have folded but one, remaining player automatically wins
                break;
            }

            let player: &mut Player = &mut self.players.get_mut(self.current_player_index).expect("Expected a player at this index, but there was None");

            if !(self.action_history.player_has_folded(player) || player.balance() == 0) {
                self.input.display_current_player_index(self.current_player_index as u32);
                self.input.display_cards(player.peek_at_cards());
                self.input.display_cards(self.community_cards.iter().collect());

                if !raise_has_occurred && self.action_history.current_bet_amount() == self.action_history.player_current_bet_amount(player) {
                    // the bring in player can check because they already paid a full bet, and on subsequent phases, everyone can check if nobody raises
                    let action_options = vec![ActionOption::Check, ActionOption::Raise, ActionOption::Fold];
                    let chosen_action_option: ActionOption = self.input.input_action_options(action_options);

                    let player_raise_limit = min(self.raise_limit, player.balance() as u32);

                    let action = match chosen_action_option {
                        ActionOption::Check => Action::Check,
                        ActionOption::Raise => Action::Raise(self.input.request_raise_amount(player_raise_limit).try_into().unwrap()),
                        ActionOption::Fold => Action::Fold,
                        _ => panic!("Player managed to select an impossible Action!")
                    };

                    match action {
                        Action::Check => {},
                        Action::Raise(raise_amount) => {
                            last_raise_player_index = self.current_player_index;
                            raise_has_occurred = true;
                            // TODO: update Pot
                            let bet_amount = self.action_history.current_bet_amount() + raise_amount as u32 - self.action_history.player_current_bet_amount(player);
                            player.bet(bet_amount as usize).unwrap();
                        },
                        Action::Fold => {},
                        _ => panic!("Player managed to perform an impossible Action!")
                    }

                    let player_action = PlayerAction::new(&player, action);
                    self.action_history.push(player_action);
                }
                else {
                    let action_options = vec![ActionOption::Call, ActionOption::Raise, ActionOption::Fold];
                    let chosen_action_option: ActionOption = self.input.input_action_options(action_options);

                    let current_bet_amount = self.action_history.current_bet_amount();
                    let player_raise_limit = if player.balance() as u32 > current_bet_amount {
                        min(self.raise_limit, player.balance() as u32 - current_bet_amount)
                    } else {
                        0
                    };

                    let action = match chosen_action_option {
                        ActionOption::Call => Action::Call,
                        ActionOption::Raise => Action::Raise(self.input.request_raise_amount(player_raise_limit).try_into().unwrap()),
                        ActionOption::Fold => Action::Fold,
                        _ => panic!("Player managed to select an impossible Action!")
                    };

                    match action {
                        Action::Call => {
                            // TODO: update Pot
                            let bet_amount = self.action_history.current_bet_amount() - self.action_history.player_current_bet_amount(player);
                            player.bet(bet_amount as usize).unwrap();
                        },
                        Action::Raise(raise_amount) => {
                            last_raise_player_index = self.current_player_index;
                            raise_has_occurred = true;
                            // TODO: update Pot
                            let bet_amount = self.action_history.current_bet_amount() + raise_amount as u32 - self.action_history.player_current_bet_amount(player);
                            player.bet(bet_amount as usize).unwrap();
                        },
                        Action::Fold => {},
                        _ => panic!("Player managed to perform an impossible Action!")
                    }

                    let player_action = PlayerAction::new(&player, action);
                    self.action_history.push(player_action);
                }
            }

            self.increment_player_index();

            if self.current_player_index == last_raise_player_index {
                // the next player is the player who last raised,
                // which means that all bets have been matched,
                // and it is time to move on to the next phase
                break;
            }
        }
    }

    fn play_phase_one(&mut self) {
        self.play_bet_phase(true);
    }

    fn play_phase_two(&mut self) {
        self.play_bet_phase(false);
    }

    fn play_phase_three(&mut self) {
        self.play_bet_phase(false);
    }

    fn play_phase_four(&mut self) {
        self.play_bet_phase(false);
    }

    fn play_phase_five(&mut self) {
        self.play_bet_phase(false);
    }

    fn showdown(&self) {
        // show to each player everyone's cards (except folded)
        let start_player_index = self.current_player_index;
        let mut current_player_index = self.current_player_index;
        loop {
            let player: &Player = self.players.get(current_player_index).expect("Expected a player at this index, but there was None");

            if !self.action_history.player_has_folded(player) {
                self.input.display_current_player_index(current_player_index as u32);
                self.input.display_cards(player.peek_at_cards());
                self.input.display_cards(self.community_cards.iter().collect());
            }

            current_player_index += 1;
            // wrap the player index around
            if current_player_index == self.players.len() {
                current_player_index = 0;
            }

            if current_player_index == start_player_index {
                // one turn has been completed for each player,
                // this marks the end of the draw phase
                break;
            }
        }
    }

    fn deal_initial_cards(&mut self) -> Result<(), String> {
        // each player is dealt two cards face down
        for _ in 0..2 {
            self.deal_down_cards();
        }
        return Ok(());
    }

    /// Deal 3 community cards
    fn deal_flop_cards(&mut self) -> Result<(), String> {
        for _ in 0..3 {
            self.deal_community_card()?;
        }
        return Ok(());
    }

    fn deal_community_card(&mut self) -> Result<(), String> {
        if self.pot.number_of_players_folded()+1 == (self.players.len() as u32) {
            // all players have folded but one
            return Ok(());
        }
        if self.number_of_players_all_in()+1 == self.players.len() {
            // all players are all in but one
            return Ok(());
        }
        self.community_cards.push(self.deck.deal(true)?);
        return Ok(());
    }

    /// each non-folded player is dealt one card face up
    fn deal_up_cards(&mut self) -> Result<(), String> {
        let remaining_players = self.players.iter_mut()
            .filter(|player| !self.action_history.player_has_folded(player));
        for player in remaining_players {
            player.obtain_card(self.deck.deal(true)?);
        }
        return Ok(());
    }

    /// each non-folded player is dealt one card face down
    fn deal_down_cards(&mut self) -> Result<(), String> {
        let remaining_players = self.players.iter_mut()
            .filter(|player| !self.action_history.player_has_folded(player));
        for player in remaining_players {
            player.obtain_card(self.deck.deal(false)?);
        }
        return Ok(());
    }

    fn return_player_cards(&mut self) {
        for player in self.players.iter_mut() {
            let cards = player.return_cards();
            for card in cards {
                self.deck.return_card(card);
            }
        }
    }

    fn return_community_cards(&mut self) {
        while let Some(card) = self.community_cards.pop() {
            self.deck.return_card(card);
        }
        assert_eq!(self.community_cards.len(), 0);
    }
}

impl<'a, I: Input> Rules<'a> for TexasHoldem<'a, I> {
    fn play_round(&mut self, players: Vec<&'a mut Player>) -> Result<(), &'static str> {
        if players.len() < 2 {
            return Err("Cannot start a game with less than 2 players");
        }
        self.pot = Pot::new(&players.iter().map(|player| &**player).collect());
        self.action_history = ActionHistory::new();
        assert_eq!(self.community_cards.len(), 0);
        assert_eq!(self.deck.size(), 52);
        self.players = players;
        self.increment_dealer_position();
        assert!(self.dealer_position < self.players.len());
        self.current_player_index = self.dealer_position;

        self.deal_initial_cards().unwrap();
        self.play_blinds();
        self.play_phase_one();
        self.deal_flop_cards().unwrap();
        self.play_phase_two();
        self.deal_community_card().unwrap();
        self.play_phase_three();
        self.deal_community_card().unwrap();
        self.play_phase_four();
        self.showdown();

        self.return_player_cards();
        self.return_community_cards();

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::input::test_input::TestInput;

    use super::*;

}
