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

#[cfg(test)]
mod tests {
    use crate::{action::Action, player::Player};

    use super::PlayerAction;

    #[test]
    fn constructor_and_getters() {
        let player = Player::new();
        let action = Action::Fold;
        let player_action = PlayerAction::new(&player, action.clone());
        assert_eq!(*player_action.player(), player);
        assert_eq!(player_action.action(), action);
    }
}
