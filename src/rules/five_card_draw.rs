use crate::deck::Deck;
use super::Rules;
use crate::player::Player;

pub struct FiveCardDraw<'a> {
    players: Vec<&'a Player>,
    deck: Deck,
    dealer_position: usize
}

impl<'a> FiveCardDraw<'a> {
    fn new(players: Vec<&Player>) -> FiveCardDraw {
        let deck = Deck::new();
        let dealer_position = 0_usize;
        return FiveCardDraw {
            players,
            deck,
            dealer_position
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
        first_blind_player.bet();
        second_blind_player.bet();
    }

    fn play_phase_one(&mut self) {
        todo!()
    }

    fn play_phase_two(&mut self) {
        todo!()
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
    fn play_round(&mut self) {
        self.play_blinds();
        self.deal_initial_cards();
        self.play_phase_one();
        self.play_phase_two();
        self.increment_dealer_position();
    }
}
