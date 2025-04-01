use crate::{action::Action, player::Player};

/// PlayerAction simply groups together a Player and an Action they have performed
#[derive(Debug)]
pub struct PlayerAction {
    player: Player,
    action: Action
}

impl PlayerAction {
    /// Create a new PlayerAction, grouping together a reference to a Player, and an Action
    pub fn new(player: &Player, action: Action) -> PlayerAction {
        return PlayerAction {
            player: player.clone(),
            action
        };
    }

    /// Get this PlayerAction's Player
    pub fn player(&self) -> &Player {
        return &self.player;
    }

    /// Get this PlayerAction's Action
    pub fn action(&self) -> Action {
        return self.action.clone();
    }
}

impl PartialEq for PlayerAction {
    fn eq(&self, other: &Self) -> bool {
        self.player == other.player && self.action == other.action
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn constructor_and_getters() {
        let player = Player::new(1000, Uuid::now_v7());
        let action = Action::Fold;
        let player_action = PlayerAction::new(&player, action.clone());
        assert_eq!(*player_action.player(), player);
        assert_eq!(player_action.action(), action);
    }
}
