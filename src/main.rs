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
mod game_type;
use database::db_handler::DbHandler;
use input::cli_input::CliInput;
use player::Player;
use rules::{five_card_draw::FiveCardDraw, Rules};

use uuid::Uuid;

fn main() {
    println!("poker time");

    let mut player1 = Player::new(1000, Uuid::now_v7());
    let mut player2 = Player::new(1000, Uuid::now_v7());
    let players= vec![&mut player1, &mut player2];
    let mut rules = FiveCardDraw::<CliInput>::new(1000, 2, DbHandler::new_dummy(), Uuid::now_v7());
    rules.play_round(players);
}
