use std::io::{Read, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day01 {}

impl Challenge for Day01 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let mut row_dial_pos = 50;
        let mut curr_row_val = 0;
        let mut going_left = false;
        let mut num_zeros = 0;

        let bytes_iter = Self::read_input_iter(input)?.bytes();
        for b in bytes_iter {
            match b? {
                b'L' => {
                    going_left = true;
                }
                b'R' => {
                    going_left = false;
                }
                b'\n' => {
                    if going_left {
                        row_dial_pos += curr_row_val
                    } else {
                        row_dial_pos -= curr_row_val
                    }

                    row_dial_pos %= 100;
                    if row_dial_pos == 0 {
                        num_zeros += 1;
                    }

                    // reset
                    curr_row_val = 0;
                }
                num => {
                    curr_row_val *= 10;
                    curr_row_val += (num - b'0') as i32
                }
            }
        }

        Ok(num_zeros)
    }
}
