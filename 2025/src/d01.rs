use std::io::{Read, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day01 {}

struct InputIter<T> {
    inner: T,
    going_left: bool,
    curr_row_val: i32,
    exhausted: bool,
}

impl<T> InputIter<T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            going_left: false,
            curr_row_val: 0,
            exhausted: false,
        }
    }
}

impl<T> Iterator for InputIter<T>
where
    T: Iterator<Item = Result<u8>>,
{
    type Item = Result<(bool, i32)>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        loop {
            match self.inner.next() {
                Some(Ok(b)) => {
                    match b {
                        b'L' => {
                            self.going_left = true;
                        }
                        b'R' => {
                            self.going_left = false;
                        }
                        b'\n' => {
                            let result = (self.going_left, self.curr_row_val);
                            // reset
                            self.curr_row_val = 0;
                            return Some(Ok(result));
                        }
                        num => {
                            self.curr_row_val *= 10;
                            self.curr_row_val += (num - b'0') as i32
                        }
                    }
                }
                None => {
                    self.exhausted = true;
                    return Some(Ok((self.going_left, self.curr_row_val)));
                }
                Some(Err(e)) => return Some(Err(e)),
            };
        }
    }
}

impl Challenge for Day01 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let mut row_dial_pos = 50;
        let mut num_zeros = 0;

        let bytes_iter = Self::read_input_iter(input)?.bytes();
        let input_iter = InputIter::new(bytes_iter);
        for row in input_iter {
            let (going_left, curr_row_val) = row?;
            if going_left {
                row_dial_pos += curr_row_val
            } else {
                row_dial_pos -= curr_row_val
            }

            row_dial_pos %= 100;
            if row_dial_pos == 0 {
                num_zeros += 1;
            }
        }

        Ok(num_zeros)
    }

    fn do_p2(&mut self, input: &str) -> Result<usize> {
        let mut row_dial_pos = 50;
        let mut num_zeros = 0;

        let bytes_iter = Self::read_input_iter(input)?.bytes();
        let input_iter = InputIter::new(bytes_iter);

        for row in input_iter {
            let (going_left, curr_row_val) = row?;
            for _ in 0..curr_row_val {
                if going_left {
                    row_dial_pos -= 1;
                    if row_dial_pos == -1 {
                        row_dial_pos = 99;
                    }
                } else {
                    row_dial_pos += 1;
                    if row_dial_pos == 100 {
                        row_dial_pos = 0;
                    }
                }

                if row_dial_pos == 0 {
                    num_zeros += 1
                }
            }
        }

        Ok(num_zeros)
    }
}
