use std::fs::File;
use std::io::prelude::*;

fn hash(s: &str) -> u128 {
    let mut curr_val = 0;
    for c in s.chars() {
        curr_val += c as u128;
        curr_val *= 17;
        curr_val %= 256;
    }
    return curr_val
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let p1_total: u128 = contents.split("\n").flat_map(|s| s.split(",")).map(|s| hash(s)).sum();
    println!("p1 total = {}", p1_total);
}
