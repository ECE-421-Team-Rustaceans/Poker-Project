use std::io::{self, Read};

use crate::action::Action;
use crate::action_option::ActionOption;

/// trait for input handling
pub trait Input {
    /// input handling for players, 
    /// returns a list of gamer names
    fn input_player(&self) -> Vec<String> {

        let num_players: usize;
        
        loop {
            println!("enter number of players (2-10):");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            match input.trim().parse::<usize>() {
                Ok(value) if (2..=10).contains(&value) =>  {
                    num_players = value;
                    break;
                }
                _ => println!("invalid input")
            }
        }   
        
        println!("\nenter names in playing order:");

        let mut players: Vec<String> = Vec::new();

        for i in 1..=num_players {

            let name = loop {
                println!("enter name for player {}: ", i);

                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("failed to read line");
                let name = input
                    .trim()
                    .to_string();

                if !name.is_empty() {
                    break name;
                }
                println!("invalid input");
            };
            players.push(name);
        }

        println!("\nplayers:");
        for (index, player) in players.iter().enumerate() {
            println!("player {}: {}", index + 1, player);
        }

        players
    }

    /// for user input to pick which poker variation to play. 
    /// will return a usize from 1-3, which correspond to different poker variations
    fn input_variation(&self) -> usize {
        loop {
            println!("\nselect a game:\n1 - five card draw\n2 - seven card draw\n3 - kansas city lowball");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            match input.trim().parse::<usize>(){
                Ok(game) if (1..=3).contains(&game) => return game,
                _ => println!("invalid! enter 1, 2, or 3."),
            }
        }
    }

    /// input a list of available actions for the user to choose from
    /// and output a action option that the user has chosen
    fn input_action_options(&self, possible_actions: Vec<ActionOption>) -> ActionOption {
        loop {
            println!("\nselect an action:");
            for (i, action) in possible_actions.iter().enumerate() {
                println!("{} - {:#?}", i, action);
            }
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            match input.trim().parse::<usize>() {
                Ok(index) if index < possible_actions.len() => return possible_actions[index],
                _ => println!("invalid input, please enter a number between 0 and {}:", possible_actions.len() - 1),
            }
        }
    }

    /// action option to action with the number
    fn input_action_option(&self, action_option: ActionOption, limit: u32) -> u32 {
        loop {
            println!("\n enter amount: ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            match input.trim().parse::<u32>() {
                Ok(amount) if (1..=limit).contains(&amount) => return amount, 
                _ => println!("invalud input. please enter a number between 1 and {}", limit),
            }
        }
        // for now, this will just return some number for 
        // corresponding action of action option since conversion 
        // from action <-> action option isn't implemented yet
    }
}