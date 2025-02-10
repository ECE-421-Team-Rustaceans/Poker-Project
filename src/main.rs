use std::io;

mod card;
mod deck;
use deck::Deck;
mod rules;
mod handRank;
mod pot;
mod database;
mod game;
mod player;


fn main() {
    // let mut deck = Deck::new();
    // println!("{deck:#?}");
    // let card = deck.deal().unwrap();
    // println!("{card:#?}");
    // println!("card rank: {:?}", card.rank());
    // println!("card suit: {:?}", card.suit());
    // println!("card is number: {:?}", card.is_number());
    // println!("card is face: {:?}", card.is_face());
    // println!("card is black {:?}", card.is_black());
    // println!("card is red {:?}", card.is_red());
    // deck.return_card(card);
    // println!("{deck:#?}");
    // let test = FiveCardDraw {};
    // test.play_game();
    // let test = SevenCardDraw {};
    // test.play_game();
    // let test = KansasCityLowball {};
    // test.play_game();

    println!("poker time");

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

    println!("\nselect a game:\n1 - five card draw\n2 - seven card draw\n3 - kansas city lowball");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let game: u32 = input
        .trim()
        .parse()
        .expect("not an integer");

    // not really sure how to get game play from game variant from here, i assume it uses a match

}
