use std::io;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use uuid::Uuid;

use crate::{database::db_handler::DbHandler, game::Game, game_type::GameType, input::cli_input::CliInput, player::Player, rules::{five_card_draw::FiveCardDraw, seven_card_stud::SevenCardStud, texas_holdem::TexasHoldem, Rules}};

#[derive(EnumIter)]
enum StartPageOption {
    LogIn,
    Register,
    Exit
}

impl std::fmt::Display for StartPageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartPageOption::LogIn => write!(f, "Log In"),
            StartPageOption::Register => write!(f, "Register"),
            StartPageOption::Exit => write!(f, "Exit"),
        }
    }
}

#[derive(EnumIter)]
enum HomePageOption {
    CreateLobby,
    JoinLobby,
    ViewStatistics,
    LogOut
}

impl std::fmt::Display for HomePageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HomePageOption::CreateLobby => write!(f, "Create Lobby"),
            HomePageOption::JoinLobby => write!(f, "Join Lobby"),
            HomePageOption::ViewStatistics => write!(f, "View Statistics"),
            HomePageOption::LogOut => write!(f, "Log Out"),
        }
    }
}

#[derive(EnumIter)]
enum LobbyCreationPageOption {
    SelectGameType,
    SelectRaiseLimit,
    SelectMinimumBet,
    Finish,
    Cancel
}

impl std::fmt::Display for LobbyCreationPageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LobbyCreationPageOption::SelectGameType => write!(f, "Select Game Type"),
            LobbyCreationPageOption::SelectRaiseLimit => write!(f, "Select Raise Limit"),
            LobbyCreationPageOption::SelectMinimumBet => write!(f, "Select Minimum Bet"),
            LobbyCreationPageOption::Finish => write!(f, "Finish"),
            LobbyCreationPageOption::Cancel => write!(f, "Cancel"),
        }
    }
}

#[derive(EnumIter)]
enum LobbyPageOption {
    RefreshPlayerList,
    AddLocalPlayer, // TODO: this is only here for CLI, as there is otherwise no way to have more than one player
    StartRound,
    LeaveLobby
}

impl std::fmt::Display for LobbyPageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LobbyPageOption::RefreshPlayerList => write!(f, "Refresh Player List"),
            LobbyPageOption::StartRound => write!(f, "Start Round"),
            LobbyPageOption::LeaveLobby => write!(f, "Leave Lobby"),
            LobbyPageOption::AddLocalPlayer => write!(f, "Add Local Player"),
        }
    }
}

pub struct MenuNavigation;

impl MenuNavigation {
    pub async fn start_page() {
        loop {
            println!("\nStart Page");
            println!("Select an option:");
            for (i, page) in StartPageOption::iter().enumerate() {
                println!("{} - {}", i, page);
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let next_page = match input.trim().parse::<usize>() {
                Ok(index) if index < StartPageOption::iter().count() => {
                    StartPageOption::iter().nth(index).unwrap()
                },
                _ => {
                    println!("invalid input, please enter a number between 0 and {}:", StartPageOption::iter().count()-1);
                    continue;
                },
            };
            match next_page {
                StartPageOption::LogIn => MenuNavigation::home_page(MenuNavigation::login_page()).await,
                StartPageOption::Register => MenuNavigation::home_page(MenuNavigation::register_page()).await,
                StartPageOption::Exit => break,
            };
        }
    }

    pub fn login_page() -> Player {
        loop {
            println!("\nLogin Page");
            println!("Enter your username:");
            println!("This has not yet been implemented! redirecting to register page");
            break MenuNavigation::register_page();
        }
    }

    pub fn register_page() -> Player {
        loop {
            println!("\nRegister Page");
            println!("Enter a username:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            input = input.trim().to_string();
            if input.len() == 0 {
                println!("username cannot be blank");
                continue;
            }
            return Player::new(Uuid::now_v7(), input, 1000);
        }
    }

    pub async fn home_page(player: Player) {
        loop {
            println!("\nHome Page");
            println!("Select an option:");
            for (i, page) in HomePageOption::iter().enumerate() {
                println!("{} - {}", i, page);
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let next_page = match input.trim().parse::<usize>() {
                Ok(index) if index < HomePageOption::iter().count() => {
                    HomePageOption::iter().nth(index).unwrap()
                },
                _ => {
                    println!("invalid input, please enter a number between 0 and {}:", HomePageOption::iter().count()-1);
                    continue;
                },
            };
            match next_page {
                HomePageOption::CreateLobby => MenuNavigation::lobby_creation_page(player.clone()).await, // FIXME: should not be cloning Player, because their balance may change but this copy will not see that change
                HomePageOption::JoinLobby => MenuNavigation::lobby_join_page(player.clone()).await, // FIXME: should not be cloning Player, because their balance may change but this copy will not see that change
                HomePageOption::ViewStatistics => MenuNavigation::game_statistics_page(),
                HomePageOption::LogOut => break,
            };
        }
    }

    pub async fn lobby_creation_page(player: Player) {
        let mut game_type = GameType::TexasHoldem;
        let mut raise_limit = 1000;
        let mut minimum_bet = 2;
        loop {
            println!("\nLobby Creation Page");
            println!("Currently Selected Game Type: {}", game_type);
            println!("Currently Selected Raise Limit: {}", raise_limit);
            println!("Currently Selected Minimum Bet: {}", minimum_bet);
            println!("Select an option:");
            for (i, page) in LobbyCreationPageOption::iter().enumerate() {
                println!("{} - {}", i, page);
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let next_page = match input.trim().parse::<usize>() {
                Ok(index) if index < LobbyCreationPageOption::iter().count() => {
                    LobbyCreationPageOption::iter().nth(index).unwrap()
                },
                _ => {
                    println!("invalid input, please enter a number between 0 and {}:", LobbyCreationPageOption::iter().count()-1);
                    continue;
                },
            };
            match next_page {
                LobbyCreationPageOption::SelectGameType => game_type = MenuNavigation::game_type_selection_page(),
                LobbyCreationPageOption::SelectRaiseLimit => raise_limit = MenuNavigation::raise_limit_selection_page(),
                LobbyCreationPageOption::SelectMinimumBet => minimum_bet = MenuNavigation::minimum_bet_selection_page(),
                LobbyCreationPageOption::Finish => {
                    match game_type {
                        GameType::FiveCardDraw => {
                            MenuNavigation::lobby_page(player, Game::<FiveCardDraw<CliInput>>::new(raise_limit, minimum_bet, DbHandler::new_dummy())).await;
                            break;
                        },
                        GameType::SevenCardStud => {
                            MenuNavigation::lobby_page(player, Game::<SevenCardStud<CliInput>>::new(raise_limit, minimum_bet, DbHandler::new_dummy())).await;
                            break;
                        },
                        GameType::TexasHoldem => {
                            MenuNavigation::lobby_page(player, Game::<TexasHoldem<CliInput>>::new(raise_limit, minimum_bet, DbHandler::new_dummy())).await;
                            break;
                        },
                    };
                },
                LobbyCreationPageOption::Cancel => break,
            };
        }
    }

    pub fn game_type_selection_page() -> GameType {
        loop {
            println!("\nGame Type Selection Page");
            println!("Select an option:");
            for (i, game_type) in GameType::iter().enumerate() {
                println!("{} - {}", i, game_type);
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            match input.trim().parse::<usize>() {
                Ok(index) if index < GameType::iter().count() => return GameType::iter().nth(index).unwrap(),
                _ => println!("invalid input, please enter a number between 0 and {}:", GameType::iter().count()-1),
            };
        }
    }

    pub fn raise_limit_selection_page() -> u32 {
        loop {
            println!("\nRaise Limit Selection Page");
            println!("Set a raise limit:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            match input.trim().parse::<u32>() {
                Ok(amount) => {
                    if amount <= 0 {
                        println!("You must enter a positive and non-zero raise limit");
                    }
                    else {
                        return amount;
                    }
                },
                _ => println!("You must enter a number")
            }
        }
    }

    pub fn minimum_bet_selection_page() -> u32 {
        loop {
            println!("\nMinimum Bet Selection Page");
            println!("Set a minimum bet:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            match input.trim().parse::<u32>() {
                Ok(amount) => {
                    if amount <= 0 {
                        println!("You must enter a positive and non-zero minimum bet");
                    }
                    else {
                        return amount;
                    }
                },
                _ => println!("You must enter a number")
            }
        }
    }

    pub async fn lobby_page<T: Rules>(player: Player, mut game: Game<T>) {
        game.add_player(player).unwrap();
        loop {
            println!("\nLobby Page");
            println!("Current players: {:?}", game.players().iter().map(|player| player.name()).collect::<Vec<&str>>());
            println!("Select an option:");
            for (i, option) in LobbyPageOption::iter().enumerate() {
                println!("{} - {}", i, option);
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let option = match input.trim().parse::<usize>() {
                Ok(index) if index < LobbyPageOption::iter().count() => {
                    LobbyPageOption::iter().nth(index).unwrap()
                },
                _ => {
                    println!("invalid input, please enter a number between 0 and {}:", LobbyPageOption::iter().count()-1);
                    continue;
                },
            };
            match option {
                LobbyPageOption::RefreshPlayerList => continue,
                LobbyPageOption::StartRound => game.play_game().await,
                LobbyPageOption::LeaveLobby => break,
                LobbyPageOption::AddLocalPlayer => {
                    game.add_player(MenuNavigation::register_page()).unwrap();
                },
            };
        }
    }

    pub async fn lobby_join_page(player: Player) {
        loop {
            println!("\nLobby Join Page");
            println!("Select a lobby:");
            println!("This has not yet been implemented!");
            break;
        }
    }

    pub fn game_statistics_page() {
        loop {
            println!("\nGame Statistics Page");
            println!("Select a game:");
            println!("This has not yet been implemented!");
            break;
        }
    }
}
