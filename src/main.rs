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
use game::Game;
use input::cli_input::CliInput;
use player::Player;
use rules::five_card_draw::FiveCardDraw;

use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut player1 = Player::new(1000, Uuid::now_v7());
    let mut player2 = Player::new(1000, Uuid::now_v7());
    let raise_limit = 1000;
    let minimum_bet = 2;
    let db_handler = DbHandler::new_dummy();
    let mut game: Game<FiveCardDraw<CliInput>> = Game::new(raise_limit, minimum_bet, db_handler);
    game.add_player(player1).unwrap();
    game.add_player(player2).unwrap();
    game.play_game().await;
}
