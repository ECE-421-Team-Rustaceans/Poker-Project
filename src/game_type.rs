use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

/// GameType enum
/// 
/// Below are the supported poker game types by this server. Other game
/// types may be added in the future. Currently, we only support draw
/// style poker.
#[derive(Serialize, Deserialize, Debug, Clone, EnumIter)]
pub enum GameType {
    FiveCardDraw,
    SevenCardStud,
    TexasHoldem,
}

impl std::fmt::Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameType::FiveCardDraw => write!(f, "Five Card Draw"),
            GameType::SevenCardStud => write!(f, "Seven Card Stud"),
            GameType::TexasHoldem => write!(f, "Texas Hold'em"),
        }
    }
}
