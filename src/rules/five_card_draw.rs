use super::Rules;

pub struct FiveCardDraw {

}

impl FiveCardDraw {
    fn play_round_one(&self) {
        todo!()
    }

    fn play_round_two(&self) {
        todo!()
    }
}

impl Rules for FiveCardDraw {
    fn play_game(&self) {
        self.play_round_one();
        self.play_round_two();
    }
}
