use crate::player::Player;
use super::Rules;

pub struct SevenCardDraw {

}

impl<'a> Rules<'a> for SevenCardDraw {
    fn play_round(&mut self, players: Vec<&'a mut Player>) {
        todo!()
    }
}
