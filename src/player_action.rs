use crate::{action::Action, player::Player};

pub struct PlayerAction<'a> {
    player: &'a Player,
    action: Action
}

impl<'a> PlayerAction<'a> {
    pub fn new(player: &Player, action: Action) -> PlayerAction {
        return PlayerAction {
            player,
            action
        };
    }

    pub fn player(&self) -> &Player {
        return &self.player;
    }

    pub fn action(&self) -> Action {
        return self.action.clone();
    }
}
