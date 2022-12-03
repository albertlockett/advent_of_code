use std::fs::File;
use std::io::{BufRead, BufReader};

const LETTERS_IN_ALPHA: u8 = 26;
const ASCII_A_LOWER: u8 = 0x60;
const ASCII_Z_LOWER: u8 = ASCII_A_LOWER + LETTERS_IN_ALPHA;
const ASCII_A_UPPER: u8 = 0x40;
const ASCII_Z_UPPER: u8 = ASCII_A_UPPER + LETTERS_IN_ALPHA;

macro_rules! is_lower {
    ($b:expr) => {{
        *$b > ASCII_A_LOWER && *$b <= ASCII_Z_LOWER
    }};
}

macro_rules! letternum_lower {
    ($b:expr) => {{
        u32::from($b - ASCII_A_LOWER)
    }};
}

macro_rules! is_upper {
    ($b:expr) => {{
        *$b > ASCII_A_UPPER && *$b <= ASCII_Z_UPPER
    }};
}

macro_rules! letternum_upper {
    ($b:expr) => {{
        u32::from($b - ASCII_A_UPPER)
    }};
}

macro_rules! priority_upper {
    ($b:expr) => {
        $b + u32::from(LETTERS_IN_ALPHA)
    };
}

trait Mask {
    fn new() -> Self;
    fn contains(&self, val: u32) -> bool;
    fn insert(&mut self, val: u32);
}

impl Mask for u32 {
    fn new() -> Self {
        0
    }

    fn contains(&self, val: u32) -> bool {
        *self & bitmask_flag(val) > 0
    }

    fn insert(&mut self, val: u32) {
        if !self.contains(val) {
            *self += bitmask_flag(val)
        }
    }
}

fn bitmask_flag(val: u32) -> u32 {
    1 << val
}

fn priority_of_line(line: &str) -> u32 {
    let mut lmask: u32 = Mask::new();
    let mut umask: u32 = Mask::new();

    let mut curr_offset = 0;
    let compartment_2_offset = line.len() / 2;

    let priority = line.as_bytes().into_iter().find_map(|b| {
        let in_compartment_2 = curr_offset >= compartment_2_offset;
        curr_offset += 1;

        // handle lowercase letter
        if is_lower!(b) {
            let letter_num = letternum_lower!(b);
            let item_in_comp_1 = lmask.contains(letternum_lower!(b));

            if !in_compartment_2 && !item_in_comp_1 {
                lmask.insert(letter_num);
            }
            if in_compartment_2 && item_in_comp_1 {
                return Some(letter_num);
            }
        }

        // handle uppercase letter
        if is_upper!(b) {
            let letter_num = letternum_upper!(b);
            let item_in_comp_1 = umask.contains(letter_num);
            if !in_compartment_2 && !item_in_comp_1 {
                umask.insert(letter_num);
            }

            if in_compartment_2 && item_in_comp_1 {
                return Some(priority_upper!(letter_num));
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
}

fn fill_masks(b: &u8, lmask: &mut u32, umask: &mut u32) {
    if is_lower!(b) {
        lmask.insert(letternum_lower!(b));
    }
    if is_upper!(b) {
        umask.insert(letternum_upper!(b));
    }
}

fn priority_of_line_group(line1: &str, line2: &str, line3: &str) -> u32 {
    let mut lmask1: u32 = Mask::new();
    let mut umask1: u32 = Mask::new();
    let mut lmask2: u32 = Mask::new();
    let mut umask2: u32 = Mask::new();

    line1.as_bytes().into_iter().for_each(|b| {
        fill_masks(b, &mut lmask1, &mut umask1);
    });
    line2.as_bytes().into_iter().for_each(|b| {
        fill_masks(b, &mut lmask2, &mut umask2);
    });

    let priority = line3.as_bytes().into_iter().find_map(|b| {
        if is_lower!(b) {
            let letter_num = letternum_lower!(b);
            if lmask1.contains(letter_num) && lmask2.contains(letter_num) {
                return Some(letter_num);
            }
        }
        if is_upper!(b) {
            let letter_num = letternum_upper!(b);
            if umask1.contains(letter_num) && umask2.contains(letter_num) {
                return Some(priority_upper!(letter_num));
            }
        }
        None
    });

    return priority.unwrap();
}

#[test]
fn priority_of_line_group_test() {
    assert_eq!(
        18,
        priority_of_line_group(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        )
    );
    assert_eq!(
        52,
        priority_of_line_group(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        )
    );
    assert_eq!(
        36,
        priority_of_line_group(
            "gfWpjRRQffQGCHHJsGqjsj",
            "SclzJZZvmmnPbJtVSqqNBqVCBdSCsd",
            "tlbvZJDZtmtPcJmlPnhMFQWWpMRFTfLDRRTWRp"
        )
    )
}

fn main() {
    let mut total_priority_p1: u32 = 0;
    let mut total_priority_p2: u32 = 0;

    let mut lines = vec![];

    let input_file = File::open("input.txt").expect("biffed it getting input");
    let input_reader = BufReader::new(input_file);
    for line_result in input_reader.lines() {
        match line_result {
            Ok(line) => {
                total_priority_p1 += priority_of_line(&line);
                lines.push(line);
                if lines.len() == 3 {
                    total_priority_p2 += priority_of_line_group(&lines[0], &lines[1], &lines[2]);
                    lines.clear();
                }
            }
            Err(_e) => {}
        }
    }

    println!("P1 the total priority is {:?}", total_priority_p1);
    println!("P2 the total priority is {:?}", total_priority_p2);
}
