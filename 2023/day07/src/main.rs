use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn incr(self) -> Self {
        match self {
            HandType::FiveOfAKind => HandType::FiveOfAKind,
            HandType::FourOfAKind => HandType::FiveOfAKind,
            HandType::FullHouse => HandType::FourOfAKind,
            HandType::ThreeOfAKind => HandType::FourOfAKind,
            HandType::TwoPair => HandType::FullHouse,
            HandType::OnePair => HandType::ThreeOfAKind,
            HandType::HighCard => HandType::OnePair,
        }
    }
}

struct Card {
  value: char,
  score: u8,  
}

impl Card {
    fn new(value: char) -> Self {
        Card {
            value,
            score: match value {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => value as u8 - '0' as u8,
            },
        }
    }

    fn to_p2(&mut self) {
        if self.value == 'J' {
            self.score = 0;
        }
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn new(line: &str) -> Self {
        let mut iter = line.split_whitespace().into_iter();
        let cards_raw = iter.next().unwrap();
        let mut cards = Vec::new();
        for i in 0..5 {
            cards.push(Card::new(cards_raw.chars().nth(i).unwrap()));
        }

        let bid = iter.next().unwrap().parse::<u32>().unwrap();

        let hand_type = calculate_hand_type(&cards);

        Hand {
            cards,
            bid,
            hand_type,
        }
    }

    fn to_p2(&mut self) {
        self.hand_type = calculate_hand_type_p2(&self.cards);
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }

        for i in 0..5 {
            if self.cards[i].score != other.cards[i].score {
                return false;
            }
        }

        true
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let mut ordering = Ordering::Equal;
        if self.hand_type == other.hand_type {
            for i in 0..5 {
                if self.cards[i].score != other.cards[i].score {
                    ordering = self.cards[i].score.cmp(&other.cards[i].score);
                    break;
                }
            }
        } else {
            ordering = other.hand_type.cmp(&self.hand_type)
        };

        Some(ordering)
    }
}

impl Eq for Hand {}
impl Ord for Hand {
    fn cmp(&self, other: &Hand) ->Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn calculate_hand_type(cards: &Vec<Card>) -> HandType {
    let mut card_counts = [0; 15];
    for card in cards {
        card_counts[card.score as usize] += 1;
    }

    let mut card_counts_sorted = card_counts.clone();
    card_counts_sorted.sort_by(|a, b| b.cmp(a));

    let mut card_counts_sorted = card_counts_sorted.iter();
    let first = card_counts_sorted.next().unwrap();
    let second = card_counts_sorted.next().unwrap();
    let third = card_counts_sorted.next().unwrap();
    let fourth = card_counts_sorted.next().unwrap();
    let fifth = card_counts_sorted.next().unwrap();

    match (first, second, third, fourth, fifth) {
        (5, _, _, _, _) => HandType::FiveOfAKind,
        (4, _, _, _, _) => HandType::FourOfAKind,
        (3, 2, _, _, _) => HandType::FullHouse,
        (3, _, _, _, _) => HandType::ThreeOfAKind,
        (2, 2, _, _, _) => HandType::TwoPair,
        (2, _, _, _, _) => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn calculate_hand_type_p2(cards: &Vec<Card>) -> HandType {
    let mut card_counts = [0; 15];
    let mut count_jokers = 0;
    for card in cards {
        if card.value == 'J' {
            count_jokers += 1;
            continue;
        }
        card_counts[card.score as usize] += 1;
    }

    let mut card_counts_sorted = card_counts.clone();
    card_counts_sorted.sort_by(|a, b| b.cmp(a));

    let mut card_counts_sorted = card_counts_sorted[0..15].iter();
    let first = card_counts_sorted.next().unwrap();
    let second = card_counts_sorted.next().unwrap();
    let third = card_counts_sorted.next().unwrap();
    let fourth = card_counts_sorted.next().unwrap();
    let fifth = card_counts_sorted.next().unwrap();

    let mut hand_type = match (first, second, third, fourth, fifth) {
        (5, _, _, _, _) => HandType::FiveOfAKind,
        (4, _, _, _, _) => HandType::FourOfAKind,
        (3, 2, _, _, _) => HandType::FullHouse,
        (3, _, _, _, _) => HandType::ThreeOfAKind,
        (2, 2, _, _, _) => HandType::TwoPair,
        (2, _, _, _, _) => HandType::OnePair,
        _ => HandType::HighCard,
    };

    for _ in 0..count_jokers {
        hand_type = hand_type.incr();
    }

    return hand_type
}

#[test]
fn test_hand() {
    let hand = Hand::new("32T3K 765");
    assert_eq!(hand.hand_type, HandType::OnePair);

    let hand = Hand::new("T55J5 684");
    assert_eq!(hand.hand_type, HandType::ThreeOfAKind);

    let hand = Hand::new("KK677 28");
    assert_eq!(hand.hand_type, HandType::TwoPair);

    let hand = Hand::new("KTJJT 220");
    assert_eq!(hand.hand_type, HandType::TwoPair);
    
    let hand = Hand::new("QQQJA 483");
    assert_eq!(hand.hand_type, HandType::ThreeOfAKind);
}

fn main() -> std::io::Result<()> {
    let mut input_file = File::open("input.txt")?;
    let mut contents = String::new();
    input_file.read_to_string(&mut contents)?;

    let mut hands = contents.split("\n")
        .into_iter()
        .map(Hand::new)
        .collect::<Vec<Hand>>();
    hands.sort();

    let mut total_winnings_p1 = 0;
    for i in 0..hands.len() {
        total_winnings_p1 += (1 + i as u32) * hands[i].bid;
    }

    println!("Total winnings P1: {}", total_winnings_p1);

    hands.iter_mut().for_each(|hand| {
        hand.to_p2();
        hand.cards.iter_mut().for_each(Card::to_p2);
    });
    hands.sort();

    let mut total_winnings_p2 = 0;
    for i in 0..hands.len() {
        total_winnings_p2 += (1 + i as u32) * hands[i].bid;
    }

    println!("Total winnings P2: {}", total_winnings_p2);

    Ok(())
}

