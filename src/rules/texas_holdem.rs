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

pub struct TexasHoldem<I: Input> {
    players: Vec<Player>,
    deck: Deck,
    dealer_position: usize,
    current_player_index: usize,
    raise_limit: u32,
    bring_in: u32,
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
        self.pot.add_turn(&first_blind_player.account_id(), Action::Ante(1), 0, first_blind_player.peek_at_cards().iter().map(|&card| card.clone()).collect());
        first_blind_player.bet(1).unwrap();
        self.increment_player_index();

        let second_blind_player = match self.players.get_mut(self.dealer_position+1) {
            Some(player) => player,
            None => {
                self.players.get_mut(0).expect("Expected a non-zero number of players")
            }
        };
        self.pot.add_turn(&second_blind_player.account_id(), Action::Ante(2), 0, second_blind_player.peek_at_cards().iter().map(|&card| card.clone()).collect());
        second_blind_player.bet(2).unwrap();
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

            let player: &mut Player = &mut self.players.get_mut(self.current_player_index).expect("Expected a player at this index, but there was None");

            if !(self.pot.player_has_folded(&player.account_id()) || player.balance() == 0) {
                self.input.display_current_player_index(self.current_player_index as u32);
                self.input.display_cards(player.peek_at_cards());

                if !raise_has_occurred && self.pot.get_call_amount() == self.pot.get_player_stake(&player.account_id()) {
                    // the big blind can check because they already paid a full bet, and on the second round, everyone can check if nobody raises
                    let action_options = vec![ActionOption::Check, ActionOption::Raise, ActionOption::Fold];
                    let chosen_action_option: ActionOption = self.input.input_action_options(action_options);

                    let player_raise_limit = min(self.raise_limit, player.balance() as u32);

                    let action = match chosen_action_option {
                        ActionOption::Check => Action::Check,
                        ActionOption::Raise => Action::Raise(self.pot.get_call_amount() as usize + self.input.request_raise_amount(player_raise_limit) as usize),
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
                    let chosen_action_option: ActionOption = self.input.input_action_options(action_options);

                    let current_bet_amount = self.pot.get_call_amount() as u32;
                    if player.balance() as u32 > current_bet_amount {
                        let player_raise_limit = min(self.raise_limit, player.balance() as u32 - current_bet_amount);
                        let action = match chosen_action_option {
                            ActionOption::Call => Action::Call,
                            ActionOption::Raise => Action::Raise(<i64 as TryInto<usize>>::try_into(self.pot.get_call_amount()).unwrap() + self.input.request_raise_amount(player_raise_limit) as usize),
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

    fn play_phase_five(&mut self) {
        self.play_bet_phase(5);
    }

    fn showdown(&mut self) {
        // show to each player everyone's cards (except folded)
        let start_player_index = self.current_player_index;
        let mut current_player_index = self.current_player_index;
        loop {
            let player: &Player = self.players.get(current_player_index).expect("Expected a player at this index, but there was None");

            if !self.pot.player_has_folded(&player.account_id()) {
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
        for (player_id, &winnings) in player_winnings_map.iter() {
            assert!(winnings >= 0);
            if winnings > 0 {
                let mut player_matches: Vec<&mut Player> = self.players.iter_mut().filter(|player| player.account_id() == *player_id).collect();
                assert_eq!(player_matches.len(), 1);
                let player_match = &mut player_matches[0];
                assert!(!self.pot.player_has_folded(&player_match.account_id()), "Player: {}, winning amount: {}", player_match.account_id(), winnings);
                player_match.win(winnings as usize);
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

    /// each non-folded player is dealt one card face up
    fn deal_up_cards(&mut self) -> Result<(), String> {
        let remaining_players = self.players.iter_mut()
            .filter(|player| !self.pot.player_has_folded(&player.account_id()));
        for player in remaining_players {
            player.obtain_card(self.deck.deal(true)?);
        }
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
    async fn play_round(&mut self, players: Vec<Player>) -> Result<(), &'static str> {
        if players.len() < 2 {
            return Err("Cannot start a game with less than 2 players");
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

        return Ok(());
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
            bring_in: minimum_bet,
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

}
