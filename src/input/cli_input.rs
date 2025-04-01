use super::*;
use crate::game_type::GameType;

pub struct CliInput;

impl Input for CliInput {
    fn new() -> Self {
        return Self;
    }

    fn input_player(&mut self) -> Vec<String> {

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

    // this will return an enum of the game (based on number inputted)
    // to be changed to reflect changed game variations
    fn input_variation(&mut self) -> GameType {
        loop {
            println!("\nselect a game:\n1 - five card draw\n2 - seven card draw\n3 - kansas city lowball");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            match input.trim().parse::<usize>(){
                Ok(1) => return GameType::FiveCardDraw,
                Ok(2) => return GameType::SevenCardStud,
                Ok(3) => return GameType::KansasCityLowball,
                _ => println!("invalid! enter 1, 2, or 3."),
            }
        }
    }

    fn input_action_options(&mut self, possible_actions: Vec<ActionOption>) -> ActionOption {
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

    fn request_raise_amount(&mut self, limit: u32) -> u32 {
        loop {
            println!("\nEnter amount to raise by, limit is {limit}: ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line from user input");

            match input.trim().parse::<u32>() {
                Ok(amount) => {
                    if amount <= 0 {
                        println!("You must enter a positive and non-zero raise amount");
                    }
                    else if amount > limit {
                        println!("You must enter an amount that is at most {limit}");
                    }
                    else {
                        return amount;
                    }
                },
                _ => println!("You must enter a number")
            }
        }
    }

    fn request_replace_cards<'a>(&mut self, cards: Vec<&'a Card>) -> Vec<&'a Card> {
        let mut selected_cards = Vec::new();
        for card in cards.iter() {
            selected_cards.push((false, *card));
        }
        loop {
            println!("\nHere are your {} cards:", selected_cards.len());
            for (card_index, (is_selected, card)) in selected_cards.iter().enumerate() {
                let selected_marker = match is_selected {
                    true => "[x]",
                    false => "[ ]",
                };
                println!("-> {selected_marker} {card_index}: {card} <-");
            }

            println!("Selected cards (which will be replaced) are marked with [x]");
            println!("Select a card to be replaced, or");
            println!("x: finish");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line from user input");

            match input.trim() {
                "x" => break,
                _ => {
                    match input.trim().parse::<u32>() {
                        Ok(value) => {
                            if value >= selected_cards.len().try_into().unwrap() {
                                println!("Invalid selection\nYou must select one of your cards");
                            }
                            else {
                                // toggle selection
                                selected_cards[value as usize].0 = !selected_cards[value as usize].0;
                            }
                        },
                        Err(_) => println!("Invalid selection"),
                    }
                }
            }
        }

        return selected_cards.iter()
            .filter(|(is_selected, _)| *is_selected)
            .map(|(_, card)| *card)
            .collect();
    }

    fn display_cards(&self, cards: Vec<&Card>) {
        println!("\nHere are your {} cards:", cards.len());
        for card in cards {
            println!("-> {card} <-");
        }
    }

    fn display_current_player_index(&self, player_index: u32) {
        println!("\nIt is now player {player_index}'s turn");
    }
}
