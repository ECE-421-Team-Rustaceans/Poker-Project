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
}
