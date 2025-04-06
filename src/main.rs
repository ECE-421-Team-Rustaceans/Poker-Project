use poker_project_rustaceans::server;

#[tokio::main]
async fn main() {
    server::run_server().await;
}
