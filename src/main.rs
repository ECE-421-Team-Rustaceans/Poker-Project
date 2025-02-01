mod rules;
use rules::{kansas_city_lowball::KansasCityLowball, seven_card_draw::SevenCardDraw, standard_five_card_draw::StandardFiveCardDraw, Rules};

fn main() {
    println!("Hello, world!");
    let test = StandardFiveCardDraw {};
    test.play_game();
    let test = SevenCardDraw {};
    test.play_game();
    let test = KansasCityLowball {};
    test.play_game();
}
