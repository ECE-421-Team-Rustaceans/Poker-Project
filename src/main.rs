use poker_project_rustaceans::menu_navigation::MenuNavigation;
use poker_project_rustaceans::server;

enum ModeSelection {
    CommandLine,
    ServerClient
}

#[tokio::main]
async fn main() {
    let mode_selection = ModeSelection::CommandLine;
    match mode_selection {
        ModeSelection::CommandLine => MenuNavigation::start_page().await,
        ModeSelection::ServerClient => server::run_server().await,
    }
}
