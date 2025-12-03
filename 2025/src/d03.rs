use std::io::{BufRead, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day03 {}

fn find_max_and_pos(nums: &[u8]) -> (usize, u8) {
    let mut max = 0;
    let mut pos = 0;
    nums.iter().cloned().enumerate().for_each(|(idx, num)| {
        if num > max {
            max = num;
            pos = idx
        }
    });

    (pos, max - b'0')
}

impl Challenge for Day03 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let mut sum_biggest = 0;
        let lines = Self::read_input_iter(input)?.split(b'\n');

        for line in lines {
            let nums = line?;
            let (left_pos, left_num) = find_max_and_pos(&nums[0..nums.len() - 1]);
            let (_, right_num) = find_max_and_pos(&nums[(left_pos + 1)..nums.len()]);
            let biggest = left_num * 10 + right_num;

            sum_biggest += biggest as usize;
        }

        Ok(sum_biggest)
    }

    fn do_p2(&mut self, _input: &str) -> Result<usize> {
        Ok(0)
    }
}
