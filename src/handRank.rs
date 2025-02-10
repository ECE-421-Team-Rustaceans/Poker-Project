use crate::card::{Card, Rank};

#[derive(Debug, PartialEq)]
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
        let highest_card = cards[4].rank().clone();
        let lowest_card = cards[0].rank().clone();

        if is_flush && is_straight {
            if highest_card == Rank::Ace {
                // this is a edge case for a straight flush with an ace
                if lowest_card == Rank::Two {
                    return HandRank::StraightFlush(Rank::Five);
                }
                return HandRank::RoyalFlush;
            }
            return HandRank::StraightFlush(highest_card);
        } else if is_flush {
            return HandRank::Flush(highest_card);
        } else if is_straight {
            return HandRank::Straight(highest_card);
        }
        
        // convert u8 to ranks
        let rank_freqs = Self::count_num_ranks(&cards);

        // if the highest frequency of rank is 4, then it must be four of a kind
        if rank_freqs[0].1 == 4 {
            return HandRank::FourOfAKind(rank_freqs[0].0.clone());
        } else if rank_freqs[0].1 == 3 {
            // if the highest frequency is 3 and second highes tis 2
            if rank_freqs[1].1 == 2 {
                return HandRank::FullHouse(rank_freqs[0].0.clone(), rank_freqs[1].0.clone());
            }
            return HandRank::ThreeOfAKind(rank_freqs[0].0.clone());
        } else if rank_freqs[0].1 == 2 {
            if rank_freqs[1].1 == 2 {
                return HandRank::TwoPair(rank_freqs[0].0.clone(), rank_freqs[1].0.clone());
            }
            return HandRank::OnePair(rank_freqs[0].0.clone());
        }

        return HandRank::HighCard(rank_freqs[0].0.clone());
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
            if cards[i+1].rank().to_u8() != cards[i].rank().to_u8() + 1 {
                is_straight = false;
                break;
            }
        }

        if cards[0].rank() == &Rank::Two && cards[4].rank() == &Rank::Ace {
            is_straight = true;
        }

        is_straight
    }

    pub fn count_num_ranks(cards: &[Card]) -> Vec<(Rank, u8)> {
        let mut counts = [0; 13]; 

        // append correponding index (Two -> 0 index) (to_u8 converts ranks directly, ie Two  -> 2)
        for card in cards {
            let index = card.rank().clone().to_u8() as usize - 2;
            counts[index] += 1;
        }

        let mut rank_freqs: Vec<(u8, u8)> = Vec::new();
        for (i, &count) in counts.iter().enumerate() {
            if count > 0 {
                rank_freqs.push((i as u8 + 2, count));
            }
        }

        // https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust
        // sort from highest to lowest freuqncy, then from highest to lowest rank
        rank_freqs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));

        // convert number rank to enum
        let mut freqs: Vec<(Rank, u8)> = Vec::new();
        for (rank_num, count) in rank_freqs {
            let rank = Rank::to_rank(rank_num);
            freqs.push((rank, count));
        }

        freqs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};
    #[test]
    fn test_high_card() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Jack, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::HighCard(Rank::Jack));
    }
    #[test]
    fn test_one_pair() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Jack, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::OnePair(Rank::Six));
    }
    #[test]
    fn test_two_pair() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Jack, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::TwoPair(Rank::Six, Rank::Two));
    }
    #[test]
    fn test_three_of_a_kind() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Six, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::ThreeOfAKind(Rank::Six));
    }
    #[test]
    fn test_straight() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::Straight(Rank::Six));
    }
    #[test]
    fn test_flush() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::Flush(Rank::Seven));
    }
    #[test]
    fn test_full_house() {
        let hand = vec![
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Six, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::FullHouse(Rank::Six, Rank::Eight));
    }
    #[test]
    fn test_four_of_a_kind() {
        let hand = vec![
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Six, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::FourOfAKind(Rank::Six));
    }
    #[test]
    fn test_straight_flush() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Six, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::StraightFlush(Rank::Six));
    }
    #[test]
    fn test_straight_flush_w_ace() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::StraightFlush(Rank::Five));
    }
    #[test]
    fn test_royal_flush() {
        let hand = vec![
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(hand);
        assert_eq!(hand_rank, HandRank::RoyalFlush);
    }
}