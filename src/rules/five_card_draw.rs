use crate::action_history::ActionHistory;
use crate::deck::Deck;
use crate::input::Input;
use crate::player::Player;
use crate::player_action::PlayerAction;
use super::Rules;
use crate::action_option::ActionOption;
use crate::action::Action;

pub struct FiveCardDraw<'a, I: Input> {
    players: Vec<&'a mut Player>,
    deck: Deck,
    dealer_position: usize,
    current_player_index: usize,
    input: I,
    action_history: ActionHistory
}

impl<'a, I: Input> FiveCardDraw<'a, I> {
    pub fn new(raise_limit: u32) -> FiveCardDraw<'a, I> {
        let players = Vec::new();
        let deck = Deck::new();
        let dealer_position = 0_usize;
        let current_player_index = 0_usize;
        let action_history = ActionHistory::new();
        let input = I::new();
        return FiveCardDraw {
            players,
            deck,
            dealer_position,
            current_player_index,
            input,
            action_history
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

    fn play_phase_one(&mut self) {
        // betting on this phase starts with the first blind player (player at self.dealer_position)
        self.current_player_index = self.dealer_position;
        let mut last_raise_player= self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None");
        loop {
            let player = self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None");
            let action_options = vec![ActionOption::Call, ActionOption::Raise, ActionOption::Fold];
            // if there are no raises, the small blind only needs to complete half-bet to stay in,
            // and the big blind can check because they already paid a full bet
            let chosen_action_option: ActionOption = self.input.input_action_options(action_options);
            let action: Action;

            match chosen_action_option {
                ActionOption::Ante => panic!("Player managed to select an impossible Action!"),
                ActionOption::AllIn => panic!("Player managed to select an impossible Action!"),
                ActionOption::Win => panic!("Player managed to select an impossible Action!"),
                ActionOption::Lose => panic!("Player managed to select an impossible Action!"),
                ActionOption::Check => panic!("Player managed to select an impossible Action!"),
                ActionOption::Bet => panic!("Player managed to select an impossible Action!"),
                ActionOption::Replace => panic!("Player managed to select an impossible Action!"),
                ActionOption::Call => action = Action::Call,
                ActionOption::Raise => action = Action::Raise(0), // TODO: request and validate user input for this
                ActionOption::Fold => action = Action::Fold,
            };

            match action {
                Action::Ante(_) => panic!("Player managed to perform an impossible Action!"),
                Action::AllIn(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Win(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Lose(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Check => panic!("Player managed to perform an impossible Action!"),
                Action::Bet(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Replace(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Call => {
                    // TODO: update Player wallet and Pot
                },
                Action::Raise(amount) => {
                    last_raise_player = player;
                    // TODO: update Player wallet and Pot
                },
                Action::Fold => {},
            }

            let player_action = PlayerAction::new(&player, action);
            self.action_history.push(player_action);

            self.current_player_index += 1;
            // wrap the player index around
            if self.current_player_index == self.players.len() {
                self.current_player_index = 0;
            }

            if self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None") == last_raise_player {
                // the next player is the player who last raised,
                // which means that all bets have been matched,
                // and it is time to move on to the next phase
                break;
            }
        }
    }

    fn play_draw_phase(&mut self) {
        // house rules: players may discard as many cards as they wish to draw new replacements
        // the exception is if there are not enough cards left in the deck to do so
        let start_player_index = self.current_player_index;
        loop {
            let mut player = self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None");
            let action_options = vec![ActionOption::Replace, ActionOption::Check];
            let chosen_action_option: ActionOption = self.input.input_action_options(action_options);
            let action: Action;

            match chosen_action_option {
                ActionOption::Ante => panic!("Player managed to select an impossible Action!"),
                ActionOption::Call => panic!("Player managed to select an impossible Action!"),
                ActionOption::Raise => panic!("Player managed to select an impossible Action!"),
                ActionOption::AllIn => panic!("Player managed to select an impossible Action!"),
                ActionOption::Win => panic!("Player managed to select an impossible Action!"),
                ActionOption::Lose => panic!("Player managed to select an impossible Action!"),
                ActionOption::Bet => panic!("Player managed to select an impossible Action!"),
                ActionOption::Fold => panic!("Player managed to select an impossible Action!"),
                ActionOption::Replace => action = Action::Replace(Vec::new()), // TODO: request and validate user input for this
                ActionOption::Check => action = Action::Check,
            };

            match action {
                Action::Ante(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Call => panic!("Player managed to perform an impossible Action!"),
                Action::Bet(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Raise(_) => panic!("Player managed to perform an impossible Action!"),
                Action::AllIn(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Fold => panic!("Player managed to perform an impossible Action!"),
                Action::Win(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Lose(_) => panic!("Player managed to perform an impossible Action!"),
                Action::Replace(_) => {
                    // TODO: update Player cards by drawing new ones from Deck and replacing
                },
                Action::Check => {
                    // do nothing, Player has chosen not to Replace any Cards
                },
            }

            let player_action = PlayerAction::new(&player, action);
            self.action_history.push(player_action);

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
            let mut player = self.players.get(self.current_player_index).expect("Expected a player at this index, but there was None");
            let action_options = vec![ActionOption::Check, ActionOption::Bet, ActionOption::Fold];
            let chosen_action_option: ActionOption = self.input.input_action_options(action_options);
            let action: Action;

            match chosen_action_option {
                ActionOption::Ante => panic!("Player managed to select an impossible Action!"),
                ActionOption::Call => panic!("Player managed to select an impossible Action!"),
                ActionOption::Raise => panic!("Player managed to select an impossible Action!"),
                ActionOption::AllIn => panic!("Player managed to select an impossible Action!"),
                ActionOption::Replace => panic!("Player managed to select an impossible Action!"),
                ActionOption::Win => panic!("Player managed to select an impossible Action!"),
                ActionOption::Lose => panic!("Player managed to select an impossible Action!"),
                ActionOption::Check => action = Action::Check,
                ActionOption::Bet => action = Action::Bet(0), // TODO: request and validate user input for this
                ActionOption::Fold => action = Action::Fold,
            };

            match action {
                Action::Ante(_) => todo!(),
                Action::Call => todo!(),
                Action::Raise(_) => todo!(),
                Action::AllIn(_) => todo!(),
                Action::Replace(_) => todo!(),
                Action::Win(_) => todo!(),
                Action::Lose(_) => todo!(),
                Action::Check => {},
                Action::Bet(amount) => {
                    // TODO: update Player wallet and Pot
                },
                Action::Fold => {},
            }

            let player_action = PlayerAction::new(&player, action);
            self.action_history.push(player_action);

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
    fn play_round(&mut self, players: Vec<&'a mut Player>) -> Result<(), &'static str> { // FIXME: merge new and play_round as they are the same
        self.play_blinds();
        self.deal_initial_cards().unwrap();
        self.play_phase_one();
        self.play_draw_phase();
        self.play_phase_two();
        self.return_player_cards();
        self.increment_dealer_position();
        return Ok(());
    }
}

// FIXME: need to account for players folding... not really accounted for right now
