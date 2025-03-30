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
    input: I,
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
            input: I::new(),
            pot
        };
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

    fn play_bet_phase(&mut self) {
        // betting starts with the first blind player (player at self.dealer_position)
        self.current_player_index = self.dealer_position;
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

                if !raise_has_occurred && self.action_history.current_bet_amount() == self.action_history.player_current_bet_amount(player) {
                    // the big blind can check because they already paid a full bet, and on the second round, everyone can check if nobody raises
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
        self.play_bet_phase();
    }

    fn play_draw_phase(&mut self) {
        // house rules: players may discard as many cards as they wish to draw new replacements
        let start_player_index = self.current_player_index;
        loop {
            if self.action_history.number_of_players_folded()+1 == (self.players.len() as u32) {
                // all players have folded but one, remaining player automatically wins
                break;
            }

            let player: &mut Player = self.players.get_mut(self.current_player_index).expect("Expected a player at this index, but there was None");

            if !self.action_history.player_has_folded(player) {
                self.input.display_current_player_index(self.current_player_index as u32);
                self.input.display_cards(player.peek_at_cards());

                let action_options = vec![ActionOption::Replace, ActionOption::Check];
                let chosen_action_option: ActionOption = self.input.input_action_options(action_options);

                let action = match chosen_action_option {
                    ActionOption::Replace => Action::Replace(
                        self.input.request_replace_cards(
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
                        if cards_to_replace.len() > 0 {
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
                                cards.push(self.deck.deal(false).unwrap());
                            }
                            // give the player back their new cards
                            cards.into_iter().for_each(|card| player.obtain_card(card));
                        }
                    },
                    Action::Check => {
                        // do nothing, Player has chosen not to Replace any Cards
                    },
                    _ => panic!("Player managed to perform an impossible Action!")
                }

                let player_action = PlayerAction::new(&player, action);
                self.action_history.push(player_action);
            }

            self.increment_player_index();

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
        self.play_bet_phase();
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
                player.obtain_card(self.deck.deal(false)?);
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
    fn try_play_round_one_player() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7())
        ];

        assert!(five_card_draw.play_round(players.iter_mut().map(|player| player).collect()).is_err_and(|err| err == "Cannot start a game with less than 2 players"));
    }

    #[test]
    fn increment_dealer_position() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();
        assert_eq!(five_card_draw.dealer_position, 0);
        five_card_draw.increment_dealer_position();
        assert_eq!(five_card_draw.dealer_position, 1);
        five_card_draw.increment_dealer_position();
        assert_eq!(five_card_draw.dealer_position, 0);
        five_card_draw.players.pop();
        five_card_draw.increment_dealer_position();
        assert_eq!(five_card_draw.dealer_position, 0);
    }

    #[test]
    fn increment_player_index() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();
        assert_eq!(five_card_draw.current_player_index, 0);
        five_card_draw.increment_player_index();
        assert_eq!(five_card_draw.current_player_index, 1);
        five_card_draw.increment_player_index();
        assert_eq!(five_card_draw.current_player_index, 0);
        five_card_draw.players.pop();
        five_card_draw.increment_player_index();
        assert_eq!(five_card_draw.current_player_index, 0);
    }

    #[test]
    fn play_blinds() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();
        five_card_draw.play_blinds();
        assert_eq!(five_card_draw.action_history.current_bet_amount(), 2);
        assert_eq!(five_card_draw.current_player_index, 2);
        assert_eq!(players.get(0).unwrap().balance(), initial_balance-1);
        assert_eq!(players.get(1).unwrap().balance(), initial_balance-2);
    }

    #[test]
    fn deal_initial_cards() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();
        five_card_draw.deal_initial_cards().unwrap();
        let mut cards = Vec::new();
        for mut player in players {
            assert_eq!(player.peek_at_cards().len(), 5);
            let temp_cards = player.return_cards();
            // make sure that cards didn't somehow get duplicated, that cards are in fact unique
            for card in temp_cards.iter() {
                assert!(!cards.contains(card));
            }
            cards.extend(temp_cards);
        }
    }

    #[test]
    fn play_phase_one_check_only() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Call,
            ActionOption::Check,
            ActionOption::Call,
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            // no raises to perform as all actions are checks or calls
        ]);

        five_card_draw.play_blinds();
        five_card_draw.play_phase_one();

        assert_eq!(five_card_draw.action_history.current_bet_amount(), 2);
        assert_eq!(five_card_draw.dealer_position, 0);
        assert_eq!(five_card_draw.current_player_index, 0);
        for player in players.into_iter() {
            assert_eq!(player.balance(), initial_balance-2);
        }
    }

    #[test]
    fn play_phase_one_with_raises() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Call,
            ActionOption::Check,
            ActionOption::Raise,
            ActionOption::Call,
            ActionOption::Raise,
            ActionOption::Call,
            ActionOption::Call
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks, calls or raises
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            10,
            15
        ]);

        five_card_draw.play_blinds();
        five_card_draw.play_phase_one();

        assert_eq!(five_card_draw.action_history.current_bet_amount(), 27);
        assert_eq!(five_card_draw.dealer_position, 0);
        assert_eq!(five_card_draw.current_player_index, 1);
        for player in players.into_iter() {
            assert_eq!(player.balance(), initial_balance-27);
        }
    }

    #[test]
    fn play_phase_one_with_folds() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Fold, // player 0 folds
            ActionOption::Check,
            ActionOption::Raise,
            ActionOption::Raise,
            ActionOption::Fold // player 2 folds, only player 1 remains
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks, calls, raises or folds
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            10,
            15
        ]);

        five_card_draw.play_blinds();
        five_card_draw.play_phase_one();

        assert_eq!(five_card_draw.action_history.current_bet_amount(), 27);
        assert_eq!(five_card_draw.dealer_position, 0);
        assert_eq!(players.get(0).unwrap().balance(), initial_balance-1); // small blind then fold
        assert_eq!(players.get(1).unwrap().balance(), initial_balance-27); // the only remaining player, they have the max bet
        assert_eq!(players.get(2).unwrap().balance(), initial_balance-12); // raise to 12 then fold
    }

    #[test]
    fn play_all_folds_auto_win() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Fold,
            ActionOption::Fold,
            ActionOption::Raise // this should not be allowed to happen as this player should automatically win
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are folds
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            100
        ]);

        five_card_draw.play_blinds();
        five_card_draw.deal_initial_cards().unwrap();
        five_card_draw.play_phase_one();
        five_card_draw.play_draw_phase();
        five_card_draw.play_phase_two();
        five_card_draw.showdown();

        assert_eq!(five_card_draw.action_history.current_bet_amount(), 2);
        assert_eq!(players.get(0).unwrap().balance(), initial_balance-1); // small blind and fold
        assert_eq!(players.get(1).unwrap().balance(), initial_balance-2); // big blind and fold
        assert_eq!(players.get(2).unwrap().balance(), initial_balance); // fold, should not have the opportunity to raise
    }

    #[test]
    fn play_draw_phase_draw_various_amounts_of_cards() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            // phase 1
            ActionOption::Call,
            ActionOption::Check,
            ActionOption::Call,
            // draw phase
            ActionOption::Check,
            ActionOption::Replace,
            ActionOption::Replace
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            vec![], // replace no cards
            vec![0, 1, 2, 3, 4] // replace all cards
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            // no raises to perform as all actions are checks
        ]);

        five_card_draw.play_blinds();
        five_card_draw.deal_initial_cards().unwrap();

        let mut initial_player_cards: Vec<Vec<Card>> = Vec::new();
        for player in five_card_draw.players.iter() {
            initial_player_cards.push(player.peek_at_cards().iter().map(|&card| card.clone()).collect());
        }

        five_card_draw.play_phase_one();
        five_card_draw.play_draw_phase();

        assert_eq!(five_card_draw.action_history.current_bet_amount(), 2);
        assert_eq!(five_card_draw.dealer_position, 0);
        assert_eq!(five_card_draw.current_player_index, 0);
        for player in five_card_draw.players.iter() {
            assert_eq!(player.balance(), initial_balance-2);
            assert_eq!(player.peek_at_cards().len(), 5);
        }
        for (card_index, card) in five_card_draw.players.get(0).unwrap().peek_at_cards().iter().enumerate() {
            assert_eq!(*card, initial_player_cards.get(0).unwrap().get(card_index).unwrap());
        }
        for (card_index, card) in five_card_draw.players.get(1).unwrap().peek_at_cards().iter().enumerate() {
            assert_eq!(*card, initial_player_cards.get(1).unwrap().get(card_index).unwrap());
        }
        for (card_index, card) in five_card_draw.players.get(2).unwrap().peek_at_cards().iter().enumerate() {
            if *card != initial_player_cards.get(2).unwrap().get(card_index).unwrap() {
                break;
            }
            if card_index == 4 {
                // last card and they have all matched so far, something is wrong or we got insanely unlucky...
                panic!();
            }
        }
    }

    #[test]
    fn play_full_round_all_checks_and_calls() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        five_card_draw.players = players.iter_mut().map(|player| player).collect();

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Call, // phase 1
            ActionOption::Check,
            ActionOption::Call,
            ActionOption::Check, // draw phase
            ActionOption::Check,
            ActionOption::Check,
            ActionOption::Check, // phase 2
            ActionOption::Check,
            ActionOption::Check
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            // no raises as all actions are checks or calls
        ]);

        five_card_draw.play_blinds();
        five_card_draw.deal_initial_cards().unwrap();
        five_card_draw.play_phase_one();
        five_card_draw.play_draw_phase();
        five_card_draw.play_phase_two();
        five_card_draw.showdown();

        assert_eq!(five_card_draw.action_history.current_bet_amount(), 2);
        assert_eq!(players.get(0).unwrap().balance(), initial_balance-2); // call to 2 and check the rest
        assert_eq!(players.get(1).unwrap().balance(), initial_balance-2); // big blind 2 and check the rest
        assert_eq!(players.get(2).unwrap().balance(), initial_balance-2); // call to 2 and check the rest
    }
}
