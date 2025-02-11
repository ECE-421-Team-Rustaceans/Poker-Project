use crate::{action::Action, player::Player};

#[derive(Debug)]
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

impl<'a> PartialEq for PlayerAction<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.action == other.action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_and_getters() {
        let player = Player::new();
        let action = Action::Fold;
        let player_action = PlayerAction::new(&player, action.clone());
        assert_eq!(*player_action.player(), player);
        assert_eq!(player_action.action(), action);
    }
}
