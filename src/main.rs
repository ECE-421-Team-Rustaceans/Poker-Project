use poker_project_rustaceans::menu_navigation::MenuNavigation;

#[tokio::main]
async fn main() {
    MenuNavigation::start_page().await;
}
