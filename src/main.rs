mod card;
use card::Card;
use card::Rank;
use card::Suit;

fn main() {
    let card = Card::new(Rank::Ace, Suit::Clubs);
    println!("{card:#?}");
    println!("card rank: {:?}", card.rank());
    println!("card suit: {:?}", card.suit());
    println!("card is number: {:?}", card.is_number());
    println!("card is face: {:?}", card.is_face());
    println!("card is black {:?}", card.is_black());
    println!("card is red {:?}", card.is_red());
}
