use std::io::{BufRead, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day03 {}

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

impl Challenge for Day03 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let mut sum_biggest = 0;
        let lines = Self::read_input_iter(input)?.split(b'\n');

        for line in lines {
            let nums = line?;
            let (left_pos, left_num) = find_max_and_pos(&nums[0..nums.len() - 1]);
            let (_, right_num) = find_max_and_pos(&nums[(left_pos + 1)..nums.len()]);
            let biggest = left_num * 10 + right_num;

            sum_biggest += biggest
        }

        Ok(sum_biggest)
    }

    fn do_p2(&mut self, input: &str) -> Result<usize> {
        let mut sum_biggest = 0;
        let lines = Self::read_input_iter(input)?.split(b'\n');

        for line in lines {
            let nums = line?;
            let mut next_pos = 0;
            let (p1_pos, p1_num) = find_max_and_pos(&nums[0..nums.len() - 11]);
            next_pos += p1_pos + 1;
            let (p2_pos, p2_num) = find_max_and_pos(&nums[next_pos..nums.len() - 10]);
            next_pos += p2_pos + 1;
            let (p3_pos, p3_num) = find_max_and_pos(&nums[next_pos..nums.len() - 9]);
            next_pos += p3_pos + 1;
            let (p4_pos, p4_num) = find_max_and_pos(&nums[next_pos..nums.len() - 8]);
            next_pos += p4_pos + 1;
            let (p5_pos, p5_num) = find_max_and_pos(&nums[next_pos..nums.len() - 7]);
            next_pos += p5_pos + 1;
            let (p6_pos, p6_num) = find_max_and_pos(&nums[next_pos..nums.len() - 6]);
            next_pos += p6_pos + 1;
            let (p7_pos, p7_num) = find_max_and_pos(&nums[next_pos..nums.len() - 5]);
            next_pos += p7_pos + 1;
            let (p8_pos, p8_num) = find_max_and_pos(&nums[next_pos..nums.len() - 4]);
            next_pos += p8_pos + 1;
            let (p9_pos, p9_num) = find_max_and_pos(&nums[next_pos..nums.len() - 3]);
            next_pos += p9_pos + 1;
            let (p10_pos, p10_num) = find_max_and_pos(&nums[next_pos..nums.len() - 2]);
            next_pos += p10_pos + 1;
            let (p11_pos, p11_num) = find_max_and_pos(&nums[next_pos..nums.len() - 1]);
            next_pos += p11_pos + 1;
            let (_, p12_num) = find_max_and_pos(&nums[next_pos..nums.len()]);

            let biggest = p12_num
                + p11_num * 10
                + p10_num * 100
                + p9_num * 1000
                + p8_num * 10000
                + p7_num * 100000
                + p6_num * 1000000
                + p5_num * 10000000
                + p4_num * 100000000
                + p3_num * 1000000000
                + p2_num * 10000000000
                + p1_num * 100000000000;

            sum_biggest += biggest;
        }

        Ok(sum_biggest)
    }
}
