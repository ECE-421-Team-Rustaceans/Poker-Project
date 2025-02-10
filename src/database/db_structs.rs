use uuid::Uuid;

use crate::card::Card;


pub enum GameType {
    FiveCardDraw,
    SevenCardDraw,
    KansasCityLowball,
}

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


pub struct Game {
    game_id: Uuid,
    game_type: GameType,
}


pub struct Round {
    game_id: Uuid,
    round_id: Uuid,
    phase_ids: Vec<Uuid>,
    player_ids: Vec<Uuid>,
}


pub struct Turn {
    pub round_id: Uuid,
    pub turn_id: Uuid,
    pub phase_num: usize,
    pub acting_player_id: Uuid,
    pub hand: Vec<Card>,
    pub action: Action,
}


pub struct Account {
    account_id: Uuid,
}