use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: u64 = contents.split("\n")
        .into_iter()
        .map(|line| {
            print!("{:?} = {:?}\n", line, parse_num_from_line(line));
            parse_num_from_line(line)
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
