use serde::{ Deserialize, Serialize, Serializer, ser::SerializeSeq };
use uuid::serde::simple;
use uuid::Uuid;

use crate::card::Card;
use crate::action::Action;


/// GameType enum
/// 
/// Below are the supported poker game types by this server. Other game
/// types may be added in the future. Currently, we only support draw
/// style poker.
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    #[serde(with = "uuid::serde::simple")]
    _id: Uuid,
    game_type: GameType,
}


fn simple_uuids<S: Serializer>(uuid_list: &Vec<Uuid>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = s.serialize_seq(Some(uuid_list.len()))?;
    for id in uuid_list {
        let id_str = id.simple().to_string();
        seq.serialize_element(&id_str)?;
    }
    return seq.end();
}

/// Round struct
/// 
/// Information about one round (i.e. playing until money is won from the pot) is held
/// in this struct. Each round has a round_id along with a game_id it is associated with.
/// Rounds are comprised of player turns and are associated to a group of players that
/// played in that round. Players should not be able to join mid round.
#[derive(Serialize, Deserialize, Debug)]
pub struct Round {
    #[serde(with = "uuid::serde::simple")]
    pub _id: Uuid,
    #[serde(with = "uuid::serde::simple")]
    pub game_id: Uuid,
    #[serde(serialize_with = "simple_uuids")]
    pub turn_ids: Vec<Uuid>,
    #[serde(serialize_with = "simple_uuids")]
    pub player_ids: Vec<Uuid>,
}

/// Turn struct
/// 
/// Information about one player action (e.g. betting, fold, checking, etc.) is held
/// in this struct. Additionally info may be added for some types of actions (e.g.
/// betting will have an additional amount sub-field). Turns are identified by IDs
/// along with a round_id it is associated with. A player ID and their associated hand before 
/// their action is also stored in this struct. These turns are also grouped together
/// by "phases". The number of phases depends on the game type. 
#[derive(Serialize, Deserialize, Debug)]
pub struct Turn {
    #[serde(with = "uuid::serde::simple")]
    pub _id: Uuid,
    #[serde(with = "uuid::serde::simple")]
    pub round_id: Uuid,
    pub phase_num: usize,
    #[serde(with = "uuid::serde::simple")]
    pub acting_player_id: Uuid,
    pub hand: Vec<Card>,
    pub action: Action,
}

/// Account struct
/// 
/// These are recognized accounts on our system. Each account has a unique ID along
/// with any personal information. To play poker games on this server, you must
/// have an account on our system.
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    #[serde(with = "uuid::serde::simple")]
    pub _id: Uuid,
}