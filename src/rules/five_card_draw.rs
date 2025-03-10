use crate::action_history::ActionHistory;
use crate::card::Card;
use crate::deck::Deck;
use crate::input::Input;
use crate::player::Player;
use crate::player_action::PlayerAction;
use crate::pot::Pot;
use super::Rules;
use crate::action_option::ActionOption;
use crate::action::Action;

use std::cmp::min;

pub struct FiveCardDraw<'a, I: Input> {
    players: Vec<&'a mut Player>,
    deck: Deck,
    dealer_position: usize,
    current_player_index: usize,
    action_history: ActionHistory,
    raise_limit: u32,
    input: std::marker::PhantomData<I>,
    pot: Pot
}

impl<'a, I: Input> FiveCardDraw<'a, I> {
    pub fn new(raise_limit: u32) -> FiveCardDraw<'a, I> {
        let deck = Deck::new();
        let dealer_position = 0_usize;
        let current_player_index = 0_usize;
        let action_history = ActionHistory::new();
        let players = Vec::new();
        let pot = Pot::new(&Vec::new());
        return FiveCardDraw {
            players,
            deck,
            dealer_position,
            current_player_index,
            action_history,
            raise_limit,
            input: std::marker::PhantomData,
            pot
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
        let first_blind_player = self.players.get(self.dealer_position).expect("Expected a player at the dealer position, but there was None");
        let second_blind_player = match self.players.get(self.dealer_position+1) {
            Some(player) => player,
            None => {
                self.players.get(0).expect("Expected a non-zero number of players")
            }
        };

        let player_action = PlayerAction::new(&first_blind_player, Action::Ante(1)); // consider not hardcoding in the future
        self.action_history.push(player_action);
        let player_action = PlayerAction::new(&second_blind_player, Action::Ante(2)); // consider not hardcoding in the future
        self.action_history.push(player_action);
    }

    fn play_bet_phase(&mut self, first_phase: bool) {
        // betting starts with the first blind player (player at self.dealer_position)
        self.current_player_index = self.dealer_position;
        let mut last_raise_player_index = self.current_player_index;
        let mut raise_has_occurred = false;
        loop {
            let player: &mut Player = &mut self.players.get_mut(self.current_player_index).expect("Expected a player at this index, but there was None");

            if !(self.action_history.player_has_folded(player) || player.balance() == 0) {
                I::display_current_player_index(self.current_player_index as u32);
                I::display_cards(player.peek_at_cards());

                if first_phase && !raise_has_occurred && self.current_player_index == self.dealer_position+1 {
                    // the big blind can check because they already paid a full bet
                    let action_options = vec![ActionOption::Check, ActionOption::Raise, ActionOption::Fold];
                    let chosen_action_option: ActionOption = I::input_action_options(action_options);

                    let player_raise_limit = min(self.raise_limit, player.balance() as u32);

                    let action = match chosen_action_option {
                        ActionOption::Check => Action::Check,
                        ActionOption::Raise => Action::Raise(I::request_raise_amount(player_raise_limit).try_into().unwrap()),
                        ActionOption::Fold => Action::Fold,
                        _ => panic!("Player managed to select an impossible Action!")
                    };

                    match action {
                        Action::Check => {},
                        Action::Raise(amount) => {
                            last_raise_player_index = self.current_player_index;
                            raise_has_occurred = true;
                            // TODO: update Pot
                            player.bet(amount).unwrap();
                        },
                        Action::Fold => {},
                        _ => panic!("Player managed to perform an impossible Action!")
                    }

                    let player_action = PlayerAction::new(&player, action);
                    self.action_history.push(player_action);
                }
                else {
                    let action_options = vec![ActionOption::Call, ActionOption::Raise, ActionOption::Fold];
                    let chosen_action_option: ActionOption = I::input_action_options(action_options);

                    let current_bet_amount = self.action_history.current_bet_amount();
                    let player_raise_limit = if player.balance() as u32 > current_bet_amount {
                        min(self.raise_limit, player.balance() as u32 - current_bet_amount)
                    } else {
                        0
                    };

                    let action = match chosen_action_option {
                        ActionOption::Call => Action::Call,
                        ActionOption::Raise => Action::Raise(I::request_raise_amount(player_raise_limit).try_into().unwrap()),
                        ActionOption::Fold => Action::Fold,
                        _ => panic!("Player managed to select an impossible Action!")
                    };

                    match action {
                        Action::Call => {
                            // TODO: update Pot
                            if first_phase && !raise_has_occurred && self.current_player_index == self.dealer_position {
                                // the small blind only needs to complete half-bet to stay in, as they already bet a half-bet
                                player.bet(current_bet_amount as usize /2).unwrap();
                            }
                            else {
                                player.bet(current_bet_amount as usize).unwrap();
                            }
                        },
                        Action::Raise(amount) => {
                            last_raise_player_index = self.current_player_index;
                            raise_has_occurred = true;
                            // TODO: update Pot
                            if first_phase && !raise_has_occurred && self.current_player_index == self.dealer_position {
                                // the small blind only needs to complete half-bet to match the bet, as they already bet a half-bet
                                player.bet(amount + current_bet_amount as usize /2).unwrap();
                            }
                            else {
                                player.bet(amount + current_bet_amount as usize).unwrap();
                            }
                        },
                        Action::Fold => {},
                        _ => panic!("Player managed to perform an impossible Action!")
                    }

                    let player_action = PlayerAction::new(&player, action);
                    self.action_history.push(player_action);
                }
            }

            self.current_player_index += 1;
            // wrap the player index around
            if self.current_player_index == self.players.len() {
                self.current_player_index = 0;
            }

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

    fn play_draw_phase(&mut self) {
        // house rules: players may discard as many cards as they wish to draw new replacements
        let start_player_index = self.current_player_index;
        loop {
            let player: &mut Player = self.players.get_mut(self.current_player_index).expect("Expected a player at this index, but there was None");

            if !self.action_history.player_has_folded(player) {
                I::display_current_player_index(self.current_player_index as u32);
                I::display_cards(player.peek_at_cards());

                let action_options = vec![ActionOption::Replace, ActionOption::Check];
                let chosen_action_option: ActionOption = I::input_action_options(action_options);

                let action = match chosen_action_option {
                    ActionOption::Replace => Action::Replace(
                        I::request_replace_cards(
                            player.peek_at_cards()
                        ).iter().map(
                            |card| Box::new((*card).clone())
                        ).collect()
                    ),
                    ActionOption::Check => Action::Check,
                    _ => panic!("Player managed to select an impossible Action!")
                };

                match action {
                    Action::Replace(ref cards_to_replace) => {
                        // take all of the player's cards
                        let mut cards = player.return_cards();
                        // find which cards are to be kept
                        let cards_to_remove: Vec<&Card> = cards.iter().filter(
                            |card| cards_to_replace.iter().any(
                                |card_to_replace|  card_to_replace.as_ref() == *card
                            )
                        ).collect();
                        // remove cards that were chosen for replacement
                        let mut card_indices_to_remove = Vec::new();
                        for (card_index, card) in cards.iter().enumerate() {
                            if cards_to_remove.contains(&card) {
                                card_indices_to_remove.push(card_index);
                            }
                        }
                        card_indices_to_remove.sort();
                        card_indices_to_remove.reverse();
                        card_indices_to_remove.into_iter().for_each(|card_index| self.deck.return_card(cards.remove(card_index)));
                        // deal replacement cards
                        for _ in 0..cards_to_replace.len() {
                            cards.push(self.deck.deal().unwrap());
                        }
                        // give the player back their new cards
                        cards.into_iter().for_each(|card| player.obtain_card(card));
                    },
                    Action::Check => {
                        // do nothing, Player has chosen not to Replace any Cards
                    },
                    _ => panic!("Player managed to perform an impossible Action!")
                }

                let player_action = PlayerAction::new(&player, action);
                self.action_history.push(player_action);
            }

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
        // this is identical to the first phase, in certain variations of five card draw, so it is in our rules
        self.play_bet_phase(false);
    }

    fn showdown(&self) {
        // show to each player everyone's cards (except folded)
        let start_player_index = self.current_player_index;
        let mut current_player_index = self.current_player_index;
        loop {
            let player: &Player = self.players.get(current_player_index).expect("Expected a player at this index, but there was None");

            if !self.action_history.player_has_folded(player) {
                I::display_current_player_index(current_player_index as u32);
                I::display_cards(player.peek_at_cards());
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
        for _ in 0..5 {
            // each player gets 5 cards
            for player in self.players.iter_mut() {
                player.obtain_card(self.deck.deal()?);
            }
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
}

impl<'a, I: Input> Rules<'a> for FiveCardDraw<'a, I> {
    fn play_round(&mut self, players: Vec<&'a mut Player>) -> Result<(), &'static str> {
        if players.len() < 2 {
            return Err("Cannot start a game with less than 2 players");
        }
        self.pot = Pot::new(&players.iter().map(|player| &**player).collect());
        self.action_history = ActionHistory::new();
        assert_eq!(self.deck.size(), 52);
        self.players = players;
        self.increment_dealer_position();
        assert!(self.dealer_position < self.players.len());
        self.current_player_index = self.dealer_position;

        self.play_blinds();
        self.deal_initial_cards().unwrap();
        self.play_phase_one();
        self.play_draw_phase();
        self.play_phase_two();
        self.showdown();

        self.return_player_cards();

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::input::test_input::TestInput;

    use super::*;

    #[test]
    fn new() {
        let five_card_draw = FiveCardDraw::<TestInput>::new(1000);

        assert_eq!(five_card_draw.deck.size(), 52);
        assert_eq!(five_card_draw.dealer_position, 0);
        assert_eq!(five_card_draw.current_player_index, 0);
        assert_eq!(five_card_draw.action_history.current_bet_amount(), 0);
        assert_eq!(five_card_draw.action_history.players().len(), 0);
        assert_eq!(five_card_draw.players.len(), 0);
    }

    #[test]
    fn play_round() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];

        five_card_draw.play_round(players.iter_mut().map(|player| player).collect()).unwrap();

        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];

        five_card_draw.play_round(players.iter_mut().map(|player| player).collect()).unwrap();

        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];

        five_card_draw.play_round(players.iter_mut().map(|player| player).collect()).unwrap();
    }

    #[test]
    fn try_play_round_one_player() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7())
        ];

        assert!(five_card_draw.play_round(players.iter_mut().map(|player| player).collect()).is_err());
    }
}
