use std::io::{BufRead, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day02 {}

impl Day02 {
    fn run_with_validity<F>(input: &str, is_valid: F) -> Result<usize>
    where
        F: Fn(usize) -> bool,
    {
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
}

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

fn is_valid_p1(input: usize) -> bool {
    let num = format!("{input}");

    let len = num.len();

    if len % 2 == 1 {
        return true;
    }

    let first_half = &num[0..len / 2];
    let next_half = &num[len / 2..len];

    first_half != next_half
}

fn is_valid_p2(input: usize) -> bool {
    let num = format!("{input}");
    let num_len = num.len();

    for range_len in 1..=num_len / 2 {
        // need to be able to divide candidate into input evenly times
        if num_len % range_len != 0 {
            continue;
        }

        let candidate = &num[0..range_len];

        let mut valid_for_range_len = false;
        for repeat in 1..num_len / range_len {
            let start = repeat * range_len;
            let end = (repeat + 1) * range_len;
            let maybe_repeat = &num[start..end];

            if candidate != maybe_repeat {
                valid_for_range_len = true;
                break;
            }
        }

        if !valid_for_range_len {
            return false;
        }
    }

    true
}

impl Challenge for Day02 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        Self::run_with_validity(input, is_valid_p1)
    }

    fn do_p2(&mut self, input: &str) -> Result<usize> {
        Self::run_with_validity(input, is_valid_p2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_p2() {
        assert!(is_valid_p2(54));
        assert!(!is_valid_p2(55));
    }
}
