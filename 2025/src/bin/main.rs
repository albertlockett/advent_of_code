use aoc2025::d01::Day01;
use aoc2025::d02::Day02;
use aoc2025::d03::Day03;
use aoc2025::d04::Day04;
use aoc2025::d05::Day05;
use aoc2025::d06::Day06;
use aoc2025::d07::Day07;
use aoc2025::d08::Day08;
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
    assert_eq!(result, ChallengeAnswer::new(16993, 168617068915447));

    let result = run::<Day04>("inputs/d04.txt").unwrap();
    println!("day4 = {result}");
    assert_eq!(result, ChallengeAnswer::new(1578, 10132));

    let result = run::<Day05>("inputs/d05.txt").unwrap();
    println!("day5 = {result}");
    assert_eq!(result, ChallengeAnswer::new(511, 350939902751909));

    let result = run::<Day06>("inputs/d06.txt").unwrap();
    println!("day6 = {result}");
    assert_eq!(result, ChallengeAnswer::new(5335495999141, 10142723156431));

    let result = run::<Day07>("inputs/d07.txt").unwrap();
    println!("day7 = {result}");
    assert_eq!(result, ChallengeAnswer::new(1541, 80158285728929));

    let result = run::<Day08>("inputs/d08.txt").unwrap();
    println!("day8 = {result}");
}
