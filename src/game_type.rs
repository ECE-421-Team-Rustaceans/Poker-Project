use serde::{Deserialize, Serialize};

/// GameType enum
/// 
/// Below are the supported poker game types by this server. Other game
/// types may be added in the future. Currently, we only support draw
/// style poker.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameType {
    FiveCardDraw,
    SevenCardStud,
    KansasCityLowball,
}
