mod card;
use card::Card;
use card::Rank;
use card::Suit;

fn main() {
    let card = Card::new(Rank::Ace, Suit::Clubs);
    println!("{card:#?}");
}
