use crate::player::Player;
use super::Rules;

pub struct KansasCityLowball {

}

impl<'a> Rules<'a> for KansasCityLowball {
    fn play_round(&mut self, players: Vec<&'a mut Player>) {
        todo!()
    }
}
