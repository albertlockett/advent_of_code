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

#[derive(PartialEq)]
enum SplitDir {
    Left,
    Right,
}

impl Challenge for Day07 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let bytes = Self::read_input_iter(input)?.bytes();
        let mut grid = Grid::<Option<ManifoldMember>>::try_from_byte_iter(bytes)?;

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

    fn do_p2(&mut self, input: &str) -> Result<usize> {
        let bytes = Self::read_input_iter(input)?.bytes();
        let grid = Grid::<Option<ManifoldMember>>::try_from_byte_iter(bytes)?;

        // find the start of the traversal
        let mut col = (0..grid.cols)
            .find(|col| matches!(grid.get(0, *col), Some(ManifoldMember::Start)))
            .expect("start there");
        let mut row = (0..grid.rows)
            .find(|row| matches!(grid.get(*row, col), Some(ManifoldMember::Splitter(_))))
            .expect("splitter there");

        let mut traversal_stack = Vec::new();
        traversal_stack.push((SplitDir::Left, (row, col)));

        // position us on left side of the start splitter
        col -= 1;

        // how many paths there were for a given position when we first pushed it onto
        // the traversal stack
        let mut push_val = Grid::<usize>::from_default(grid.rows, grid.cols);

        // how many values there were for a given traversal when we popped it off the stack
        let mut pop_val = Grid::<Option<usize>>::from_default(grid.rows, grid.cols);

        let mut paths = 0;
        while !traversal_stack.is_empty() {
            // move down
            row += 1;

            // if we've reached the bottom, or are at a position where we know how many paths
            // there are below ..
            if row == grid.rows - 1 || pop_val.get(row, col).is_some() {
                // increment count of total paths
                if let Some(val) = pop_val.get(row, col) {
                    paths += val;
                } else {
                    paths += 1;
                }

                // pop stack until we are somewhere we haven't been before
                while let Some((dir, pos)) = traversal_stack.pop() {
                    match dir {
                        // if we went left last time, go right:
                        SplitDir::Left => {
                            // position ourselves on the right
                            row = pos.0;
                            debug_assert!(col < grid.cols);
                            col = pos.1 + 1;
                            traversal_stack.push((SplitDir::Right, pos));
                            break;
                        }

                        // otherwise we went right last time, keep popping
                        SplitDir::Right => {
                            // if it's the first time we seen the # of paths below this
                            // splitter, then cache it
                            if pop_val.get(pos.0, pos.1).is_none() {
                                let paths_below = paths - push_val.get(pos.0, pos.1);
                                pop_val.set(pos.0, pos.1, Some(paths_below));
                            }
                            continue;
                        }
                    }
                }
            } else {
                // push the splitter onto the stack if we're on one
                if matches!(grid.get(row, col), Some(ManifoldMember::Splitter(_))) {
                    traversal_stack.push((SplitDir::Left, (row, col)));
                    debug_assert!(col > 0);
                    push_val.set(row, col, paths);
                    col -= 1;
                }
            }
        }

        Ok(paths)
    }
}
