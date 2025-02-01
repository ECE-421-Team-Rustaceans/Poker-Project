mod rules;
use rules::{kansas_city_lowball::KansasCityLowball, seven_card_draw::SevenCardDraw, five_card_draw::FiveCardDraw, Rules};

fn main() {
    println!("Hello, world!");
    let test = FiveCardDraw {};
    test.play_game();
    let test = SevenCardDraw {};
    test.play_game();
    let test = KansasCityLowball {};
    test.play_game();
}
