use std::io::{BufRead, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day02 {}

fn to_range(bytes: &[u8]) -> (usize, usize) {
    let mut start = 0usize;
    let mut end = 0usize;
    let mut target = &mut start;

    for b in bytes {
        match b {
            b'-' => {
                target = &mut end;
            }
            num => {
                *target *= 10;
                *target += (num - b'0') as usize;
            }
        }
    }

    (start, end)
}

fn is_valid(input: usize) -> bool {
    let num = format!("{input}");

    let len = num.len();

    if len % 2 == 1 {
        return true;
    }

    let first_half = &num[0..len / 2];
    let next_half = &num[len / 2..len];

    first_half != next_half
}

impl Challenge for Day02 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let ranges = Self::read_input_iter(input)?
            .split(b',')
            .map(|range| range.map(|r| to_range(&r)));

        let mut sum_invalid = 0;
        for range in ranges {
            let (start, end) = range?;
            for i in start..end {
                if !is_valid(i) {
                    sum_invalid += i;
                }
            }
        }

        Ok(sum_invalid)
    }

    fn do_p2(&mut self, _input: &str) -> Result<usize> {
        Ok(0)
    }
}
