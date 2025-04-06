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

/// Texas Holdem Rules
/// 
/// This struct keeps track of all information relevant to a game of texas hold'em,
/// and has methods for each of the phases of the game as per the rules on wikipedia,
/// as well as some helper methods for commonly used operations.
/// The only methods that are used by external code, however, are the constructor (new)
/// and the play_round method which uses the rest of the methods to run a whole
/// round of texas hold'em. Those two methods are an implementation of the Rules trait.
pub struct TexasHoldem<I: Input> {
    players: Vec<Player>,
    deck: Deck,
    dealer_position: usize,
    current_player_index: usize,
    raise_limit: u32,
    big_blind_amount: u32,
    input: I,
    pot: Pot,
    game_id: Uuid,
    community_cards: Vec<Card>
}

impl<I: Input> TexasHoldem<I> {
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
        // for every betting phase except the first, betting starts with the first blind player (player at self.dealer_position)
        if phase_number != 1 {
            self.current_player_index = self.dealer_position;
        }
        // otherwise (so, for the first betting phase) betting starts with the player after the big blind
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
                self.input.display_player_balances(self.players.iter().collect());
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
                    let action_options = vec![ActionOption::Call, ActionOption::Raise, ActionOption::Fold];
                    let chosen_action_option: ActionOption = self.input.input_action_options(action_options, &player);

                    let current_bet_amount = self.pot.get_call_amount() as u32;
                    if player.balance() as u32 > current_bet_amount {
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

    fn play_phase_two(&mut self) {
        self.play_bet_phase(2);
    }

    fn play_phase_three(&mut self) {
        self.play_bet_phase(3);
    }

    fn play_phase_four(&mut self) {
        self.play_bet_phase(4);
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
        // each player is dealt two cards face down
        for _ in 0..2 {
            self.deal_down_cards()?;
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

    /// deals a community card, iff there are at least two players who can still take bet actions (haven't folded or gone all in)
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

    /// each non-folded player is dealt one card face down
    fn deal_down_cards(&mut self) -> Result<(), String> {
        let remaining_players = self.players.iter_mut()
            .filter(|player| !self.pot.player_has_folded(&player.account_id()));
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

impl<I: Input> Rules for TexasHoldem<I> {
    async fn play_round(&mut self, players: Vec<Player>) -> Result<Vec<Player>, (&'static str, Vec<Player>)> {
        if players.len() < 2 {
            return Err(("Cannot start a game with less than 2 players", players));
        }
        if players.len() > 23 {
            return Err(("Cannot start a game with more than 23 players, as the deck may run out of cards", players));
        }
        self.pot.clear(&players.iter().collect());
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
        self.pot.save(self.game_id).await;

        self.return_player_cards();
        self.return_community_cards();

        return Ok(self.players.drain(..).collect());
    }

    fn new(raise_limit: u32, minimum_bet: u32, db_handler: DbHandler, game_id: Uuid) -> TexasHoldem<I> {
        let deck = Deck::new();
        let dealer_position = 0_usize;
        let current_player_index = 0_usize;
        let players = Vec::new();
        let pot = Pot::new(&Vec::new(), db_handler);
        let community_cards = Vec::new();
        return TexasHoldem {
            players,
            deck,
            dealer_position,
            current_player_index,
            raise_limit,
            big_blind_amount: minimum_bet,
            input: I::new(),
            pot,
            game_id,
            community_cards
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
        let texas_holdem = TexasHoldem::<TestInput>::new(1000, 1, DbHandler::new_dummy(), Uuid::now_v7());

        assert_eq!(texas_holdem.deck.size(), 52);
        assert_eq!(texas_holdem.dealer_position, 0);
        assert_eq!(texas_holdem.current_player_index, 0);
        assert_eq!(texas_holdem.pot.get_call_amount(), 0);
        assert_eq!(texas_holdem.pot.get_player_ids().len(), 0);
        assert_eq!(texas_holdem.players.len(), 0);
    }

    #[tokio::test]
    async fn try_play_round_one_player() {
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, 1, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000)
        ];

        assert!(texas_holdem.play_round(players).await.is_err_and(|err| err.0 == "Cannot start a game with less than 2 players"));
    }

    #[test]
    fn increment_dealer_position() {
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, 1, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000)
        ];
        texas_holdem.players = players;
        assert_eq!(texas_holdem.dealer_position, 0);
        texas_holdem.increment_dealer_position();
        assert_eq!(texas_holdem.dealer_position, 1);
        texas_holdem.increment_dealer_position();
        assert_eq!(texas_holdem.dealer_position, 0);
        texas_holdem.players.pop();
        texas_holdem.increment_dealer_position();
        assert_eq!(texas_holdem.dealer_position, 0);
    }

    #[test]
    fn increment_player_index() {
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, 1, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000)
        ];
        texas_holdem.players = players;
        assert_eq!(texas_holdem.current_player_index, 0);
        texas_holdem.increment_player_index();
        assert_eq!(texas_holdem.current_player_index, 1);
        texas_holdem.increment_player_index();
        assert_eq!(texas_holdem.current_player_index, 0);
        texas_holdem.players.pop();
        texas_holdem.increment_player_index();
        assert_eq!(texas_holdem.current_player_index, 0);
    }

    #[test]
    fn deal_initial_cards() {
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, 1, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000)
        ];
        texas_holdem.players = players;
        texas_holdem.deal_initial_cards().unwrap();
        let mut cards = Vec::new();
        for mut player in texas_holdem.players {
            assert_eq!(player.peek_at_cards().len(), 2);
            assert_eq!(player.peek_at_cards().iter().filter(|card| card.is_face_up()).count(), 0);
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
    fn deal_down_cards() {
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, 1, DbHandler::new_dummy(), Uuid::now_v7());
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000),
            Player::new(Uuid::now_v7(), "player".to_string(), 1000)
        ];
        texas_holdem.players = players;
        texas_holdem.deal_down_cards().unwrap();
        let mut cards = Vec::new();
        for mut player in texas_holdem.players {
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
    fn play_blinds() {
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        texas_holdem.players = players;
        texas_holdem.play_blinds();
        assert_eq!(texas_holdem.pot.get_call_amount(), 2);
        assert_eq!(texas_holdem.current_player_index, 2);
        assert_eq!(texas_holdem.players.get(0).unwrap().balance(), initial_balance-1);
        assert_eq!(texas_holdem.players.get(1).unwrap().balance(), initial_balance-2);
    }

    #[test]
    fn play_phase_one_check_only() {
        let big_blind_amount = 2;
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, big_blind_amount, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        texas_holdem.players = players;

        texas_holdem.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        texas_holdem.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        texas_holdem.input.set_action_option_selections(vec![
            ActionOption::Call,
            ActionOption::Call,
            ActionOption::Check,
        ]);
        texas_holdem.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        texas_holdem.input.set_raise_amounts(vec![
            // no raises to perform as all actions are checks or calls
        ]);

        texas_holdem.play_blinds();
        texas_holdem.play_phase_one();

        assert_eq!(texas_holdem.pot.get_call_amount() as u32, big_blind_amount);
        assert_eq!(texas_holdem.current_player_index, 2);
        for player in texas_holdem.players.into_iter() {
            assert_eq!(player.balance(), initial_balance - big_blind_amount as usize);
        }
    }

    #[test]
    fn play_phase_one_with_raises() {
        let big_blind_amount = 2;
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, big_blind_amount, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        texas_holdem.players = players;

        texas_holdem.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        texas_holdem.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        texas_holdem.input.set_action_option_selections(vec![
            ActionOption::Call,
            ActionOption::Call,
            ActionOption::Raise,
            ActionOption::Call,
            ActionOption::Raise,
            ActionOption::Call,
            ActionOption::Call
        ]);
        texas_holdem.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        texas_holdem.input.set_raise_amounts(vec![
            100 - big_blind_amount,
            100
        ]);

        texas_holdem.play_blinds();
        texas_holdem.play_phase_one();

        assert_eq!(texas_holdem.pot.get_call_amount() as u32, 200);
        assert_eq!(texas_holdem.current_player_index, 0);
        for player in texas_holdem.players.into_iter() {
            assert_eq!(player.balance(), initial_balance - 200);
        }
    }

    #[test]
    fn play_phase_one_with_folds() {
        let big_blind_amount = 2;
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, big_blind_amount, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        texas_holdem.players = players;

        texas_holdem.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        texas_holdem.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        texas_holdem.input.set_action_option_selections(vec![
            ActionOption::Fold, // player 2 folds
            ActionOption::Call,
            ActionOption::Raise,
            ActionOption::Raise,
            ActionOption::Fold // player 1 folds, only player 0 remains
        ]);
        texas_holdem.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        texas_holdem.input.set_raise_amounts(vec![
            100 - big_blind_amount,
            100
        ]);

        texas_holdem.play_blinds();
        texas_holdem.play_phase_one();

        assert_eq!(texas_holdem.pot.get_call_amount() as u32, 200);
        assert_eq!(texas_holdem.players.get(0).unwrap().balance(), initial_balance-200); // call, raise to 200, then fold
        assert_eq!(texas_holdem.players.get(1).unwrap().balance(), initial_balance-100); // bring in, raise to 100, then fold
        assert_eq!(texas_holdem.players.get(2).unwrap().balance(), initial_balance); // immediately fold
    }

    #[test]
    fn play_all_folds_auto_win() {
        let big_blind_amount = 2;
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, big_blind_amount, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        texas_holdem.players = players;

        texas_holdem.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        texas_holdem.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        texas_holdem.input.set_action_option_selections(vec![
            ActionOption::Fold,
            ActionOption::Fold,
            ActionOption::Raise // this should not be allowed to happen as this player (0) should automatically win
        ]);
        texas_holdem.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        texas_holdem.input.set_raise_amounts(vec![
            100 - big_blind_amount,
        ]);

        texas_holdem.play_blinds();
        texas_holdem.play_phase_one();

        assert_eq!(texas_holdem.pot.get_call_amount() as u32, big_blind_amount);
        assert_eq!(texas_holdem.players.get(0).unwrap().balance(), initial_balance - big_blind_amount as usize / 2); // pays small blind, then immediately fold
        assert_eq!(texas_holdem.players.get(1).unwrap().balance(), initial_balance - big_blind_amount as usize); // pays big blind, should not have the opportunity to raise
        assert_eq!(texas_holdem.players.get(2).unwrap().balance(), initial_balance); // immediately fold
    }

    #[test]
    fn play_full_round_all_checks_and_calls() {
        let big_blind_amount = 2;
        let mut texas_holdem = TexasHoldem::<TestInput>::new(1000, big_blind_amount, DbHandler::new_dummy(), Uuid::now_v7());
        let initial_balance = 1000;
        let players = vec![
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance),
            Player::new(Uuid::now_v7(), "player".to_string(), initial_balance)
        ];
        texas_holdem.players = players;

        texas_holdem.input.set_player_names(vec!["p1".to_string(), "p2".to_string(), "p3".to_string()]);
        texas_holdem.input.set_game_variation(crate::game_type::GameType::SevenCardStud);
        texas_holdem.input.set_action_option_selections(vec![
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
            ActionOption::Check
        ]);
        texas_holdem.input.set_card_replace_selections(vec![
            // no cards to replace as all actions are checks or calls
        ]);
        texas_holdem.input.set_raise_amounts(vec![
            // no raises as all actions are checks or calls
        ]);

        // manually deal initial (up) cards so we know which player pays bring in
        texas_holdem.deal_initial_cards().unwrap();
        texas_holdem.play_blinds();
        texas_holdem.play_phase_one();
        texas_holdem.deal_flop_cards().unwrap();
        texas_holdem.play_phase_two();
        texas_holdem.deal_community_card().unwrap();
        texas_holdem.play_phase_three();
        texas_holdem.deal_community_card().unwrap();
        texas_holdem.play_phase_four();
        assert_eq!(texas_holdem.pot.get_call_amount() as u32, big_blind_amount);
        assert_eq!(texas_holdem.players.get(0).unwrap().balance(), initial_balance - big_blind_amount as usize);
        assert_eq!(texas_holdem.players.get(1).unwrap().balance(), initial_balance - big_blind_amount as usize);
        assert_eq!(texas_holdem.players.get(2).unwrap().balance(), initial_balance - big_blind_amount as usize);
        texas_holdem.showdown();
    }
}
