use crate::player::Player;
use super::Rules;

pub struct SevenCardStud {

}

impl<'a> Rules<'a> for SevenCardStud {
    fn play_round(&mut self, players: Vec<&'a mut Player>) -> Result<(), &'static str> {
        todo!()
    }
}
