use poker_project_rustaceans::server;

#[tokio::main]
async fn main() {
    println!("poker time");

    // let mut player1 = Player::new(Uuid::now_v7(), "player".to_string(), 1000);
    // let mut player2 = Player::new(Uuid::now_v7(), "player".to_string(), 1000);
    // let raise_limit = 1000;
    // let minimum_bet = 2;
    // let db_handler = DbHandler::new_dummy();
    // let mut game: Game<FiveCardDraw<CliInput>> = Game::new(raise_limit, minimum_bet, db_handler);
    // game.add_player(player1).unwrap();
    // game.add_player(player2).unwrap();
    // game.play_game().await;
    server::run_server().await;
}
