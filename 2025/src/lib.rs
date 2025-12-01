use std::fs::File;
use std::io::{BufReader, Result};

pub mod d01;

pub trait Challenge: Default {
    fn do_p1(&mut self, input: &str) -> Result<usize>;

    fn read_input_iter(path: &str) -> Result<BufReader<File>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }
}

pub struct ChallengeAnswer {
    part_01: usize,
    part_02: usize,
}

impl std::fmt::Display for ChallengeAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "part 1 = {}\npart 2 = {}", self.part_01, self.part_02)
    }
}

pub fn run<T: Challenge>(input: &str) -> Result<ChallengeAnswer> {
    let mut day = T::default();
    Ok(ChallengeAnswer {
        part_01: day.do_p1(input)?,
        part_02: 0,
    })
}
