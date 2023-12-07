use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: [u8; 5],
    pub hand_type: HandType,
    pub bid: u32,
}

impl Hand {
    pub fn new(input: &str) -> Hand {
        let (hand, bid) = input.split_at(6);
        let hand = hand.trim();
        let bid = bid.trim();
        let mut cards = [0; 5];
        let mut i = 0;
        let input = hand.bytes();
        for card in input {
            cards[i] = card_to_num(card as char);
            i += 1;
        }
        let hand_type = get_hand_type(&cards);
        Hand {
            cards,
            hand_type,
            bid: bid.parse::<u32>().unwrap(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }
        if self.cards != other.cards {
            return false;
        }
        return true;
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self.hand_type != other.hand_type {
            return Some(self.hand_type.cmp(&other.hand_type));
        }
        for i in 0..5 {
            if self.cards[i] > other.cards[i] {
                return Some(Ordering::Greater);
            }
            if self.cards[i] < other.cards[i] {
                return Some(Ordering::Less);
            }
        }
        return None;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }
        for i in 0..5 {
            if self.cards[i] > other.cards[i] {
                return Ordering::Greater;
            }
            if self.cards[i] < other.cards[i] {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
}

fn get_hand_type(cards: &[u8; 5]) -> HandType {
    let mut counts: [u8; 15] = [0; 15];
    for card in cards {
        counts[*card as usize] += 1;
    }
    let mut counts = counts.into_iter().filter(|&c| c > 0).collect::<Vec<u8>>();
    counts.sort();
    if counts.len() == 1 {
        return HandType::FiveOfAKind;
    }

    if counts.len() == 2 {
        if counts[0] == 1 {
            return HandType::FourOfAKind;
        }
        return HandType::FullHouse;
    }

    if counts.len() == 3 {
        if counts[2] == 3 {
            return HandType::ThreeOfAKind;
        }
        return HandType::TwoPair;
    }

    if counts.len() == 4 {
        return HandType::OnePair;
    }

    return HandType::HighCard;
}

fn card_to_num(card: char) -> u8 {
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ordering() {
        let hand = Hand {
            cards: [13, 13, 6, 7, 7],
            hand_type: HandType::TwoPair,
            bid: 28,
        };

        let hand2 = Hand {
            cards: [13, 10, 11, 11, 10],
            hand_type: HandType::TwoPair,
            bid: 28,
        };

        assert!(hand > hand2);
    }

    #[test]
    fn test_enum_ordering() {
        let hand_type = HandType::TwoPair;
        let hand_type2 = HandType::OnePair;

        assert!(hand_type > hand_type2);
    }
}
