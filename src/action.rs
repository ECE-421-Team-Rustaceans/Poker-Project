use serde::{ Deserialize, Serialize };

/// Player Action enum
/// 
/// This contains action types that players can do throuhgout a game.
/// Money related actions like Ante, Bet, etc. have values which should
/// be the money they stake IN TOTAL.
/// 
/// e.g. 
/// In a two player game (P1 and P2), if they both pay the ante of $2,
/// then if P1 bets $5, then the corresponding action is Bet(5).
///
/// If P2 were to raise by $5 (i.e. raise $10) then the corresponding action
/// is Raise(10).
/// 
/// Finally, if P1 has a balance of $1000 and they go all-in, then the corresponding
/// action should be AllIn(1000).
/// 
/// Win and Lose actions are for book keeping and will be added onto the pot history
/// after dividing the winnings for a particular round as turns in a separte phase.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Ante(usize),
    Call,
    Bet(usize),
    Raise(usize),
    Check,
    AllIn(usize),
    Fold,
    Replace(usize),
    Win(usize),
    Lose(usize),
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Ante(l0), Self::Ante(r0)) => l0 == r0,
            (Self::Bet(l0), Self::Bet(r0)) => l0 == r0,
            (Self::Raise(l0), Self::Raise(r0)) => l0 == r0,
            (Self::AllIn(l0), Self::AllIn(r0)) => l0 == r0,
            (Self::Replace(l0), Self::Replace(r0)) => l0 == r0,
            (Self::Win(l0), Self::Win(r0)) => l0 == r0,
            (Self::Lose(l0), Self::Lose(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
