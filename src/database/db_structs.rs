use uuid::Uuid;

use crate::card::Card;


pub enum GameType {
    FiveCardDraw,
    SevenCardDraw,
    KansasCityLowball,
}


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