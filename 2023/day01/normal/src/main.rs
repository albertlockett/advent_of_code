use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: u64 = contents.split("\n")
        .into_iter()
        .map(|line| {
            let num = parse_num_from_line_v2(line);
            print!("{:?} = {:?}\n", line, num);
            num
        })
        .sum();

    print!("Sum of all numbers in file: {:?}\n", lines);
    Ok(())
}

fn parse_num_from_line(line: &str) -> u64 {
    let mut first_digit = None;
    let mut last_digit = None;

    line.as_bytes().iter().for_each(|byte| {
        if byte.is_ascii_digit() {
            if first_digit.is_none() {
                first_digit = Some(byte - 48);
            } else {
                last_digit = Some(byte - 48);
            }
        }
    });

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => (first * 10 + last) as u64,
        (Some(first), None) => (first * 10 + first) as u64,
        _ => 0 as u64
    }
}

fn parse_num_from_line_v2(line: &str) -> u64 {
    let candidates = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ];


    let mut first_start_idx = 9999999;
    let mut first_digit = None;

    let mut last_start_idx = 0;
    let mut last_digit = None;

    for (word, num) in candidates {
        let segments: Vec<&str> = line.split(word).into_iter().collect();
        if segments.len() > 1 {
            let first_start = segments[0].len();
            let last_start = line.len() - word.len() - segments[segments.len() - 1].len();

            if first_start < first_start_idx {
                first_start_idx = first_start;
                first_digit = Some(num);
            };

            if last_start >= last_start_idx {
                last_start_idx = last_start;
                last_digit = Some(num);
            };
        }

    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => (first * 10 + last) as u64,
        (Some(first), None) => (first * 10 + first) as u64,
        _ => 0 as u64
    }
}
