use crate::deck::Deck;
use super::Rules;
use crate::player::Player;

pub struct FiveCardDraw<'a> {
    players: Vec<&'a Player>,
    deck: Deck
}

impl<'a> FiveCardDraw<'a> {
    fn new(players: Vec<&Player>) -> FiveCardDraw {
        let deck = Deck::new();
        return FiveCardDraw {
            players,
            deck
        };
    }

    fn play_round_one(&self) {
        todo!()
    }

    fn play_round_two(&self) {
        todo!()
    }

    fn deal_initial_cards(&self) -> Result<(), String> {
        for player in self.players {
            player.obtain_card(self.deck.deal()?);
        }
        return Ok(());
    }
}

impl<'a> Rules for FiveCardDraw<'a> {
    fn play_game(&self) {
        self.play_round_one();
        self.play_round_two();
    }
}
