use std::io;

use poker_project_rustaceans::menu_navigation::MenuNavigation;
use poker_project_rustaceans::server;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
enum ModeSelection {
    CommandLine,
    ServerClient,
    Exit
}

impl std::fmt::Display for ModeSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModeSelection::CommandLine => write!(f, "Command Line"),
            ModeSelection::ServerClient => write!(f, "Server-Client"),
            ModeSelection::Exit => write!(f, "Exit"),
        }
    }
}

#[tokio::main]
async fn main() {
    loop {
        println!("\nPoker Project Rustaceans Dealer");
        println!("Select an execution mode:");
        for (i, mode) in ModeSelection::iter().enumerate() {
            println!("{} - {}", i, mode);
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let mode_selection = match input.trim().parse::<usize>() {
            Ok(index) if index < ModeSelection::iter().count() => {
                ModeSelection::iter().nth(index).unwrap()
            },
            _ => {
                println!("invalid input, please enter a number between 0 and {}:", ModeSelection::iter().count()-1);
                continue;
            },
        };
        match mode_selection {
            ModeSelection::CommandLine => MenuNavigation::start_page().await,
            ModeSelection::ServerClient => server::run_server().await,
            ModeSelection::Exit => break,
        };
    }
}
