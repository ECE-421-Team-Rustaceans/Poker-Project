use crate::player::Player;
use super::Rules;

pub struct TexasHoldem {

}

impl<'a> Rules<'a> for TexasHoldem {
    fn play_round(&mut self, players: Vec<&'a mut Player>) -> Result<(), &'static str> {
        todo!()
    }
}
