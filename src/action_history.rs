use crate::{action::Action, player::Player, player_action::PlayerAction};

pub struct ActionHistory<'a> {
    player_actions: Vec<PlayerAction<'a>>
}

impl<'a> ActionHistory<'a> {
    pub fn new() -> ActionHistory<'a> {
        return ActionHistory {
            player_actions: Vec::new()
        };
    }

    pub fn push(&mut self, player_action: PlayerAction<'a>) {
        self.player_actions.push(player_action);
    }

    pub fn players(&self) -> Vec<&Player> {
        let mut players: Vec<&Player> = Vec::new();
        for player_action in self.player_actions.iter() {
            if !players.contains(&player_action.player()) {
                players.push(player_action.player());
            }
        }
        return players;
    }

    pub fn player_has_folded(&self, player: &Player) -> Result<bool, &'static str> {
        let players = self.players();
        if !players.contains(&player) {
            return Err("Player was not found in the action history");
        }
        else {
            for player_action in self.player_actions.iter() {
                if player_action.action() == Action::Fold {
                    return Ok(true);
                }
            }
            return Ok(false);
        }
    }

    fn get_history(&self) -> &Vec<PlayerAction> {
        return &self.player_actions;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let _ = ActionHistory::new();
    }

    #[test]
    fn push() {
        let mut action_history = ActionHistory::new();
        assert_eq!(action_history.get_history().len(), 0);
        let player = Player::new();
        let action = Action::Fold;
        let player_action = PlayerAction::new(&player, action.clone());
        action_history.push(player_action);
        assert_eq!(action_history.get_history().len(), 1);
        let player_action = PlayerAction::new(&player, action.clone());
        assert_eq!(action_history.get_history().get(0).unwrap(), &player_action);
    }

    #[test]
    fn players() {
        let mut action_history = ActionHistory::new();
        assert_eq!(action_history.players().len(), 0);
        let player = Player::new();
        let action = Action::Fold;
        let player_action = PlayerAction::new(&player, action.clone());
        action_history.push(player_action);
        assert_eq!(action_history.players().len(), 1);
        let mut players: Vec<&Player> = Vec::new();
        players.push(&player);
        assert_eq!(action_history.players(), players);
    }

    #[test]
    fn player_has_folded() {
        let mut action_history = ActionHistory::new();
        let player = Player::new();
        assert_eq!(action_history.player_has_folded(&player), Err("Player was not found in the action history"));
        let action = Action::Check;
        let player_action = PlayerAction::new(&player, action.clone());
        action_history.push(player_action);
        assert_eq!(action_history.player_has_folded(&player), Ok(false));
        let action = Action::Fold;
        let player_action = PlayerAction::new(&player, action.clone());
        action_history.push(player_action);
        assert_eq!(action_history.player_has_folded(&player), Ok(true));
    }
}
