use crate::card::{Card, Rank};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
/// hand classification rankings, 
/// containing the highest rank in the classification for straight/flush
/// and/or identifies rank in pair/three/four of a kind
/// usage example:
/// ```
/// HandRank::OnePair(Rank::Six)
/// HandRank::TwoPair(Rank::Six, Rank::Two)
/// ```
/// NOTE: in the case of 7 card draw, where there might be multiple rankings, the highest one is returned
pub enum HandRank {
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

impl HandRank {
    fn rank_value(&self) -> u8 {
        match self {
            HandRank::HighCard(_) => 1,
            HandRank::OnePair(_) => 2,
            HandRank::TwoPair(_, _) => 3,
            HandRank::ThreeOfAKind(_) => 4,
            HandRank::Straight(_) => 5,
            HandRank::Flush(_) => 6,
            HandRank::FullHouse(_, _) => 7,
            HandRank::FourOfAKind(_) => 8,
            HandRank::StraightFlush(_) => 9,
            HandRank::RoyalFlush => 10,
        }
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandRank {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank_value().cmp(&other.rank_value()).then_with(|| match (self, other) {
            (HandRank::HighCard(a), HandRank::HighCard(b)) => a.cmp(b),
            (HandRank::OnePair(a), HandRank::OnePair(b)) => a.cmp(b),
            (HandRank::TwoPair(a1, a2), HandRank::TwoPair(b1, b2)) => (a1, a2).cmp(&(b1, b2)),
            (HandRank::ThreeOfAKind(a), HandRank::ThreeOfAKind(b)) => a.cmp(b),
            (HandRank::Straight(a), HandRank::Straight(b)) => a.cmp(b),
            (HandRank::Flush(a), HandRank::Flush(b)) => a.cmp(b),
            (HandRank::FullHouse(a1, a2), HandRank::FullHouse(b1, b2)) => (a1, a2).cmp(&(b1, b2)),
            (HandRank::FourOfAKind(a), HandRank::FourOfAKind(b)) => a.cmp(b),
            (HandRank::StraightFlush(a), HandRank::StraightFlush(b)) => a.cmp(b),
            (HandRank::RoyalFlush, HandRank::RoyalFlush) => Ordering::Equal,
            _ => Ordering::Equal,
        })
    }
}

#[derive(PartialEq, Eq)]
/// hand of cards struct containing vec of cards
pub struct Hand {
    cards: Vec<Card>
}

impl Hand {
    /// create a new hand from a vector of cards
    pub fn new(cards: Vec<Card>) -> Hand {
        Hand{cards}
    }
    /// return the poker hand classified
    pub fn rank_hand(cards: &[Card]) -> HandRank {
        let mut sorted_cards = cards.to_vec();
        sorted_cards.sort();

        let is_flush = Self::is_flush(&sorted_cards);
        let is_straight = Self::is_straight(&sorted_cards);
        let highest_card = sorted_cards.last().unwrap().rank().clone();
        let lowest_card = sorted_cards.first().unwrap().rank().clone();

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
        let rank_freqs = Self::count_num_ranks(&sorted_cards);

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

    /// true if the poker hand is a flush 
    /// (if all suits in the hand are the same)
    pub fn is_flush(cards: &[Card]) -> bool {
        let suit = cards[0].suit();
        for i in 0..cards.len() - 1 {
            if cards[i].suit() != suit {
                return false;
            }
        }
        true
    }

    /// true if the poker hand is a stright
    /// (if the ranks of cards are in a row)
    /// NOTE: the special case of an ace-low straight is checked
    /// with the 4 lowest cards and the last card (this will need to be updated for 7 card draw)
    pub fn is_straight(cards: &[Card]) -> bool {
        let mut is_straight = true;

        // this logic need to be changed for 7 card draw in case of duplicate cards
        // check if ace-low straight, the first few cards must be in order... 
        // this works for 5 card draws but not for 7 card draw
        if cards[0].rank() == &Rank::Two
            && cards[1].rank() == &Rank::Three
            && cards[2].rank() == &Rank::Four
            && cards[3].rank() == &Rank::Five
            && cards[cards.len() - 1].rank() == &Rank::Ace {
                return is_straight;
            }


        for i in 0..cards.len() - 1 {
            // if the next card rank isn't equal to the current card rank + 1
            if cards[i+1].rank().to_u8() != cards[i].rank().to_u8() + 1 {
                is_straight = false;
                break;
            }
        }

        is_straight
    }

    /// returns the sorted (descending) card ranks and their corresponding frequencies in a hand. 
    /// sorted first based on highest frequency, then rank in each respective frequency. 
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

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rank = Hand::rank_hand(&self.cards);
        let other_rank = Hand::rank_hand(&other.cards);
        self_rank.cmp(&other_rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Card, Rank, Suit};
    #[test]
    fn test_new() {
        let cards = vec![
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
        ];
        let hand = Hand::new(cards.clone());

        assert_eq!(hand.cards.len(), 5);
        assert_eq!(hand.cards, cards);
    }

    #[test]
    fn test_high_card() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Jack, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
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
        let hand_rank = Hand::rank_hand(&hand);
        assert_eq!(hand_rank, HandRank::RoyalFlush);
    }

    #[test]
    fn test_ordering() {
        let cards1 = vec![
            Card::new(Rank::King, Suit::Hearts),
            Card::new(Rank::Jack, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Queen, Suit::Hearts),
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Jack, Suit::Hearts),
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 > hand2);
    }

    #[test]
    fn test_ordering_one_card() {
        let cards1 = vec![
            Card::new(Rank::King, Suit::Spades)
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Three, Suit::Hearts)
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 > hand2);
    }

    #[test]
    fn test_ordering_two_cards() {
        let cards1 = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades)
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds)
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 < hand2);
    }

    #[test]
    fn test_ordering_three_cards() {
        let cards1 = vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs)
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Clubs)
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 < hand2);
    }
}
