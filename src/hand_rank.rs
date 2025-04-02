use crate::card::{Card, Rank, Suit};
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
    HighCard(Rank, Vec<Rank>), // highest card plus kickers
    OnePair(Rank, Vec<Rank>), // pair plus kickers
    TwoPair(Rank, Rank, Rank), // two pair plus kicker
    ThreeOfAKind(Rank, Vec<Rank>), // three of a kind plus kickers
    Straight(Rank),
    Flush(Rank, Vec<Rank>), // flush plus ordered cards
    FullHouse(Rank, Rank), 
    FourOfAKind(Rank, Rank), // four of a kind rank plus second highest card
    StraightFlush(Rank),
    RoyalFlush,
}

impl HandRank {
    fn rank_value(&self) -> u8 {
        match self {
            HandRank::HighCard(_, _) => 1,
            HandRank::OnePair(_, _) => 2,
            HandRank::TwoPair(_, _, _) => 3,
            HandRank::ThreeOfAKind(_, _) => 4,
            HandRank::Straight(_) => 5,
            HandRank::Flush(_, _) => 6,
            HandRank::FullHouse(_, _) => 7,
            HandRank::FourOfAKind(_, _) => 8,
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
            (HandRank::HighCard(a, kickers1), HandRank::HighCard(b, kickers2)) => a.cmp(b).then_with(|| kickers1.cmp(kickers2)),
            (HandRank::OnePair(a, kickers1), HandRank::OnePair(b, kickers2)) => a.cmp(b).then_with(|| kickers1.cmp(kickers2)),
            (HandRank::TwoPair(a1, a2, kickers1), HandRank::TwoPair(b1, b2, kickers2)) => (a1, a2).cmp(&(b1, b2)).then_with(|| kickers1.cmp(kickers2)),
            (HandRank::ThreeOfAKind(a, kickers1), HandRank::ThreeOfAKind(b, kickers2)) => a.cmp(b).then_with(|| kickers1.cmp(kickers2)),
            (HandRank::Straight(a), HandRank::Straight(b)) => a.cmp(b),
            (HandRank::Flush(a, kickers1), HandRank::Flush(b, kickers2)) => a.cmp(b).then_with(|| kickers1.cmp(kickers2)),
            (HandRank::FullHouse(a1, a2), HandRank::FullHouse(b1, b2)) => (a1, a2).cmp(&(b1, b2)),
            (HandRank::FourOfAKind(a, kickers1), HandRank::FourOfAKind(b, kickers2)) => a.cmp(b).then_with(|| kickers1.cmp(kickers2)),
            (HandRank::StraightFlush(a), HandRank::StraightFlush(b)) => a.cmp(b),
            (HandRank::RoyalFlush, HandRank::RoyalFlush) => Ordering::Equal,
            _ => Ordering::Equal,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
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
        let mut sorted_ranks: Vec<Rank> = sorted_cards.iter().map(|card| card.rank().clone()).collect();
        sorted_ranks.sort();
        sorted_ranks.reverse();

        sorted_cards.sort();

        let is_flush = Self::is_flush(&sorted_cards);
        let is_straight = Self::is_straight(&sorted_cards);
        let is_straight_flush = Self::is_straight_flush(&sorted_cards);
        let highest_card = sorted_cards.last().unwrap().rank().clone(); // sorted_ranks.first().unwrap().clone().clone();
        let lowest_card = sorted_cards.first().unwrap().rank().clone(); // sorted_ranks.last().unwrap().clone().clone();

        if is_straight_flush {
            if highest_card == Rank::Ace {
                // this is a edge case for a straight flush with an ace
                if lowest_card == Rank::Two {
                    for card_index in 0..sorted_cards.len()-1 {
                        if sorted_cards[card_index].rank().to_u8() != sorted_cards[card_index+1].rank().to_u8() - 1 {
                            return HandRank::StraightFlush(sorted_cards[card_index].rank().clone());
                        }
                    }
                    return HandRank::StraightFlush(highest_card);
                }
                return HandRank::RoyalFlush;
            }
            return HandRank::StraightFlush(highest_card);
        } else if is_flush {
            let high_card = sorted_ranks.remove(0).clone();
            let kickers = sorted_ranks.into_iter().take(4).collect();
            return HandRank::Flush(high_card, kickers);
        } else if is_straight {
            // check for ace low straight
            if lowest_card == Rank::Two 
                && highest_card == Rank::Ace
                // need to also check if there isn't a higher straight
                && cards.iter().any(|c| c.rank() != &Rank::Six){
                return HandRank::Straight(Rank::Five);
            } else {
                return HandRank::Straight(highest_card);
            }
        }
        
        // convert u8 to ranks
        let rank_freqs = Self::count_num_ranks(&sorted_cards);

        // let has_three_of_a_kind = rank_freqs.iter().any(|&(_, count)| count == 3);
        // let three_count = rank_freqs.iter().filter(|&&(_, count)| count == 3).count();
        
        match (
            rank_freqs.iter().filter(|&&(_, count)| count == 4).count(),
            rank_freqs.iter().filter(|&&(_, count)| count == 3).count(),
            rank_freqs.iter().filter(|&&(_, count)| count == 2).count(),
        ) {
            // if there is such a frequency count of 4, then it must be four of a kind
            // it is not possible to have 2 four of a kinds
            (1, _, _) => {
                let rank = rank_freqs.iter().find(|&&(_, count)| count == 4).unwrap().0.clone();
                sorted_ranks.retain(|r| *r != rank);
                let kicker = sorted_ranks[0].clone();
                return HandRank::FourOfAKind(rank, kicker);
            }
            // if there is some combination of 3 of a kind and pair, it must be a full house
            // in 7 card stud, there might be two sets of 3 or 2
            (0, 1.., 1..) => {
                let mut pair = rank_freqs.iter().find(|&&(_, count)| count == 2).unwrap().0.clone();
                let mut three = rank_freqs.iter().find(|&&(_, count)| count == 3).unwrap().0.clone();
                if rank_freqs.iter().filter(|&&(_, count)| count == 3).count() == 2 {
                    pair = rank_freqs.iter()
                        .filter(|&&(_, count)| count == 2)
                        .map(|(rank, _)| rank)
                        .max()
                        .unwrap()                              
                        .clone();
                } else if rank_freqs.iter().filter(|&&(_, count)| count == 2).count() == 2 {
                    three = rank_freqs.iter()
                        .filter(|&&(_, count)| count == 3)
                        .map(|(rank, _)| rank)
                        .max()  
                        .unwrap()                            
                        .clone();
                } 
                return HandRank::FullHouse(three, pair);
            }
            // three of a kind
            // may be more than 1 is 7 card variation
            (0, 1.., 0) => {
                let mut rank = rank_freqs.iter().find(|&&(_, count)| count == 3).unwrap().0.clone();
                if rank_freqs.iter().filter(|&&(_, count)| count == 3).count() == 2 {
                    rank = rank_freqs.iter()
                        .filter(|&&(_, count)| count == 3)
                        .map(|(rank, _)| rank)
                        .max()
                        .unwrap()                              
                        .clone();
                }
                sorted_ranks.retain(|r| *r != rank);
                let kickers = sorted_ranks.into_iter().take(2).collect();
                return HandRank::ThreeOfAKind(rank, kickers);
            }
            // two pair
            // there might be 3 pairs in 7 card variation
            (0, 0, 1..) => {
                let mut pairs: Vec<Rank> = rank_freqs.iter()
                    .filter(|&&(_, count)| count == 2)
                    .map(|(rank, _)| rank)
                    .cloned()
                    .collect();

                pairs.sort_by(|a, b| b.cmp(a));

                let kickers: Vec<Rank> = rank_freqs.iter()
                    .map(|(rank, _)| rank)
                    .filter(|rank| !pairs.contains(rank))
                    .take(3)
                    .cloned()
                    .collect();
                
                if pairs.len() >=2 {
                    let kicker = &kickers[0];
                    return HandRank::TwoPair(pairs[0].clone(), pairs[1].clone(), kicker.clone());
                }

                return HandRank::OnePair(pairs[0].clone(), kickers);
            }
            _ => {
                let high_card = sorted_ranks.remove(0);
                let kickers = sorted_ranks.into_iter().take(5).collect();
                return HandRank::HighCard(high_card, kickers);
            }
        };
    }

    /// true if the poker hand is a flush
    pub fn is_flush(cards: &[Card]) -> bool {
        let suits: Vec<Suit> = cards.iter()
            .map(|card| card.suit().clone())
            .collect();
        for suit in vec![Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds] {
            if suits.iter().filter(|card_suit| **card_suit == suit).count() >= 5 {
                return true;
            }
        }

        false
    }

    /// true if the poker hand is a stright
    /// NOTE: the special case of an ace-low straight is checked
    pub fn is_straight(cards: &[Card]) -> bool {
        // seperate to just the ranks
        let mut ranks: Vec<Rank> = cards.iter()
            .map(|card| card.rank().clone())
            .collect();
        // sort ascending order
        ranks.sort_by(|a, b| a.cmp(b));
        ranks.dedup(); // remove the duplicate ranks

        // it is definitely not a straight if there is less than 5
        if ranks.len() < 5 {
            return false;
        }

        // check if ace-low straight (ie ace 2 3 4 5)        
        if ranks.iter().any(|c| c == &Rank::Ace)
            && ranks.iter().any(|c| c == &Rank::Two)
            && ranks.iter().any(|c| c == &Rank::Three)
            && ranks.iter().any(|c| c == &Rank::Four)
            && ranks.iter().any(|c| c == &Rank::Five) {
            return true;
        }

        let mut straight_counter = 1;
        for i in 0..ranks.len() - 1 {
            if ranks[i+1].to_u8() == ranks[i].to_u8() + 1 {
                straight_counter += 1;
            }
            else {
                straight_counter = 1;
            }
            if straight_counter == 5 {
                return true;
            }
        }

        return false;
    }

    /// necessary because hands may or may not have 5 cards
    /// true if the poker hand is a straight flush
    pub fn is_straight_flush(cards: &[Card]) -> bool {
        // it is definitely not a straight if there is less than 5
        if cards.len() < 5 {
            return false;
        }
        let mut cards: Vec<Card> = cards.to_vec();
        // sort ascending order
        cards.sort_by(|a, b| a.rank().cmp(b.rank()));

        let mut suit_cards: Vec<Vec<Card>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for card in cards {
            match card.suit() {
                Suit::Clubs => suit_cards[0].push(card),
                Suit::Spades => suit_cards[1].push(card),
                Suit::Hearts => suit_cards[2].push(card),
                Suit::Diamonds => suit_cards[3].push(card),
            }
        }

        // check if ace-low straight (ie ace 2 3 4 5)
        for cards_with_matching_suit in suit_cards.iter() {
            if cards_with_matching_suit.iter().any(|c| c.rank() == &Rank::Ace)
                && cards_with_matching_suit.iter().any(|c| c.rank() == &Rank::Two)
                && cards_with_matching_suit.iter().any(|c| c.rank() == &Rank::Three)
                && cards_with_matching_suit.iter().any(|c| c.rank() == &Rank::Four)
                && cards_with_matching_suit.iter().any(|c| c.rank() == &Rank::Five) {

                return true;
            }
        }

        for cards_with_matching_suit in suit_cards.iter() {
            if cards_with_matching_suit.len() < 5 {
                continue;
            }
            let mut straight_counter = 1;
            for i in 0..cards_with_matching_suit.len() - 1 {
                if cards_with_matching_suit[i+1].rank().to_u8() == cards_with_matching_suit[i].rank().to_u8() + 1 {
                    straight_counter += 1;
                }
                else {
                    straight_counter = 1;
                }
                if straight_counter == 5 {
                    return true;
                }
            }
        }

        return false;
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
        assert_eq!(hand_rank, HandRank::HighCard(Rank::Jack, vec![Rank::Eight, Rank::Six, Rank::Four, Rank::Two]));
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
        assert_eq!(hand_rank, HandRank::OnePair(Rank::Six, vec![Rank::Jack, Rank::Eight, Rank::Two]));
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
        assert_eq!(hand_rank, HandRank::TwoPair(Rank::Six, Rank::Two, Rank::Jack));
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
        assert_eq!(hand_rank, HandRank::ThreeOfAKind(Rank::Six, vec![Rank::Eight, Rank::Two]));
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
    fn test_straight_w_ace() {
        let hand = vec![
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Four, Suit::Hearts),
        ];
        let hand_rank = Hand::rank_hand(&hand);
        assert_eq!(hand_rank, HandRank::Straight(Rank::Five));
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
        assert_eq!(hand_rank, HandRank::Flush(Rank::Seven, vec![Rank::Six, Rank::Five, Rank::Three, Rank::Two]));
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
        assert_eq!(hand_rank, HandRank::FourOfAKind(Rank::Six, Rank::Eight));
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
    fn test_ordering_three_cards_high_card_to_one_pair() {
        let high_card = Hand::new(vec![
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs)
        ]);
        let one_pair = Hand::new(vec![
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Clubs)
        ]);
        assert!(high_card < one_pair);
    }

    #[test]
    fn test_ordering_three_cards_one_pair_to_three_of_a_kind() {
        let one_pair = Hand::new(vec![
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Clubs)
        ]);
        let three_of_a_kind = Hand::new(vec![
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs)
        ]);
        assert!(one_pair < three_of_a_kind);
    }

    #[test]
    fn test_ordering_three_cards_both_one_pair() {
        let one_pair1 = Hand::new(vec![
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Clubs)
        ]);
        let one_pair2 = Hand::new(vec![
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs)
        ]);
        assert!(one_pair1 > one_pair2);
    }

    #[test]
    fn test_ordering_seven_cards() {
        let cards1 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Diamonds)
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades)
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 < hand2);
    }

    #[test]
    fn test_ordering_seven_cards_2() {
        let cards1 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Diamonds)
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Hearts)
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 < hand2);
    }

    #[test]
    fn test_ordering_seven_cards_3() {
        let cards1 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Diamonds)
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Hearts)
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 > hand2);
    }

    #[test]
    fn test_ordering_seven_cards_4() {
        let cards1 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Ten, Suit::Diamonds)
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades)
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 > hand2);
    }

    #[test]
    fn test_ordering_seven_cards_5() {
        let cards1 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Hearts),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Diamonds)
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Seven, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Spades)
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 < hand2);
    }

    #[test]
    fn test_ordering_seven_cards_6() {
        let cards1 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ace, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Ace, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ];
        let hand1 = Hand::new(cards1);
        let cards2 = vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Four, Suit::Hearts),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Six, Suit::Hearts)
        ];
        let hand2 = Hand::new(cards2);
        assert!(hand1 < hand2);
    }

    #[test]
    fn test_ordering_seven_cards_high_card_to_one_pair() {
        let high_card = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        let one_pair = Hand::new(vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs)
        ]);
        assert!(high_card < one_pair);
    }

    #[test]
    fn test_ordering_seven_cards_one_pair_to_two_pair() {
        let one_pair = Hand::new(vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs)
        ]);
        let two_pair = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        assert!(one_pair < two_pair);
    }

    #[test]
    fn test_ordering_seven_cards_two_pair_to_three_of_a_kind() {
        let two_pair = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        let three_of_a_kind = Hand::new(vec![
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Queen, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Two, Suit::Clubs)
        ]);
        assert!(two_pair < three_of_a_kind);
    }

    #[test]
    fn test_ordering_seven_cards_three_of_a_kind_to_straight() {
        let three_of_a_kind = Hand::new(vec![
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Clubs)
        ]);
        let straight = Hand::new(vec![
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        assert!(three_of_a_kind < straight);
    }

    #[test]
    fn test_ordering_seven_cards_straight_to_flush() {
        let straight = Hand::new(vec![
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Clubs),
            Card::new(Rank::Five, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        let flush = Hand::new(vec![
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Clubs)
        ]);
        assert!(straight < flush);
    }

    #[test]
    fn test_ordering_seven_cards_flush_to_full_house() {
        let flush = Hand::new(vec![
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Clubs)
        ]);
        let full_house = Hand::new(vec![
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Clubs)
        ]);
        assert!(flush < full_house);
    }

    #[test]
    fn test_ordering_seven_cards_full_house_to_four_of_a_kind() {
        let full_house = Hand::new(vec![
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Spades),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Nine, Suit::Clubs)
        ]);
        let four_of_a_kind = Hand::new(vec![
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Clubs)
        ]);
        assert!(full_house < four_of_a_kind);
    }

    #[test]
    fn test_ordering_seven_cards_four_of_a_kind_to_straight_flush() {
        let four_of_a_kind = Hand::new(vec![
            Card::new(Rank::Nine, Suit::Hearts),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Spades),
            Card::new(Rank::Nine, Suit::Diamonds),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Hearts),
            Card::new(Rank::Nine, Suit::Clubs)
        ]);
        let straight_flush = Hand::new(vec![
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        assert!(four_of_a_kind < straight_flush);
    }

    #[test]
    fn test_ordering_seven_cards_straight_flush_to_royal_flush() {
        let straight_flush = Hand::new(vec![
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Four, Suit::Spades),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Four, Suit::Clubs),
            Card::new(Rank::Six, Suit::Spades),
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        let royal_flush = Hand::new(vec![
            Card::new(Rank::Ace, Suit::Spades),
            Card::new(Rank::Ten, Suit::Spades),
            Card::new(Rank::Five, Suit::Clubs),
            Card::new(Rank::Queen, Suit::Spades),
            Card::new(Rank::King, Suit::Spades),
            Card::new(Rank::Ten, Suit::Diamonds),
            Card::new(Rank::Jack, Suit::Spades)
        ]);
        assert!(straight_flush < royal_flush);
    }

    #[test]
    fn test_ordering_seven_cards_two_pair_to_two_pair_with_kicker() {
        let two_pair_high_kicker = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        let two_pair_low_kicker = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds)
        ]);
        assert!(two_pair_low_kicker < two_pair_high_kicker);
        assert!(two_pair_low_kicker != two_pair_high_kicker);
    }

    #[test]
    fn test_ordering_seven_cards_one_pair_to_one_pair_with_kicker() {
        let one_pair_high_kicker = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Ace, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        let one_pair_low_kicker = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::King, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds)
        ]);
        assert!(one_pair_low_kicker < one_pair_high_kicker);
        assert!(one_pair_low_kicker != one_pair_high_kicker);
    }

    #[test]
    fn test_ordering_seven_cards_three_of_a_kind_to_three_of_a_kind_with_kicker() {
        let two_pair_high_kicker = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        let two_pair_low_kicker = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Three, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Four, Suit::Diamonds)
        ]);
        assert!(two_pair_low_kicker < two_pair_high_kicker);
        assert!(two_pair_low_kicker != two_pair_high_kicker);
    }

    #[test]
    fn test_ordering_seven_cards_two_pair_to_two_pair_equal() {
        let two_pair1 = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Ten, Suit::Diamonds)
        ]);
        let two_pair2 = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Ten, Suit::Clubs),
            Card::new(Rank::Seven, Suit::Hearts),
            Card::new(Rank::Eight, Suit::Diamonds)
        ]);
        let pair1 = Hand::rank_hand(&two_pair1.cards);
        let pair2 = Hand::rank_hand(&two_pair2.cards);
        assert!(!(pair1 < pair2));
        assert!(!(pair2 < pair1));
        assert!(pair1 == pair2);
    }

    #[test]
    fn test_ordering_five_cards_one_pair_equal() {
        let two_pair1 = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Clubs),
            Card::new(Rank::Two, Suit::Clubs),
            Card::new(Rank::Eight, Suit::Clubs)
        ]);
        let two_pair2 = Hand::new(vec![
            Card::new(Rank::Three, Suit::Diamonds),
            Card::new(Rank::Two, Suit::Spades),
            Card::new(Rank::Three, Suit::Hearts),
            Card::new(Rank::Two, Suit::Diamonds),
            Card::new(Rank::Eight, Suit::Hearts)
        ]);
        let pair1 = Hand::rank_hand(&two_pair1.cards);
        let pair2 = Hand::rank_hand(&two_pair2.cards);
        assert!(!(pair1 < pair2));
        assert!(!(pair2 < pair1));
        assert!(pair1 == pair2);
    }

    #[test]
    fn test_ordering_two_cards_equal() {
        let high_card1 = Hand::new(vec![
            Card::new(Rank::Five, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Hearts)
        ]);
        let high_card2 = Hand::new(vec![
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades)
        ]);
        println!("high card 1 - {:?}", high_card1);
        println!("high card 2 - {:?}", high_card2);
        let high1 = Hand::rank_hand(&high_card1.cards);
        let high2 = Hand::rank_hand(&high_card2.cards);
        assert!(!(high1 < high2));
        assert!(!(high2 < high1));
        assert!(high1 == high2);
    }

    #[test]
    fn test_ordering_two_cards_with_kicker() {
        let high_card1 = Hand::new(vec![
            Card::new(Rank::Five, Suit::Spades),
            Card::new(Rank::Nine, Suit::Spades)
        ]);
        let high_card2 = Hand::new(vec![
            Card::new(Rank::Six, Suit::Diamonds),
            Card::new(Rank::Nine, Suit::Hearts)
        ]);
        assert!(high_card1 < high_card2);
        assert!(high_card1 != high_card2);
    }
}
