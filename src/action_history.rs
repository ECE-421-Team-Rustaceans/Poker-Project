use crate::{action::Action, player::Player, player_action::PlayerAction};

use std::cmp::max;

/// ActionHistory keeps a history/log of Players' Actions (PlayerActions).
/// It also provides useful methods for checks that game rules need to perform regularly,
/// such as checking if a player has folded.
pub struct ActionHistory {
    player_actions: Vec<PlayerAction>
}

impl ActionHistory {
    /// Create a new ActionHistory, which starts out blank
    pub fn new() -> ActionHistory {
        return ActionHistory {
            player_actions: Vec::new()
        };
    }

    /// Push a new PlayerAction to the history, which adds it to the end of the log
    pub fn push(&mut self, player_action: PlayerAction) {
        self.player_actions.push(player_action);
    }

    /// Get a Vector of references to all the Players to be found in the History
    pub fn players(&self) -> Vec<&Player> {
        let mut players: Vec<&Player> = Vec::new();
        for player_action in self.player_actions.iter() {
            if !players.contains(&player_action.player()) {
                players.push(player_action.player());
            }
        }
        return players;
    }

    /// Get whether a Player in the History has Folded or not.
    /// Returns true if the Player is found and has Folded as one of their Actions
    /// Returns false if the Player is found and has not Folded as any of their Actions
    /// Return false if the Player is not in the list of player actions
    pub fn player_has_folded(&self, player: &Player) -> bool {
        let players = self.players();
        if !players.contains(&player) {
            return false;
        }
        else {
            for player_action in self.player_actions.iter() {
                if player_action.action() == Action::Fold {
                    return true;
                }
            }
            return false;
        }
    }

    /// Get the current bet amount, obtained by going through all ante/bet/raise/allIn
    /// actions in the game so far, and getting the maximum bet value
    pub fn current_bet_amount(&self) -> u32 {
        let mut bet_amount = 0;
        for player_action in self.player_actions.iter() {
            match player_action.action() {
                Action::Ante(amount) => {
                    assert!(amount >= bet_amount);
                    bet_amount = amount;
                },
                Action::Bet(amount) => {
                    assert!(amount >= bet_amount);
                    bet_amount = amount;    
                },
                Action::Raise(amount) => {
                    bet_amount += amount;
                },
                Action::AllIn(amount) => {
                    bet_amount = max(amount, bet_amount);
                },
                _ => {}
            }
        }
        return bet_amount as u32;
    }

    /// Get the entire history, used for testing purposes, which is why it's private
    fn get_history(&self) -> &Vec<PlayerAction> {
        return &self.player_actions;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn constructor() {
        let _ = ActionHistory::new();
    }

    #[test]
    fn push() {
        let mut action_history = ActionHistory::new();
        assert_eq!(action_history.get_history().len(), 0);
        let player = Player::new(1000, Uuid::now_v7());
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
        let player = Player::new(1000, Uuid::now_v7());
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
        let player = Player::new(1000, Uuid::now_v7());
        assert_eq!(action_history.player_has_folded(&player), false);
        let action = Action::Check;
        let player_action = PlayerAction::new(&player, action.clone());
        action_history.push(player_action);
        assert_eq!(action_history.player_has_folded(&player), false);
        let action = Action::Fold;
        let player_action = PlayerAction::new(&player, action.clone());
        action_history.push(player_action);
        assert_eq!(action_history.player_has_folded(&player), true);
    }
}
