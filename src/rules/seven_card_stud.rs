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

pub struct SevenCardStud<'a, I: Input> {
    players: Vec<&'a mut Player>,
    deck: Deck,
    dealer_position: usize,
    current_player_index: usize,
    action_history: ActionHistory,
    raise_limit: u32,
    bring_in: u32,
    input: I,
    pot: Pot
}

impl<'a, I: Input> SevenCardStud<'a, I> {
    pub fn new(raise_limit: u32, bring_in: u32) -> SevenCardStud<'a, I> {
        let deck = Deck::new();
        let dealer_position = 0_usize;
        let current_player_index = 0_usize;
        let action_history = ActionHistory::new();
        let players = Vec::new();
        let pot = Pot::new(&Vec::new());
        return SevenCardStud {
            players,
            deck,
            dealer_position,
            current_player_index,
            action_history,
            raise_limit,
            bring_in,
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

    fn play_bring_in(&mut self) {
        // the player with the lowest ranking up-card pays the bring in,
        // and betting proceeds after that player in normal clockwise order.
        let mut bring_in_player_index = 0;
        let mut bring_in_player_card: Option<&Card> = None;
        // find player with lowest ranking up-card
        for (player_index, player) in self.players.iter().enumerate() {
            let player_up_cards: Vec<&Card> = player.peek_at_cards().iter()
                .filter(|card| card.is_face_up())
                .map(|card| *card)
                .collect();
            assert_eq!(player_up_cards.len(), 1);
            let player_up_card = player_up_cards[0];
            match bring_in_player_card {
                Some(card) => {
                    assert!(player_up_card != card);
                    if player_up_card < card {
                        bring_in_player_card = Some(player_up_card);
                        bring_in_player_index = player_index;
                    }
                },
                None => {
                    bring_in_player_card = Some(player_up_card);
                    bring_in_player_index = player_index;
                }
            }
        }
        let bring_in_player_index = bring_in_player_index;
        let bring_in_player = self.players.get_mut(bring_in_player_index).unwrap();
        let player_action = PlayerAction::new(&bring_in_player, Action::Ante(self.bring_in as usize));
        self.action_history.push(player_action);
        bring_in_player.bet(self.bring_in as usize).unwrap();
        self.current_player_index = bring_in_player_index;
        self.increment_player_index();
    }

    /// finds the (non-folded) player with the up cards that make the best poker hand,
    /// and returns the index of that player
    fn find_player_with_best_up_card_hand(&self) -> usize {
        let mut best_up_card_hand_player_index = 0;
        let mut best_up_card_hand: Option<Hand> = None;
        // find player with lowest ranking up-card
        for (player_index, player) in self.players.iter().enumerate() {
            if self.action_history.player_has_folded(&player) {
                continue;
            }
            let player_up_cards: Vec<&Card> = player.peek_at_cards().iter()
                .filter(|card| card.is_face_up())
                .map(|card| *card)
                .collect();
            let player_up_card_hand = Hand::new(player_up_cards.iter().map(|&card| card.clone()).collect());
            match best_up_card_hand {
                Some(ref hand) => {
                    assert!(player_up_card_hand != *hand);
                    if player_up_card_hand > *hand {
                        best_up_card_hand = Some(player_up_card_hand);
                        best_up_card_hand_player_index = player_index;
                    }
                },
                None => {
                    best_up_card_hand = Some(player_up_card_hand);
                    best_up_card_hand_player_index = player_index;
                }
            }
        }
        assert!(best_up_card_hand.is_some());
        return best_up_card_hand_player_index;
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
        // each player is dealt two cards face down and one card face up
        for _ in 0..2 {
            self.deal_down_cards()?;
        }
        self.deal_up_cards()?;
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
}

impl<'a, I: Input> Rules<'a> for SevenCardStud<'a, I> {
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

        self.deal_initial_cards().unwrap();
        self.play_bring_in();
        self.play_phase_one();
        self.deal_up_cards().unwrap();
        self.play_phase_two();
        self.deal_up_cards().unwrap();
        self.play_phase_three();
        self.deal_up_cards().unwrap();
        self.play_phase_four();
        self.deal_down_cards().unwrap();
        self.play_phase_five();
        self.showdown();

        self.return_player_cards();

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::input::test_input::TestInput;
    use crate::card::{Rank, Suit};

    use super::*;

    #[test]
    fn new() {
        let seven_card_stud = SevenCardStud::<TestInput>::new(1000, 1);

        assert_eq!(seven_card_stud.deck.size(), 52);
        assert_eq!(seven_card_stud.dealer_position, 0);
        assert_eq!(seven_card_stud.current_player_index, 0);
        assert_eq!(seven_card_stud.action_history.current_bet_amount(), 0);
        assert_eq!(seven_card_stud.action_history.players().len(), 0);
        assert_eq!(seven_card_stud.players.len(), 0);
    }

    #[test]
    fn try_play_round_one_player() {
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, 1);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7())
        ];

        assert!(seven_card_stud.play_round(players.iter_mut().map(|player| player).collect()).is_err_and(|err| err == "Cannot start a game with less than 2 players"));
    }

    #[test]
    fn increment_dealer_position() {
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, 1);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();
        assert_eq!(seven_card_stud.dealer_position, 0);
        seven_card_stud.increment_dealer_position();
        assert_eq!(seven_card_stud.dealer_position, 1);
        seven_card_stud.increment_dealer_position();
        assert_eq!(seven_card_stud.dealer_position, 0);
        seven_card_stud.players.pop();
        seven_card_stud.increment_dealer_position();
        assert_eq!(seven_card_stud.dealer_position, 0);
    }

    #[test]
    fn increment_player_index() {
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, 1);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();
        assert_eq!(seven_card_stud.current_player_index, 0);
        seven_card_stud.increment_player_index();
        assert_eq!(seven_card_stud.current_player_index, 1);
        seven_card_stud.increment_player_index();
        assert_eq!(seven_card_stud.current_player_index, 0);
        seven_card_stud.players.pop();
        seven_card_stud.increment_player_index();
        assert_eq!(seven_card_stud.current_player_index, 0);
    }

    #[test]
    fn deal_initial_cards() {
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, 1);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();
        seven_card_stud.deal_initial_cards().unwrap();
        let mut cards = Vec::new();
        for mut player in players {
            assert_eq!(player.peek_at_cards().len(), 3);
            assert_eq!(player.peek_at_cards().iter().filter(|card| card.is_face_up()).count(), 1);
            assert_eq!(player.peek_at_cards().iter().filter(|card| !card.is_face_up()).count(), 2);
            let temp_cards = player.return_cards();
            // make sure that cards didn't somehow get duplicated, that cards are in fact unique
            for card in temp_cards.iter() {
                assert!(!cards.contains(card));
            }
            cards.extend(temp_cards);
        }
    }

    #[test]
    fn deal_up_cards() {
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, 1);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();
        seven_card_stud.deal_up_cards().unwrap();
        let mut cards = Vec::new();
        for mut player in players {
            assert_eq!(player.peek_at_cards().len(), 1);
            assert_eq!(player.peek_at_cards().iter().filter(|card| card.is_face_up()).count(), 1);
            assert_eq!(player.peek_at_cards().iter().filter(|card| !card.is_face_up()).count(), 0);
            let temp_cards = player.return_cards();
            // make sure that cards didn't somehow get duplicated, that cards are in fact unique
            for card in temp_cards.iter() {
                assert!(!cards.contains(card));
            }
            cards.extend(temp_cards);
        }
    }

    #[test]
    fn deal_down_cards() {
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, 1);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();
        seven_card_stud.deal_down_cards().unwrap();
        let mut cards = Vec::new();
        for mut player in players {
            assert_eq!(player.peek_at_cards().len(), 1);
            assert_eq!(player.peek_at_cards().iter().filter(|card| card.is_face_up()).count(), 0);
            assert_eq!(player.peek_at_cards().iter().filter(|card| !card.is_face_up()).count(), 1);
            let temp_cards = player.return_cards();
            // make sure that cards didn't somehow get duplicated, that cards are in fact unique
            for card in temp_cards.iter() {
                assert!(!cards.contains(card));
            }
            cards.extend(temp_cards);
        }
    }


    #[test]
    fn deal_initial_cards_up_cards_and_down_cards() {
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, 1);
        let mut players = vec![
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7()),
            Player::new(1000, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();
        seven_card_stud.deal_initial_cards().unwrap();
        seven_card_stud.deal_up_cards().unwrap();
        seven_card_stud.deal_up_cards().unwrap();
        seven_card_stud.deal_up_cards().unwrap();
        seven_card_stud.deal_down_cards().unwrap();
        let mut cards = Vec::new();
        for mut player in players {
            assert_eq!(player.peek_at_cards().len(), 7);
            assert_eq!(player.peek_at_cards().iter().filter(|card| card.is_face_up()).count(), 4);
            assert_eq!(player.peek_at_cards().iter().filter(|card| !card.is_face_up()).count(), 3);
            let temp_cards = player.return_cards();
            // make sure that cards didn't somehow get duplicated, that cards are in fact unique
            for card in temp_cards.iter() {
                assert!(!cards.contains(card));
            }
            cards.extend(temp_cards);
        }
    }

    #[test]
    fn play_bring_in() {
        let bring_in_amount = 1;
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, bring_in_amount);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();
        seven_card_stud.deal_initial_cards().unwrap();
        seven_card_stud.play_bring_in();
        assert_eq!(seven_card_stud.action_history.current_bet_amount(), bring_in_amount);
        assert_eq!(players.iter().filter(|player| player.balance() == initial_balance - bring_in_amount as usize).count(), 1);
        assert_eq!(players.iter().filter(|player| player.balance() == initial_balance).count(), 2);
    }

    #[test]
    fn play_phase_one_check_only() {
        let bring_in_amount = 1;
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, bring_in_amount);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();

        seven_card_stud.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        seven_card_stud.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        seven_card_stud.input.set_action_option_selections(vec![
            ActionOption::Call,
            ActionOption::Call,
            ActionOption::Check,
        ]);
        seven_card_stud.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        seven_card_stud.input.set_raise_amounts(vec![
            // no raises to perform as all actions are checks or calls
        ]);

        // manually deal initial (up) cards so we know which player pays bring in
        seven_card_stud.players[0].obtain_card(Card::new(Rank::Two, Suit::Spades, true)); // this player pays bring in
        seven_card_stud.players[1].obtain_card(Card::new(Rank::Three, Suit::Spades, true)); // phase one starts on this player
        seven_card_stud.players[2].obtain_card(Card::new(Rank::Four, Suit::Spades, true));
        seven_card_stud.play_bring_in();
        seven_card_stud.play_phase_one();

        assert_eq!(seven_card_stud.action_history.current_bet_amount(), bring_in_amount);
        assert_eq!(seven_card_stud.current_player_index, 1);
        for player in players.into_iter() {
            assert_eq!(player.balance(), initial_balance - bring_in_amount as usize);
        }
    }

    #[test]
    fn play_phase_one_with_raises() {
        let bring_in_amount = 1;
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, bring_in_amount);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();

        seven_card_stud.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        seven_card_stud.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        seven_card_stud.input.set_action_option_selections(vec![
            ActionOption::Call,
            ActionOption::Call,
            ActionOption::Raise,
            ActionOption::Call,
            ActionOption::Raise,
            ActionOption::Call,
            ActionOption::Call
        ]);
        seven_card_stud.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        seven_card_stud.input.set_raise_amounts(vec![
            100 - bring_in_amount,
            100
        ]);

        // manually deal initial (up) cards so we know which player pays bring in
        seven_card_stud.players[0].obtain_card(Card::new(Rank::Two, Suit::Spades, true)); // this player pays bring in
        seven_card_stud.players[1].obtain_card(Card::new(Rank::Three, Suit::Spades, true)); // phase one starts on this player
        seven_card_stud.players[2].obtain_card(Card::new(Rank::Four, Suit::Spades, true));
        seven_card_stud.play_bring_in();
        seven_card_stud.play_phase_one();

        assert_eq!(seven_card_stud.action_history.current_bet_amount(), 200);
        assert_eq!(seven_card_stud.current_player_index, 2);
        for player in players.into_iter() {
            assert_eq!(player.balance(), initial_balance - 200);
        }
    }

    #[test]
    fn play_phase_one_with_folds() {
        let bring_in_amount = 1;
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, bring_in_amount);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();

        seven_card_stud.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        seven_card_stud.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        seven_card_stud.input.set_action_option_selections(vec![
            ActionOption::Fold, // player 1 folds
            ActionOption::Call,
            ActionOption::Raise,
            ActionOption::Raise,
            ActionOption::Fold // player 0 folds, only player 2 remains
        ]);
        seven_card_stud.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        seven_card_stud.input.set_raise_amounts(vec![
            100 - bring_in_amount,
            100
        ]);

        // manually deal initial (up) cards so we know which player pays bring in
        seven_card_stud.players[0].obtain_card(Card::new(Rank::Two, Suit::Spades, true)); // this player pays bring in
        seven_card_stud.players[1].obtain_card(Card::new(Rank::Three, Suit::Spades, true)); // phase one starts on this player
        seven_card_stud.players[2].obtain_card(Card::new(Rank::Four, Suit::Spades, true));
        seven_card_stud.play_bring_in();
        seven_card_stud.play_phase_one();

        assert_eq!(seven_card_stud.action_history.current_bet_amount(), 200);
        assert_eq!(players.get(0).unwrap().balance(), initial_balance-100); // bring in, raise to 100, then fold
        assert_eq!(players.get(1).unwrap().balance(), initial_balance); // immediately fold
        assert_eq!(players.get(2).unwrap().balance(), initial_balance-200); // call, raise to 200, then fold
    }

    #[test]
    fn play_all_folds_auto_win() {
        let bring_in_amount = 1;
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, bring_in_amount);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();

        seven_card_stud.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        seven_card_stud.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        seven_card_stud.input.set_action_option_selections(vec![
            ActionOption::Fold,
            ActionOption::Fold,
            ActionOption::Raise // this should not be allowed to happen as this player (0) should automatically win
        ]);
        seven_card_stud.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        seven_card_stud.input.set_raise_amounts(vec![
            100 - bring_in_amount,
        ]);

        // manually deal initial (up) cards so we know which player pays bring in
        seven_card_stud.players[0].obtain_card(Card::new(Rank::Two, Suit::Spades, true)); // this player pays bring in
        seven_card_stud.players[1].obtain_card(Card::new(Rank::Three, Suit::Spades, true)); // phase one starts on this player
        seven_card_stud.players[2].obtain_card(Card::new(Rank::Four, Suit::Spades, true));
        seven_card_stud.play_bring_in();
        seven_card_stud.play_phase_one();

        assert_eq!(seven_card_stud.action_history.current_bet_amount(), bring_in_amount);
        assert_eq!(players.get(0).unwrap().balance(), initial_balance - bring_in_amount as usize); // pays bring in, should not have the opportunity to raise
        assert_eq!(players.get(1).unwrap().balance(), initial_balance); // immediately fold
        assert_eq!(players.get(2).unwrap().balance(), initial_balance); // immediately fold
    }

    #[test]
    fn play_full_round_all_checks_and_calls() {
        let bring_in_amount = 1;
        let mut seven_card_stud = SevenCardStud::<TestInput>::new(1000, bring_in_amount);
        let initial_balance = 1000;
        let mut players = vec![
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7()),
            Player::new(initial_balance, Uuid::now_v7())
        ];
        seven_card_stud.players = players.iter_mut().map(|player| player).collect();

        seven_card_stud.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        seven_card_stud.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        seven_card_stud.input.set_action_option_selections(vec![
            ActionOption::Call, // phase 1
            ActionOption::Call,
            ActionOption::Check,
            ActionOption::Check, // phase 2
            ActionOption::Check,
            ActionOption::Check,
            ActionOption::Check, // phase 3
            ActionOption::Check,
            ActionOption::Check,
            ActionOption::Check, // phase 4
            ActionOption::Check,
            ActionOption::Check,
            ActionOption::Check, // phase 5
            ActionOption::Check,
            ActionOption::Check
        ]);
        seven_card_stud.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        seven_card_stud.input.set_raise_amounts(vec![
            // no raises as all actions are checks or calls
        ]);

        // manually deal initial (up) cards so we know which player pays bring in
        seven_card_stud.deal_initial_cards().unwrap();
        seven_card_stud.play_bring_in();
        seven_card_stud.play_phase_one();
        seven_card_stud.deal_up_cards().unwrap();
        seven_card_stud.play_phase_two();
        seven_card_stud.deal_up_cards().unwrap();
        seven_card_stud.play_phase_three();
        seven_card_stud.deal_up_cards().unwrap();
        seven_card_stud.play_phase_four();
        seven_card_stud.deal_down_cards().unwrap();
        seven_card_stud.play_phase_five();
        seven_card_stud.showdown();

        assert_eq!(seven_card_stud.action_history.current_bet_amount(), bring_in_amount);
        assert_eq!(players.get(0).unwrap().balance(), initial_balance - bring_in_amount as usize);
        assert_eq!(players.get(1).unwrap().balance(), initial_balance - bring_in_amount as usize);
        assert_eq!(players.get(2).unwrap().balance(), initial_balance - bring_in_amount as usize);
    }
}
