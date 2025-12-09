use std::io::{Read, Result};

use crate::Challenge;
use crate::grid::{Display, Grid, GridChar};

#[derive(Default)]
pub struct Day07 {}

enum ManifoldMember {
    Start,
    Splitter(bool),
    Beam,
}

impl Display for ManifoldMember {
    fn fmt(&self) -> char {
        match self {
            Self::Start => 'S',
            Self::Beam => '|',
            Self::Splitter(_) => '^',
        }
    }
}

impl From<GridChar> for ManifoldMember {
    fn from(value: GridChar) -> Self {
        match *value {
            b'S' => Self::Start,
            b'^' => Self::Splitter(false),
            b'|' => Self::Beam,
            invalid => panic!("invalid input {invalid}"),
        }
    }
}

impl Challenge for Day07 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let bytes = Self::read_input_iter(input)?.bytes();
        let mut grid = Grid::<Option<ManifoldMember>>::try_from_byte_iter(bytes)?;

        // grid.display();

        // go through and activate all the splitters
        for row in 0..(grid.rows - 1) {
            for col in 0..grid.cols {
                match grid.get(row, col) {
                    Some(member) => match member {
                        ManifoldMember::Start => {
                            // here we gotta set the row below to a beam
                            grid.set(row + 1, col, Some(ManifoldMember::Beam));
                        }
                        ManifoldMember::Beam => {
                            match grid.get(row + 1, col) {
                                Some(member) => match member {
                                    ManifoldMember::Splitter(_) => {
                                        // set that the splitter is active
                                        grid.set(
                                            row + 1,
                                            col,
                                            Some(ManifoldMember::Splitter(true)),
                                        );

                                        // set the beam going down onto the next row
                                        if col > 0 {
                                            grid.set(row + 1, col - 1, Some(ManifoldMember::Beam))
                                        }
                                        if col < grid.cols - 1 {
                                            grid.set(row + 1, col + 1, Some(ManifoldMember::Beam))
                                        }
                                    }
                                    _ => {
                                        grid.set(row + 1, col, Some(ManifoldMember::Beam));
                                    }
                                },
                                None => {
                                    // beam propagates to row below
                                    grid.set(row + 1, col, Some(ManifoldMember::Beam))
                                }
                            }
                        }
                        ManifoldMember::Splitter(_) => {
                            // do nothing
                        }
                    },
                    None => {
                        // do noting
                    }
                }
            }
        }

        // grid.display();

        // now go through and count all the active splitters
        let mut total_active_splitters = 0;
        for row in 0..(grid.rows) {
            for col in 0..grid.cols {
                if let Some(ManifoldMember::Splitter(active)) = grid.get(row, col) {
                    if *active {
                        total_active_splitters += 1;
                    }
                }
            }
        }

        Ok(total_active_splitters)
    }

    fn do_p2(&mut self, _input: &str) -> Result<usize> {
        Ok(0)
    }
}
