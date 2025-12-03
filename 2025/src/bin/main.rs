use aoc2025::d01::Day01;
use aoc2025::d02::Day02;
use aoc2025::d03::Day03;
use aoc2025::{ChallengeAnswer, run};

fn main() {
    let result = run::<Day01>("inputs/d01.txt").unwrap();
    println!("day1 = {result}");
    assert_eq!(result, ChallengeAnswer::new(1066, 6223));

    let result = run::<Day02>("inputs/d02.txt").unwrap();
    println!("day2 = {result}");
    assert_eq!(result, ChallengeAnswer::new(29818212493, 37432260594));

    let result = run::<Day03>("inputs/d03.txt").unwrap();
    println!("day3 = {result}");
    // assert_eq!(result, ChallengeAnswer::new(16993, 168617068915447))
}
