use super::*;
use crate::game_type::GameType;

pub struct CliInput;

impl Input for CliInput {
    fn new() -> Self {
        return Self;
    }

    fn request_username(&mut self) -> String {
        let name = loop {
            println!("\nEnter your player name:");

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

        return name;
    }

    fn input_variation(&mut self) -> GameType {
        loop {
            println!("\nSelect a game:\n1 - Five Card Draw\n2 - Seven Card Stud\n3 - Texas Hold'em");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line");

            match input.trim().parse::<usize>(){
                Ok(1) => return GameType::FiveCardDraw,
                Ok(2) => return GameType::SevenCardStud,
                Ok(3) => return GameType::TexasHoldem,
                _ => println!("invalid! enter 1, 2, or 3."),
            }
        }
    }

    fn input_action_options(&mut self, possible_actions: Vec<ActionOption>, player: &Player) -> ActionOption {
        println!("\nPlayer: {}", player.name());
        loop {
            println!("Select an action:");
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

    fn request_raise_amount(&mut self, limit: u32, player: &Player) -> u32 {
        println!("\nPlayer: {}", player.name());
        loop {
            println!("Enter amount to raise by, limit is {limit}: ");
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

    fn request_replace_cards<'a>(&mut self, player: &'a Player) -> Vec<&'a Card> {
        let cards = player.peek_at_cards();
        let mut selected_cards = Vec::new();
        for card in cards.iter() {
            selected_cards.push((false, *card));
        }
        println!("\nPlayer: {}", player.name());
        loop {
            println!("Here are your {} cards:", selected_cards.len());
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

    fn display_current_player(&self, player: &Player) {
        println!("\nIt is now {}'s turn", player.name());
    }

    fn display_player_cards_to_player(&self, player: &Player) {
        let cards = player.peek_at_cards();
        println!("\nPlayer: {},", player.name());
        println!("Here are your {} cards:", cards.len());
        for card in cards {
            println!("-> {card} <-");
        }
    }

    fn display_community_cards_to_player(&self, community_cards: Vec<&Card>, _player: &Player) {
        println!("\nHere are the community cards:");
        for card in community_cards {
            println!("-> {card} <-");
        }
    }

    fn display_other_player_up_cards_to_player(&self, other_players: Vec<&Player>, player: &Player) {
        let other_players: Vec<&Player> = other_players.into_iter().filter(|other_player| other_player.name() != player.name()).collect();
        println!("\nPlayer: {},", player.name());
        println!("Here are the other {} players' up cards:", other_players.len());
        for other_player in other_players {
            let up_cards: Vec<&Card> = other_player.peek_at_cards().into_iter().filter(|card| card.is_face_up()).collect();
            println!("\tPlayer {}'s up cards:", other_player.name());
            for up_card in up_cards {
                println!("\t-> {up_card} <-");
            }
        }
    }

    fn announce_winner(&self, winner: Vec<&Player>, _all_players: Vec<&Player>) {
        assert!(winner.len() > 0);
        if winner.len() == 1 {
            println!("\nThe winner is: {}!", winner[0].name());
        }
        else {
            println!("\nThe winners are:");
            for winner_player in winner {
                println!("- {}!", winner_player.name());
            }
        }
    }

    fn display_pot(&self, pot_amount: u32, _all_players: Vec<&Player>) {
        println!("\nThe pot currently holds {pot_amount}");
    }
}
