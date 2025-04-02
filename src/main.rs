mod card;
mod deck;
mod rules;
mod input;
mod hand_rank;
mod pot;
mod database;
mod game;
mod player;
mod action;
mod action_option;
mod player_action;
mod action_history;
mod game_type;
use input::cli_input::CliInput;
use player::Player;
use rules::{five_card_draw::FiveCardDraw, Rules};

use uuid::Uuid;

fn main() {
    println!("poker time");

    let mut player1 = Player::new(Uuid::now_v7(), "player".to_string(), 1000);
    let mut player2 = Player::new(Uuid::now_v7(), "player".to_string(), 1000);
    let players= vec![&mut player1, &mut player2];
    let mut rules = FiveCardDraw::<CliInput>::new(1000);
    rules.play_round(players);
}
