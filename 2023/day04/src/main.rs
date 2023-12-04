
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

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

        let win_nums_sec = num_sec_iter.next().unwrap();
        let card_nums_sec = num_sec_iter.next().unwrap();

        let win_nums = to_num_set(win_nums_sec);
        let card_nums = to_num_set(card_nums_sec);

        Card {
            win_nums,
            card_nums,
        }
    }

    fn p1_score(&self) -> u64 {
        self.win_nums.intersection(&self.card_nums).for_each(|f| println!("INTERSECT {}", f));
        let intersection_size = self.win_nums.intersection(&self.card_nums).count();
        if intersection_size == 0 {
            return 0
        }

        let base: u64 = 2;
        base.pow((intersection_size - 1) as u32)
    }
}

#[test]
fn test_card() {
    let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let card = Card::new(&line);
    assert_eq!(card.win_nums.len(), 5);
    assert_eq!(card.card_nums.len(), 8);
    assert_eq!(card.p1_score(), 8);
}

fn to_num_set(nums_raw: &str) -> HashSet<u8> {
    let result = nums_raw.split(" ").into_iter()
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

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let p1_result: u64 = contents.split("\n")
        .into_iter()
        .map(Card::new)
        .map(|card| card.p1_score())
        .sum();

    println!("results of part 1 = {:?}", p1_result);

    Ok(())
}
