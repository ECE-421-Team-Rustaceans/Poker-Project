mod rules;
use rules::{seven_card_draw::SevenCardDraw, standard_five_card_draw::StandardFiveCardDraw, Rules};

fn main() {
    println!("Hello, world!");
    let test = StandardFiveCardDraw {};
    test.play_game();
    let test = SevenCardDraw {};
    test.play_game();
}
