use uuid::Uuid;

use crate::card::Card;


/// GameType enum
/// 
/// Below are the supported poker game types by this server. Other game
/// types may be added in the future. Currently, we only support draw
/// style poker.
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

/// Game struct
/// 
/// Information about one "poker game" (i.e. playing until there are insufficient players to play)
/// is held in this struct. game_id is used to identify unique games and to relate them to 
/// "rounds" within the database.
pub struct Game {
    game_id: Uuid,
    game_type: GameType,
}

/// Round struct
/// 
/// Information about one round (i.e. playing until money is won from the pot) is held
/// in this struct. Each round has a round_id along with a game_id it is associated with.
/// Rounds are comprised of player turns and are associated to a group of players that
/// played in that round. Players should not be able to join mid round.
pub struct Round {
    game_id: Uuid,
    round_id: Uuid,
    turn_ids: Vec<Uuid>,
    player_ids: Vec<Uuid>,
}

/// Turn struct
/// 
/// Information about one player action (e.g. betting, fold, checking, etc.) is held
/// in this struct. Additionally info may be added for some types of actions (e.g.
/// betting will have an additional amount sub-field). Turns are identified by IDs
/// along with a round_id it is associated with. A player ID and their associated hand before 
/// their action is also stored in this struct. These turns are also grouped together
/// by "phases". The number of phases depends on the game type. 
pub struct Turn {
    pub round_id: Uuid,
    pub turn_id: Uuid,
    pub phase_num: usize,
    pub acting_player_id: Uuid,
    pub hand: Vec<Card>,
    pub action: Action,
}

/// Account struct
/// 
/// These are recognized accounts on our system. Each account has a unique ID along
/// with any personal information. To play poker games on this server, you must
/// have an account on our system.
pub struct Account {
    account_id: Uuid,
}