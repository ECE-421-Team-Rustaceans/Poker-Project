use std::io;

use crate::action_option::ActionOption;

// TODO: error checking
/// trait for input handling
pub trait Input {
    /// input handling for players
    fn input_player() -> Vec<String> {
        // TODO: implement error checking for out of bounds entries
        println!("enter number of players (2-10):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let num_players: usize = input
            .trim()
            .parse()
            .expect("not an integer");

        println!("\nenter names in playing order:");

        let mut players: Vec<String> = Vec::new();

        for i in 1..=num_players {
            let mut input = String::new();
            println!("enter name for player {}: ", i);
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            let name = input
                .trim()
                .to_string();
            players.push(name);
        }

        println!("\nplayers:");
        for (index, player) in players.iter().enumerate() {
            println!("player {}: {}", index + 1, player);
        }

        players 
        // enter either name or username to look at stats
    }

    fn input_game() {
        println!("\nselect a game:\n1 - five card draw\n2 - seven card draw\n3 - kansas city lowball");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let game: u32 = input
            .trim()
            .parse()
            .expect("not an integer");
        
        // convert integer game option to game to send to game, or have game handle it
        // something like Game::play_game(players, game)
    }

    /// list of avaialble actions to action option
    fn input_action_options(possible_actions: Vec<ActionOption>) -> ActionOption {
        println!("\nselect an action:");
        for (i, action) in possible_actions.iter().enumerate() {
            println!("{} - {:#?}", i, action);
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let action_index: usize = input
            .trim()
            .parse()
            .expect("not an integer");

        possible_actions[action_index]
    }

    /// action option to action with the number
    fn input_action_option(action_option: ActionOption) -> u32 {
        println!("\n enter amount: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        let amount: u32 = input
            .trim()
            .parse()
            .expect("not an integer");

        // for now, this will just return some number for 
        // corresponding action of action option since conversion 
        // from action <-> action option isn't implemented yet

        amount
    }
}
