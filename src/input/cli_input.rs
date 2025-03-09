use super::*;

pub struct CliInput;

impl Input for CliInput {
    fn input_player() -> Vec<String> {

        let num_players: usize;
        
        loop {
            println!("enter number of players (2-6):");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");
            match input.trim().parse::<usize>() {
                Ok(value) if (2..=6).contains(&value) =>  {
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

    fn input_variation() -> usize {
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

    fn input_action_options(possible_actions: Vec<ActionOption>) -> ActionOption {
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

    fn input_action_option(action_option: ActionOption, limit: u32) -> u32 {
        loop {
            println!("\n enter amount: ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            match input.trim().parse::<u32>() {
                Ok(amount) if (1..=limit).contains(&amount) => return amount, 
                _ => println!("invalid input. please enter a number between 1 and {}", limit),
            }
        }
        // for now, this will just return some number for 
        // corresponding action of action option since conversion 
        // from action <-> action option isn't implemented yet
    }
    
    fn request_raise_amount(limit: u32) -> u32 {
        todo!()
    }
    
    fn request_replace_cards(cards: Vec<&Card>) -> Vec<&Card> {
        todo!()
    }
    
    fn display_cards(cards: Vec<&Card>) {
        todo!()
    }
    
    fn display_current_player_index(player_index: u32) {
        todo!()
    }
}
