use crate::{action::Action, player::Player};

pub struct PlayerAction<'a> {
    player: &'a Player,
    action: Action
}

impl<'a> PlayerAction<'a> {
    fn new(player: &Player, action: Action) -> PlayerAction {
        return PlayerAction {
            player,
            action
        };
    }

    fn player(&self) -> &Player {
        return &self.player;
    }

    fn action(&self) -> Action {
        return self.action.clone();
    }
}
