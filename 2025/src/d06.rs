use std::{
    io::{BufRead, Result},
    str::Chars,
};

use crate::Challenge;

#[derive(Default)]
pub struct Day06 {}

impl Challenge for Day06 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let mut lines = Self::read_input_iter(input)?
            .lines()
            .collect::<Result<Vec<_>>>()?;

        let op_line = lines.pop().expect("not empty");
        let op_iter = OpIter::new(op_line.chars());
        let mut arg_iters = lines
            .iter()
            .map(|line| ArgsIter::new(line.chars()))
            .collect::<Vec<_>>();

        let mut grand_total = 0;
        for op in op_iter {
            let mut col_total = match op {
                Op::Add => 0,
                Op::Multiply => 1,
            };

            for arg_iter in &mut arg_iters {
                let arg = arg_iter.next().expect("not exhausted");
                col_total = match op {
                    Op::Add => col_total + arg,
                    Op::Multiply => col_total * arg,
                };
            }

            grand_total += col_total;
        }

        Ok(grand_total)
    }

    fn do_p2(&mut self, input: &str) -> Result<usize> {
        let mut lines = Self::read_input_iter(input)?
            .lines()
            .collect::<Result<Vec<_>>>()?;

        let op_line = lines.pop().expect("not empty");
        let op_iter = OpIter::new(op_line.chars());
        let range_iter = ArgColumnRangeIter::new(op_line.chars());

        let arg_lines = lines.iter().map(|line| line.as_bytes()).collect::<Vec<_>>();

        // reuse the args vec to avoid heap allocation each range
        let mut args = Vec::with_capacity(arg_lines.len());

        let mut grand_total = 0;
        for (op, range) in op_iter.zip(range_iter) {
            args_from_range(range, &arg_lines, &mut args);

            let mut range_total = match op {
                Op::Add => 0,
                Op::Multiply => 1,
            };

            for arg in &args {
                match op {
                    Op::Add => range_total += *arg,
                    Op::Multiply => range_total *= *arg,
                }
            }

            grand_total += range_total;
        }

        Ok(grand_total)
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

/// iterates the operators from the last line of input
struct OpIter<'a> {
    chars: Chars<'a>,
}

impl<'a> OpIter<'a> {
    fn new(chars: Chars<'a>) -> Self {
        Self { chars }
    }
}

impl<'a> Iterator for OpIter<'a> {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.chars.next()? {
                '+' => return Some(Op::Add),
                '*' => return Some(Op::Multiply),
                _ => {
                    // continue
                }
            }
        }
    }
}

// iterates the row-wise args for each line (for part 1)
struct ArgsIter<'a> {
    chars: Chars<'a>,
}

impl<'a> ArgsIter<'a> {
    fn new(chars: Chars<'a>) -> Self {
        Self { chars }
    }
}

impl<'a> Iterator for ArgsIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = None;
        loop {
            match self.chars.next() {
                Some(' ') => {
                    if next.is_some() {
                        return next;
                    }
                }
                Some(num) => {
                    if next.is_none() {
                        next = Some(0);
                    }

                    let next = next.as_mut().expect("initialized");
                    *next *= 10;
                    *next += (num as u8 - b'0') as usize;
                }
                None => return next,
            }
        }
    }
}

/// computes the vertical ranges of arguments from the op column, which happens to always align
/// with the +/* operators (hence why this iterates the op column)
struct ArgColumnRangeIter<'a> {
    chars: Chars<'a>,
    curr_range_start: usize,
    exhausted: bool,
}

impl<'a> ArgColumnRangeIter<'a> {
    fn new(mut op_line_chars: Chars<'a>) -> Self {
        // advance b/c we know the first char starts a range
        _ = op_line_chars.next();

        Self {
            chars: op_line_chars,
            curr_range_start: 0,
            exhausted: false,
        }
    }
}

impl<'a> Iterator for ArgColumnRangeIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let mut idx = self.curr_range_start + 1;

        // iterate until we reach the start of next column of arguments
        loop {
            match self.chars.next() {
                Some(' ') => idx += 1,
                Some(_) => {
                    let range = (self.curr_range_start, idx - 1);
                    self.curr_range_start = idx;
                    return Some(range);
                }
                None => {
                    self.exhausted = true;
                    return Some((self.curr_range_start, idx));
                }
            }
        }
    }
}

/// populates results of add/mult args from weird columnar arg layout
fn args_from_range(range: (usize, usize), arg_col_bytes: &[&[u8]], results: &mut Vec<usize>) {
    // initialize results vec
    results.clear();

    for col in range.0..range.1 {
        let mut col_val = 0;
        for arg_col in arg_col_bytes {
            match arg_col[col] {
                b' ' => {
                    // ignore
                }
                num => {
                    col_val *= 10;
                    col_val += (num - b'0') as usize;
                }
            }
        }
        results.push(col_val);
    }
}
