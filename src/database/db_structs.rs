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