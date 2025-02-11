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
use player::Player;
use rules::{five_card_draw::FiveCardDraw, Rules};

fn main() {
    println!("poker time");

    let mut player1 = Player::new();
    let mut player2 = Player::new();
    let player11 = player1.clone();
    let player22 = player2.clone();
    let players= vec![&mut player1, &mut player2];
    let input = input::cli_input::CliInput {};
    let mut rules = FiveCardDraw::new(players, input);
    let players = vec![&player11, &player22];
    rules.play_round(players);
}
