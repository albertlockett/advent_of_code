use std::io::{BufRead, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day03 {}

impl Day03 {
    fn do_it(input: &str, digs: usize) -> Result<usize> {
        Self::read_input_iter(input)?
            .split(b'\n')
            .map(|line| line.map(|nums| calc_biggest(&nums, digs)))
            .sum()
    }
}

fn find_max_and_pos(nums: &[u8]) -> (usize, usize) {
    let mut max = 0;
    let mut pos = 0;
    nums.iter().cloned().enumerate().for_each(|(idx, num)| {
        if num > max {
            max = num;
            pos = idx
        }
    });

    (pos, (max - b'0') as usize)
}

fn calc_biggest(nums: &[u8], digs: usize) -> usize {
    let mut next_pos = 0;
    let mut total = 0;
    for i in 0..digs {
        let end = digs - (i + 1);
        let (pos, num) = find_max_and_pos(&nums[next_pos..nums.len() - end]);
        next_pos += pos + 1;
        total *= 10;
        total += num;
    }

    total
}

impl Challenge for Day03 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        Self::do_it(input, 2)
    }

    fn do_p2(&mut self, input: &str) -> Result<usize> {
        Self::do_it(input, 12)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_do_it() {
        assert_eq!(calc_biggest(b"987654321111111", 12), 987654321111);
        assert_eq!(calc_biggest(b"811111111111119", 12), 811111111119);
        assert_eq!(calc_biggest(b"234234234234278", 12), 434234234278);
    }
}
