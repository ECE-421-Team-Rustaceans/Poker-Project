mod card;
mod deck;
use deck::Deck;
mod rules;
mod pot;
mod database;
mod game;


fn main() {
    let mut deck = Deck::new();
    println!("{deck:#?}");
    let card = deck.deal().unwrap();
    println!("{card:#?}");
    println!("card rank: {:?}", card.rank());
    println!("card suit: {:?}", card.suit());
    println!("card is number: {:?}", card.is_number());
    println!("card is face: {:?}", card.is_face());
    println!("card is black {:?}", card.is_black());
    println!("card is red {:?}", card.is_red());
    deck.return_card(card);
    println!("{deck:#?}");
    // let test = FiveCardDraw {};
    // test.play_round();
    // let test = SevenCardDraw {};
    // test.play_round();
    // let test = KansasCityLowball {};
    // test.play_round();
}
