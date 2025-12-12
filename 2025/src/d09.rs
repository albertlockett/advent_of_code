use std::io::{BufRead, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day09 {}

impl Challenge for Day09 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let coords = Self::read_input_iter(input)?
            .lines()
            .map(|line| {
                line.map(|line| {
                    let mut split = line.split(",");
                    (
                        str::parse::<i64>(split.next().unwrap()).unwrap(),
                        str::parse::<i64>(split.next().unwrap()).unwrap(),
                    )
                })
            })
            .collect::<Result<Vec<_>>>()?;

        let mut max_area = 0;
        for i in 0..coords.len() {
            let (x1, y1) = coords[i];
            #[allow(clippy::needless_range_loop)]
            for j in i + 1..coords.len() {
                let (x2, y2) = coords[j];
                let area = ((x1 - x2 + 1) * (y1 - y2 + 1)).unsigned_abs();
                if area > max_area {
                    max_area = area
                }
            }
        }

        Ok(max_area as usize)
    }

    fn do_p2(&mut self, _input: &str) -> Result<usize> {
        Ok(0)
    }
}
