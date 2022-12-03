use std::fs::File;
use std::io::{BufRead, BufReader, Read};

trait Mask {
    fn new() -> Self;
    fn contains(&self, val: &Self) -> bool;
    fn insert(&mut self, val: &Self);
    fn reset(&mut self);
}

impl Mask for u32 {
    fn new() -> Self {
        0
    }

    fn contains(&self, val: &u32) -> bool {
        *self & bitmask_flag(*val) > 0
    }

    fn insert(&mut self, val: &u32) {
        if !self.contains(val) {
            *self += bitmask_flag(*val)
        }
    }

    fn reset(&mut self) {
        *self = 0;
    }
}

fn bitmask_flag(val: u32) -> u32 {
    1 << val
}

const ASCII_A_LOWER: u8 = 0x60;
const ASCII_Z_LOWER: u8 = ASCII_A_LOWER + 26;
const ASCII_A_UPPER: u8 = 0x40;
const ASCII_Z_UPPER: u8 = ASCII_A_UPPER + 26;

fn priority_of_line(line: &str) -> u32 {
    let mut lmask: u32 = Mask::new();
    let mut umask: u32 = Mask::new();

    let mut curr_offset = 0;
    let compartment_2_offset = line.len() / 2;

    let priority = line.as_bytes().into_iter().find_map(|b| {
        let in_compartment_2 = curr_offset >= compartment_2_offset;
        curr_offset += 1;

        // handle lowercase letter
        if *b > ASCII_A_LOWER && *b <= ASCII_Z_LOWER {
            let letter_num = u32::from(b - ASCII_A_LOWER);
            let item_in_comp_1 = lmask.contains(&letter_num);

            if !in_compartment_2 && !item_in_comp_1 {
                lmask.insert(&letter_num);
            }
            if in_compartment_2 && item_in_comp_1 {
                let priority = letter_num;
                return Some(priority);
            }
        }

        // handle uppercase letter
        if *b > ASCII_A_UPPER && *b <= ASCII_Z_UPPER {
            let letter_num = u32::from(b - ASCII_A_UPPER);
            let item_in_comp_1 = umask.contains(&letter_num);
            if !in_compartment_2 && !item_in_comp_1 {
                umask.insert(&letter_num);
            }

            if in_compartment_2 && item_in_comp_1 {
                let priority = letter_num + 26;
                return Some(priority);
            }
        }

        return None;
    });

    return priority.unwrap();
}

#[test]
fn priority_of_line_test1() {
    assert_eq!(52, priority_of_line("tmGZtjGjHZpVbfMT"));
    assert_eq!(16, priority_of_line("vJrwpWtwJgWrhcsFMMfFFhFp"));
    assert_eq!(38, priority_of_line("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
    assert_eq!(42, priority_of_line("PmmdzqPrVvPwwTWBwg"));
    assert_eq!(22, priority_of_line("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
    // assert_eq!(22)
}

fn main() {
    let mut total_priority: u32 = 0;

    let input_file = File::open("input.txt").expect("biffed it getting input");
    let input_reader = BufReader::new(input_file);
    for line_result in input_reader.lines() {
        match line_result {
            Ok(line) => {
                total_priority += priority_of_line(&line);
            }
            Err(e) => {}
        }
    }

    println!("P1 the total priority is {:?}", total_priority);
}
