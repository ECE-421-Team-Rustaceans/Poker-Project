use uuid::Uuid;

use crate::card::Card;
use crate::database::db_handler::DbHandler;
use crate::deck::Deck;
use crate::hand_rank::Hand;
use crate::input::Input;
use crate::player::Player;
use crate::pot::Pot;
use super::Rules;
use crate::action_option::ActionOption;
use crate::action::Action;

use std::cmp::min;

/// Five Card Draw Rules
/// 
/// This struct keeps track of all information relevant to a game of five card draw,
/// and has methods for each of the phases of the game as per the rules on wikipedia,
/// as well as some helper methods for commonly used operations.
/// The only methods that are used by external code, however, are the constructor (new)
/// and the play_round method which uses the rest of the methods to run a whole
/// round of five card draw. Those two methods are an implementation of the Rules trait.
pub struct FiveCardDraw<I: Input> {
    players: Vec<Player>,
    deck: Deck,
    dealer_position: usize,
    current_player_index: usize,
    raise_limit: u32,
    big_blind_amount: u32,
    input: I,
    pot: Pot,
    game_id: Uuid
}

impl<I: Input> FiveCardDraw<I> {
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
        self.pot.add_turn(&first_blind_player.account_id(), Action::Ante(<u32 as TryInto<usize>>::try_into(self.big_blind_amount).unwrap()/2), 0, first_blind_player.peek_at_cards().iter().map(|&card| card.clone()).collect());
        first_blind_player.bet(<u32 as TryInto<usize>>::try_into(self.big_blind_amount).unwrap()/2).unwrap();
        self.increment_player_index();

        let second_blind_player = match self.players.get_mut(self.dealer_position+1) {
            Some(player) => player,
            None => {
                self.players.get_mut(0).expect("Expected a non-zero number of players")
            }
        };
        self.pot.add_turn(&second_blind_player.account_id(), Action::Ante(self.big_blind_amount as usize), 0, second_blind_player.peek_at_cards().iter().map(|&card| card.clone()).collect());
        second_blind_player.bet(self.big_blind_amount as usize).unwrap();
        self.increment_player_index();
    }

    fn play_bet_phase(&mut self, phase_number: usize) {
        // betting starts with the first blind player (player at self.dealer_position)
        self.current_player_index = self.dealer_position;
        let mut last_raise_player_index = self.current_player_index;
        let mut raise_has_occurred = false;
        loop {
            if self.pot.number_of_players_folded()+1 == (self.players.len() as u32) {
                // all players have folded but one, remaining player automatically wins
                break;
            }
            let player_matched_call = self.pot.get_call_amount() == self.pot.get_player_stake(&self.players.get(self.current_player_index).unwrap().account_id());
            if self.number_of_players_all_in()+1 == self.players.len() && player_matched_call {
                // all players are all in but one, remaining player doesn't need to bet
                break;
            }

            let player: &Player = &self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None");

            if !(self.pot.player_has_folded(&player.account_id()) || player.balance() == 0) {
                self.input.display_pot(self.pot.get_total_stake(), self.players.iter().map(|player| player as &Player).collect());
                self.input.display_current_player(player);
                self.input.display_player_cards_to_player(player);

                let player: &mut Player = &mut self.players.get_mut(self.current_player_index).expect("Expected a player at this index, but there was None");

                if !raise_has_occurred && self.pot.get_call_amount() == self.pot.get_player_stake(&player.account_id()) {
                    // the big blind can check because they already paid a full bet, and on the second round, everyone can check if nobody raises
                    let action_options = vec![ActionOption::Check, ActionOption::Raise, ActionOption::Fold];
                    let chosen_action_option: ActionOption = self.input.input_action_options(action_options, &player);

                    let player_raise_limit = min(self.raise_limit, player.balance() as u32);

                    let action = match chosen_action_option {
                        ActionOption::Check => Action::Check,
                        ActionOption::Raise => Action::Raise(self.pot.get_call_amount() as usize + self.input.request_raise_amount(player_raise_limit, &player) as usize),
                        ActionOption::Fold => Action::Fold,
                        _ => panic!("Player managed to select an impossible Action!")
                    };

                    match action {
                        Action::Check => {},
                        Action::Raise(raise_amount) => {
                            last_raise_player_index = self.current_player_index;
                            raise_has_occurred = true;
                            let bet_amount = raise_amount - self.pot.get_player_stake(&player.account_id()) as usize;
                            player.bet(bet_amount as usize).unwrap();
                        },
                        Action::Fold => {},
                        _ => panic!("Player managed to perform an impossible Action!")
                    }

                    self.pot.add_turn(&player.account_id(), action, phase_number, player.peek_at_cards().iter().map(|&card| card.clone()).collect());
                }
                else {
                    let current_bet_amount = self.pot.get_call_amount() as u32;
                    if player.balance() as u32 > current_bet_amount {
                        let action_options = vec![ActionOption::Call, ActionOption::Raise, ActionOption::Fold];
                        let chosen_action_option: ActionOption = self.input.input_action_options(action_options, &player);

                        let player_raise_limit = min(self.raise_limit, player.balance() as u32 - current_bet_amount);
                        let action = match chosen_action_option {
                            ActionOption::Call => Action::Call,
                            ActionOption::Raise => Action::Raise(<i64 as TryInto<usize>>::try_into(self.pot.get_call_amount()).unwrap() + self.input.request_raise_amount(player_raise_limit, &player) as usize),
                            ActionOption::Fold => Action::Fold,
                            _ => panic!("Player managed to select an impossible Action!")
                        };
    
                        match action {
                            Action::Call => {
                                let bet_amount = self.pot.get_call_amount() - self.pot.get_player_stake(&player.account_id());
                                player.bet(bet_amount as usize).unwrap();
                            },
                            Action::Raise(raise_amount) => {
                                last_raise_player_index = self.current_player_index;
                                raise_has_occurred = true;
                                let bet_amount = raise_amount - <i64 as TryInto<usize>>::try_into(self.pot.get_player_stake(&player.account_id())).unwrap();
                                player.bet(bet_amount).unwrap();
                            },
                            Action::Fold => {},
                            _ => panic!("Player managed to perform an impossible Action!")
                        }
                        self.pot.add_turn(&player.account_id(), action, phase_number, player.peek_at_cards().iter().map(|&card| card.clone()).collect());
                    } else {
                        let action_options = vec![ActionOption::AllIn, ActionOption::Fold];
                        let chosen_action_option: ActionOption = self.input.input_action_options(action_options, &player);

                        // player does not have enough money for a full call, nevermind a raise
                        let action = match chosen_action_option {
                            ActionOption::AllIn => Action::AllIn(<i64 as TryInto<usize>>::try_into(self.pot.get_player_stake(&player.account_id())).unwrap() + player.balance()),
                            ActionOption::Fold => Action::Fold,
                            _ => panic!("Player managed to select an impossible Action!")
                        };
    
                        match action {
                            Action::AllIn(total_stake) => {
                                let bet_amount = total_stake - <i64 as TryInto<usize>>::try_into(self.pot.get_player_stake(&player.account_id())).unwrap();
                                assert_eq!(bet_amount, player.balance());
                                player.bet(bet_amount).unwrap();
                            },
                            Action::Fold => {},
                            _ => panic!("Player managed to perform an impossible Action!")
                        }
                        self.pot.add_turn(&player.account_id(), action, phase_number, player.peek_at_cards().iter().map(|&card| card.clone()).collect());
                    };
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
        self.play_bet_phase(1);
    }

    fn play_draw_phase(&mut self) {
        // house rules: players may discard as many cards as they wish to draw new replacements
        let start_player_index = self.current_player_index;
        loop {
            if self.pot.number_of_players_folded()+1 == (self.players.len() as u32) {
                // all players have folded but one, remaining player automatically wins
                break;
            }

            let player: &Player = &self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None");

            if !self.pot.player_has_folded(&player.account_id()) {
                self.input.display_pot(self.pot.get_total_stake(), self.players.iter().map(|player| player as &Player).collect());
                self.input.display_player_balances(self.players.iter().collect());
                self.input.display_current_player(player);
                self.input.display_player_cards_to_player(player);

                let player: &mut Player = self.players.get_mut(self.current_player_index).expect("Expected a player at this index, but there was None");

                let action_options = vec![ActionOption::Replace, ActionOption::Check];
                let chosen_action_option: ActionOption = self.input.input_action_options(action_options, &player);

                let action = match chosen_action_option {
                    ActionOption::Replace => Action::Replace(
                        self.input.request_replace_cards(
                            &player
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

                self.pot.add_turn(&player.account_id(), action, 2, player.peek_at_cards().iter().map(|&card| card.clone()).collect());
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
        self.play_bet_phase(3);
    }

    /// take each non-folded player's cards, and make them all up cards (visible to everyone)
    fn flip_non_folded_players_cards_up(&mut self) {
        for player in self.players.iter_mut().filter(|player| !self.pot.player_has_folded(&player.account_id())) {
            let mut cards = player.return_cards();
            cards.iter_mut().for_each(|card| card.set_face_up(true));
            for card in cards {
                player.obtain_card(card);
            }
        }
    }

    fn showdown(&mut self) {
        // show to each player everyone's cards (except folded)
        let start_player_index = self.current_player_index;
        let mut current_player_index = self.current_player_index;
        self.input.display_pot(self.pot.get_total_stake(), self.players.iter().map(|player| player as &Player).collect());
        self.flip_non_folded_players_cards_up();
        loop {
            let player: &Player = self.players.get(current_player_index).expect("Expected a player at this index, but there was None");

            if !self.pot.player_has_folded(&player.account_id()) {
                let other_players: Vec<&Player> = self.players.iter()
                    .filter(|&other_player| other_player != player)
                    .map(|player| player as &Player)
                    .collect();
                self.input.display_other_player_up_cards_to_player(other_players, player);
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

        let mut player_cards: Vec<(Uuid, Vec<&Card>)> = self.players.iter()
            .filter(|player| !self.pot.player_has_folded(&player.account_id()))
            .map(|player| (player.account_id(), player.peek_at_cards()))
            .collect();
        player_cards.sort_by(|left, right| Hand::new(right.1.iter().map(|&card| card.clone()).collect())
            .cmp(&Hand::new(left.1.iter().map(|&card| card.clone())
            .collect()))); // sort by best hand of cards first // FIXME: unsure if problematic if there's one or more ties
        let mut winning_order: Vec<Vec<Uuid>> = vec![vec![player_cards[0].0]];
        for player_cards_index in 1..player_cards.len() {
            let this_players_hand = Hand::new(player_cards[player_cards_index].1.iter().map(|&card| card.clone()).collect());
            let last_players_hand = Hand::new(player_cards[player_cards_index-1].1.iter().map(|&card| card.clone()).collect());
            if this_players_hand == last_players_hand {
                winning_order.last_mut().unwrap().push(player_cards[player_cards_index].0);
            }
            else {
                assert!(this_players_hand < last_players_hand);
                winning_order.push(vec![player_cards[player_cards_index].0]);
            }
        }
        winning_order.push(self.players.iter()
            .filter(|player| self.pot.player_has_folded(&player.account_id()))
            .map(|player| player.account_id()).collect());
        let player_winnings_map = self.pot.divide_winnings(winning_order);
        let mut winner_uuids = Vec::new();
        for (player_id, &winnings) in player_winnings_map.iter() {
            assert!(winnings >= 0);
            if winnings > 0 {
                let mut player_matches: Vec<&mut Player> = self.players.iter_mut().filter(|player| player.account_id() == *player_id).collect();
                assert_eq!(player_matches.len(), 1);
                let player_match = &mut player_matches[0];
                assert!(!self.pot.player_has_folded(&player_match.account_id()), "Player: {}, winning amount: {}", player_match.account_id(), winnings);
                player_match.win(winnings as usize);
                winner_uuids.push(player_id);
            }
        }
        let winners: Vec<&Player> = self.players.iter().filter(|player| winner_uuids.iter().any(|&uuid| player.account_id() == *uuid)).map(|player| player as &Player).collect();
        self.input.announce_winner(winners, self.players.iter().map(|player| player as &Player).collect());
        self.input.display_player_balances(self.players.iter().collect());
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

impl<I: Input> Rules for FiveCardDraw<I> {
    async fn play_round(&mut self, players: Vec<Player>) -> Result<Vec<Player>, (&'static str, Vec<Player>)> {
        if players.len() < 2 {
            return Err(("Cannot start a game with less than 2 players", players));
        }
        if players.len() > 10 {
            return Err(("Cannot start a game with more than 10 players, as the deck may run out of cards", players));
        }
        self.pot.clear(&players.iter().collect());
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
        self.pot.save(self.game_id).await;

        self.return_player_cards();

        return Ok(self.players.drain(..).collect());
    }

    fn new(raise_limit: u32, minimum_bet: u32, db_handler: DbHandler, game_id: Uuid) -> FiveCardDraw<I> {
        let deck = Deck::new();
        let dealer_position = 0_usize;
        let current_player_index = 0_usize;
        let players = Vec::new();
        let pot = Pot::new(&Vec::new(), db_handler);
        return FiveCardDraw {
            players,
            deck,
            dealer_position,
            current_player_index,
            raise_limit,
            big_blind_amount: minimum_bet,
            input: I::new(),
            pot,
            game_id
        };
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::input::test_input::TestInput;

    use super::*;

    #[test]
    fn new() {
        let five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());

        assert_eq!(five_card_draw.deck.size(), 52);
        assert_eq!(five_card_draw.dealer_position, 0);
        assert_eq!(five_card_draw.current_player_index, 0);
        assert_eq!(five_card_draw.pot.get_call_amount(), 0);
        assert_eq!(five_card_draw.pot.get_player_ids().len(), 0);
        assert_eq!(five_card_draw.players.len(), 0);
    }

    #[tokio::test]
    async fn try_play_round_one_player() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000)
        ];

        assert!(five_card_draw.play_round(players).await.is_err_and(|err| err.0 == "Cannot start a game with less than 2 players"));
    }

    #[test]
    fn increment_dealer_position() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000)
        ];
        five_card_draw.players = players;
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
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000)
        ];
        five_card_draw.players = players;
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
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        five_card_draw.players = players;
        five_card_draw.play_blinds();
        assert_eq!(five_card_draw.pot.get_call_amount(), 2);
        assert_eq!(five_card_draw.current_player_index, 2);
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), initial_balance-1);
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), initial_balance-2);
    }

    #[test]
    fn deal_initial_cards() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000)
        ];
        five_card_draw.players = players;
        five_card_draw.deal_initial_cards().unwrap();
        let mut cards = Vec::new();
        for mut player in five_card_draw.players {
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
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        five_card_draw.players = players;

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

        assert_eq!(five_card_draw.pot.get_call_amount(), 2);
        assert_eq!(five_card_draw.dealer_position, 0);
        assert_eq!(five_card_draw.current_player_index, 0);
        for player in five_card_draw.players.into_iter() {
            assert_eq!(player.balance(), initial_balance-2);
        }
    }

    #[test]
    fn play_phase_one_with_raises() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        five_card_draw.players = players;

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

        assert_eq!(five_card_draw.pot.get_call_amount(), 27);
        assert_eq!(five_card_draw.dealer_position, 0);
        assert_eq!(five_card_draw.current_player_index, 1);
        for player in five_card_draw.players.into_iter() {
            assert_eq!(player.balance(), initial_balance-27);
        }
    }

    #[test]
    fn play_phase_one_with_folds() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        five_card_draw.players = players;

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

        assert_eq!(five_card_draw.pot.get_call_amount(), 27);
        assert_eq!(five_card_draw.dealer_position, 0);
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), initial_balance-1); // small blind then fold
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), initial_balance-27); // the only remaining player, they have the max bet
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), initial_balance-12); // raise to 12 then fold
    }

    #[test]
    fn play_all_folds_auto_win() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        five_card_draw.players = players;

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
        assert_eq!(five_card_draw.pot.get_call_amount(), 2);
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), initial_balance-1); // small blind and fold
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), initial_balance-2); // big blind and fold
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), initial_balance); // should not have the opportunity to raise due to auto-winning
        five_card_draw.showdown();
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), initial_balance-1); // small blind and fold
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), initial_balance-2); // big blind and fold
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), initial_balance+3); // automatically wins due to other players folding, gets 3$
    }

    #[test]
    fn play_full_game_auto_win() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        five_card_draw.players = players;

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Call, // phase 1
            ActionOption::Check,
            ActionOption::Raise,
            ActionOption::Call,
            ActionOption::Call,
            ActionOption::Check, // draw phase
            ActionOption::Check,
            ActionOption::Check,
            ActionOption::Raise, // phase 2, start back at player 0
            ActionOption::Raise,
            ActionOption::Fold,
            ActionOption::Raise,
            ActionOption::Fold
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are folds
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            98,
            100,
            100,
            100
        ]);

        five_card_draw.play_blinds();
        five_card_draw.deal_initial_cards().unwrap();
        five_card_draw.play_phase_one();
        five_card_draw.play_draw_phase();
        five_card_draw.play_phase_two();
        assert_eq!(five_card_draw.pot.get_call_amount(), 400);
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), initial_balance-400); // small blind, call to 2, call to 100, raise to 200, raise to 400, auto-wins
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), initial_balance-300); // big blind, call to 100, raise to 300, and fold
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), initial_balance-100); // raise to 100, and fold
        five_card_draw.showdown();
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), initial_balance+400);
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), initial_balance-300);
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), initial_balance-100);
    }

    #[test]
    fn play_draw_phase_draw_various_amounts_of_cards() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        five_card_draw.players = players;

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

        assert_eq!(five_card_draw.pot.get_call_amount(), 2);
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
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        five_card_draw.players = players;

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
        assert_eq!(five_card_draw.pot.get_call_amount(), 2);
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), initial_balance-2); // call to 2 and check the rest
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), initial_balance-2); // big blind 2 and check the rest
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), initial_balance-2); // call to 2 and check the rest
        five_card_draw.showdown();
    }

    #[test]
    fn play_phase_one_with_all_ins() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 100;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        five_card_draw.players = players;

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Call,
            ActionOption::Check,
            ActionOption::Raise,
            ActionOption::AllIn,
            ActionOption::AllIn // this player MUST go all in (call would do the same thing as all in, raise limit is 0) to match the call
            // players should no longer be able to play bet phases, as they have nothing to bet (but they can still replace cards)
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks, calls, raises or folds
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            98 // raise to the amount that every player has
        ]);

        five_card_draw.play_blinds();
        five_card_draw.play_phase_one();

        assert_eq!(five_card_draw.pot.get_call_amount(), 100);
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), 0);
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), 0);
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), 0);
    }

    #[test]
    fn play_phase_one_with_all_ins_not_enough_further_raise() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 100),
            Player::new(Uuid::now_v7(), "player".to_string(), 10)
        ];
        five_card_draw.players = players;

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Raise,
            ActionOption::AllIn,
            ActionOption::AllIn // players 1 and 2 should no longer be able to play bet phases, as they have nothing to bet (but they can still replace cards)
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks, calls, raises or folds
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            498 // raise to more than players 1 and 2 have
        ]);

        five_card_draw.play_blinds();
        five_card_draw.play_phase_one();

        assert_eq!(five_card_draw.pot.get_call_amount(), 500);
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), 500);
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), 0);
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), 0);
    }

    #[test]
    fn play_full_round_with_all_ins_not_enough() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 100),
            Player::new(Uuid::now_v7(), "player".to_string(), 10)
        ];
        five_card_draw.players = players;

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Raise,
            ActionOption::AllIn,
            ActionOption::AllIn, // players 1 and 2 should no longer be able to play bet phases, as they have nothing to bet (but they can still replace cards)
            ActionOption::Check, // draw phase
            ActionOption::Replace,
            ActionOption::Check // last betting phase is skipped because all players are all in but one
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            vec![0, 2, 4] // player 1 replaces cards after all in
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            498 // raise to more than players 1 and 2 have
        ]);

        five_card_draw.play_blinds();
        five_card_draw.deal_initial_cards().unwrap();
        five_card_draw.play_phase_one();
        five_card_draw.play_draw_phase();
        five_card_draw.play_phase_two();
        assert_eq!(five_card_draw.pot.get_call_amount(), 500);
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), 500);
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), 0);
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), 0);
        five_card_draw.showdown();
        let total_balance: usize = five_card_draw.players.iter().map(|player| player.balance()).sum();
        assert_eq!(total_balance, 1110);
    }

    #[test]
    fn play_full_round_with_all_ins_not_enough_further_raise() {
        let mut five_card_draw = FiveCardDraw::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 100),
            Player::new(Uuid::now_v7(), "player".to_string(), 10)
        ];
        five_card_draw.players = players;

        five_card_draw.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        five_card_draw.input.set_game_variation(crate::game_type::GameType::FiveCardDraw);
        five_card_draw.input.set_action_option_selections(vec![
            ActionOption::Raise,
            ActionOption::Call,
            ActionOption::AllIn, // player 2 should no longer be able to play bet phases, as they have nothing to bet (but they can still replace cards)
            ActionOption::Check, // draw phase
            ActionOption::Replace,
            ActionOption::Check,
            ActionOption::Raise, // phase 2, player 0 can raise because not everyone else is all in yet
            ActionOption::AllIn // however, after this, both player 1 and 2 can no longer bet, so the round is over
        ]);
        five_card_draw.input.set_card_replace_selections(vec![
            vec![0, 2, 4] // player 1 replaces cards after all in
        ]);
        five_card_draw.input.set_raise_amounts(vec![
            48, // raise to more than player 2 has
            150 // raise to more than player 1 has
        ]);

        five_card_draw.play_blinds();
        five_card_draw.deal_initial_cards().unwrap();
        five_card_draw.play_phase_one();
        five_card_draw.play_draw_phase();
        five_card_draw.play_phase_two();
        assert_eq!(five_card_draw.pot.get_call_amount(), 200);
        assert_eq!(five_card_draw.players.get(0).unwrap().balance(), 800);
        assert_eq!(five_card_draw.players.get(1).unwrap().balance(), 0);
        assert_eq!(five_card_draw.players.get(2).unwrap().balance(), 0);
        five_card_draw.showdown();
        let total_balance: usize = five_card_draw.players.iter().map(|player| player.balance()).sum();
        assert_eq!(total_balance, 1110);
    }
}
