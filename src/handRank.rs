use crate::card::{Card, Rank};

#[derive(Debug)]
pub enum HandRank {
    // save the highest ish number of combination
    HighCard(Rank),
    OnePair(Rank),
    TwoPair(Rank, Rank),
    ThreeOfAKind(Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    pub fn rank_hand(mut cards: Vec<Card>) -> HandRank {
        cards.sort();

        let is_flush = Self::is_flush(&cards);
        let is_straight = Self::is_straight(&cards);
        let count_num_ranks = Self::count_num_ranks(&cards);
        let highest_card = cards[4].rank();

        if is_flush && is_straight {
            
            if highest_card == &Rank::Ace {
                return HandRank::RoyalFlush;
            }

            return HandRank::StraightFlush(highest_card);
        } 

        // else if just straight or just flush, return
        
    }

    pub fn is_flush(cards: &[Card]) -> bool {
        let suit = cards[0].suit();
        for i in 0..cards.len() - 1 {
            if cards[i].suit() != suit {
                return false;
            }
        }
        true
    }

    pub fn is_straight (cards: &[Card]) -> bool {
        let mut is_straight = true;
        for i in 0..cards.len() - 1 {
            // if the next card rank isn't equal to the current card rank + 1
            if cards[i+1].rank().as_u8() != cards[i].rank().as_u8() + 1 {
                is_straight = false;
                break;
            }
        }

        if cards[0].rank() == &Rank::Two && cards[4].rank() == &Rank::Ace {
            is_straight = true;
        }

        is_straight
    }

    pub fn count_num_ranks(cards: &[Card]) -> [u8; 13] {
        let mut counts = [0; 13]; 

        for card in cards {
            let index = card.rank().clone().as_u8() as usize - 2;
            counts[index] += 1;
        }

        counts
    }
}