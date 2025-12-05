use std::io::{Read, Result};

use crate::Challenge;
use crate::grid::{self, Grid};

#[derive(Default)]
pub struct Day04 {}

#[derive(Debug, Default)]
struct PaperRoll {}

impl grid::Display for PaperRoll {
    fn fmt(&self) -> char {
        '@'
    }
}

impl Challenge for Day04 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let bytes = Self::read_input_iter(input)?.bytes();
        let grid = Grid::<Option<PaperRoll>>::try_from_byte_iter(bytes)?;

        let mut accessible_roles = 0;
        for row in 0..grid.rows {
            for col in 0..grid.cols {
                if grid.get(row, col).is_some() {
                    let neighbours = grid.iter_neighbours(row, col).flatten().count();
                    if neighbours < 4 {
                        accessible_roles += 1;
                    }
                }
            }
        }

        Ok(accessible_roles)
    }

    fn do_p2(&mut self, _input: &str) -> Result<usize> {
        Ok(0)
    }
}
