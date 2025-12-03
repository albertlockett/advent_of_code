use std::fs::File;
use std::io::{BufReader, Result};
use std::time::{Duration, Instant};

pub mod d01;
pub mod d02;
pub mod d03;

pub trait Challenge: Default {
    fn do_p1(&mut self, input: &str) -> Result<usize>;

    fn do_p2(&mut self, input: &str) -> Result<usize>;

    fn read_input_iter(path: &str) -> Result<BufReader<File>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }
}

#[derive(Debug)]
pub struct ChallengeAnswer {
    part_01: usize,
    part_02: usize,
    p1_time: Duration,
    p2_time: Duration,
}

impl PartialEq for ChallengeAnswer {
    fn eq(&self, other: &Self) -> bool {
        self.part_01 == other.part_01 && self.part_02 == other.part_02
    }
}

impl ChallengeAnswer {
    pub fn new(part_01: usize, part_02: usize) -> Self {
        Self::new_with_timing(
            part_01,
            part_02,
            Duration::from_nanos(0),
            Duration::from_nanos(0),
        )
    }

    pub fn new_with_timing(
        part_01: usize,
        part_02: usize,
        p1_time: Duration,
        p2_time: Duration,
    ) -> Self {
        Self {
            part_01,
            part_02,
            p1_time,
            p2_time,
        }
    }
}

impl std::fmt::Display for ChallengeAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "p1={}, p2={}, p1_time={:?}, p2_time={:?}, total_time={:?}",
            self.part_01,
            self.part_02,
            self.p1_time,
            self.p2_time,
            self.p1_time + self.p2_time
        )
    }
}

pub fn run<T: Challenge>(input: &str) -> Result<ChallengeAnswer> {
    let mut day = T::default();

    let start = Instant::now();
    let p1 = day.do_p1(input)?;
    let p1_time = start.elapsed();

    let start = Instant::now();
    let p2 = day.do_p2(input)?;
    let p2_time = start.elapsed();

    Ok(ChallengeAnswer::new_with_timing(p1, p2, p1_time, p2_time))
}
