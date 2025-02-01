mod rules;
use rules::{Rules, standard_five_card_draw::StandardFiveCardDraw};

fn main() {
    println!("Hello, world!");
    let test = StandardFiveCardDraw {};
    test.play_game();
}
