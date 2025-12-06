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

    fn do_p2(&mut self, _input: &str) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

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
