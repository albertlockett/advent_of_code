use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::ops::AddAssign;

struct Card {
    win_nums: HashSet<u8>,
    card_nums: HashSet<u8>,
}

impl Card {
    fn new(line: &str) -> Self {
        let mut prefix_iter = line.split(":").into_iter();
        prefix_iter.next();

        let number_section = prefix_iter.next().unwrap();
        let mut num_sec_iter = number_section.split("|").into_iter();

        Card {
            win_nums: to_num_set(num_sec_iter.next().unwrap()),
            card_nums: to_num_set(num_sec_iter.next().unwrap())
        }
    }

    fn score(&self) -> u64 {
        let intersection_size = self.num_intersect();
        if intersection_size == 0 {
            return 0;
        }

        let base: u64 = 2;
        base.pow((intersection_size - 1) as u32)
    }

    fn num_intersect(&self) -> u32 {
        self.win_nums.intersection(&self.card_nums).count() as u32
    }
}

#[test]
fn test_card() {
    let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let card = Card::new(&line);
    assert_eq!(card.win_nums.len(), 5);
    assert_eq!(card.card_nums.len(), 8);
    assert_eq!(card.score(), 8);
}

fn to_num_set(nums_raw: &str) -> HashSet<u8> {
    let result = nums_raw
        .split(" ")
        .into_iter()
        .filter_map(|sec| {
            if sec.len() > 0 {
                Some(sec.parse::<u8>().unwrap())
            } else {
                None
            }
        })
        .collect::<HashSet<u8>>();

    result
}

#[test]
fn test_to_num_set() {
    let nums_raw = " 83 86  6 31 17  9 48 53";
    let result = to_num_set(nums_raw);
    assert!(result.len() == 8);
}

struct CardCount<T> {
    counts: Vec<T>,
}

impl <T: AddAssign + Default + Copy> CardCount<T> {
    fn new() -> Self {
        CardCount { counts: vec![] }
    }

    fn add_at_idx(&mut self, idx: usize, count: T) {
        while idx + 1 > self.counts.len() {
            self.counts.push(T::default());
        }
        self.counts[idx] += count;
    }

    fn get_count(&self, idx: usize) -> T {
        return self.counts[idx];
    }

    fn into_vec(self, end: usize) -> Vec<T> {
        self.counts.into_iter().take(end).collect::<Vec<T>>()
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let cards = contents
        .split("\n")
        .into_iter()
        .map(Card::new)
        .collect::<Vec<_>>();

    let p1_result: u64 = cards.iter().map(|card| card.score()).sum();

    println!("results of part 1 = {:?}", p1_result);

    let mut card_counts = CardCount::new();
    for i in 0..cards.len() {
        // add the count for the original
        card_counts.add_at_idx(i, 1);

        let num_matches = cards[i].num_intersect() as usize;
        if num_matches > 0 {
            for j in 0..num_matches {
                card_counts.add_at_idx(i + j + 1, card_counts.get_count(i));
            }
        }
    }

    let card_count = card_counts.into_vec(cards.len());
    let p2_total: u64 = card_count.iter().sum();
    println!("results of part 2 = {:?}", p2_total);

    Ok(())
}
