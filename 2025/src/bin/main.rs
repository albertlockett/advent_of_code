use aoc2025::d01::Day01;
use aoc2025::{ChallengeAnswer, run};

fn main() {
    let result = run::<Day01>("inputs/d01.txt").unwrap();
    println!("{result}");
    assert_eq!(result, ChallengeAnswer::new(1066, 6223))

    // 7026 too high
}
