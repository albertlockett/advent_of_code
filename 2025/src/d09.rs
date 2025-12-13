use std::{
    io::{BufRead, Result},
    str::FromStr,
};

use crate::Challenge;

#[derive(Default)]
pub struct Day09 {}

impl Day09 {
    fn read_cords<T>(input: &str) -> Result<Vec<(T, T)>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        Self::read_input_iter(input)?
            .lines()
            .map(|line| {
                line.map(|line| {
                    let mut split = line.split(",");
                    (
                        str::parse::<T>(split.next().unwrap()).unwrap(),
                        str::parse::<T>(split.next().unwrap()).unwrap(),
                    )
                })
            })
            .collect::<Result<Vec<_>>>()
    }
}

impl Challenge for Day09 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let coords = Self::read_cords::<i64>(input)?;
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

    fn do_p2(&mut self, input: &str) -> Result<usize> {
        let coords = Self::read_cords::<usize>(input)?;

        // initialize a grid (internal bitmap) containing which rows are inside
        // (+ 2 so we don't have to futz w/ out of bounds I guess)
        let max_row = coords
            .iter()
            .map(|(x, _)| x)
            .copied()
            .max()
            .expect("not empty")
            + 2;
        let max_col = coords
            .iter()
            .map(|(_, y)| y)
            .copied()
            .max()
            .expect("not empty")
            + 2;
        let mut perimeter = GiantBitmapGrid::new(max_row, max_col);

        // fill in the perimeter of the gird
        let mut last_cord = coords[0];
        for curr_cord in coords.iter().copied().skip(1) {
            perimeter.fill_range(last_cord, curr_cord);
            last_cord = curr_cord;
        }

        // fill in the remaining edge
        perimeter.fill_range(
            *coords.last().expect("not empty"),
            *coords.first().expect("not_empty"),
        );

        // now we're going to build up a set of points that are outside the loop
        // some of the code below may seem strange, but it makes sense when you realize
        // that the loop is a big circle with a rectangle cut out of the middle centered
        // at (50_000, 50_000). I figured this out using excel.
        let mut outside = Outside::new(max_row, max_col);

        fn calc_squared_dist_from_middle(row: usize, col: usize) -> usize {
            let dx = row.max(50000) - row.min(50000);
            let dy = col.max(50000) - col.min(50000);
            dx * dx + dy * dy
        }

        let mut max_dist_from_middle = 0;
        for (row, col) in coords.iter().copied() {
            let dist = calc_squared_dist_from_middle(row, col);
            if max_dist_from_middle < dist {
                max_dist_from_middle = dist;
            }
        }

        // we'll only fill in the outside set around a thin margin around the circle.
        // this is the radius squared of that outer ring
        let outer_radius_squared = (max_dist_from_middle as f64 * 1.0001) as usize;

        // start at a known outside point that is within the outside radius
        let mut curr_squares = vec![(1600, 52000)];
        let mut next_squares = Vec::new();

        while !curr_squares.is_empty() {
            for (row, col) in curr_squares.iter().copied() {
                // check if it's a bunch of stuff we don't wanna put in the outside grid
                if row == 0 || row > perimeter.rows - 1 {
                    continue;
                }
                if col == 0 || col > perimeter.cols - 1 {
                    continue;
                }

                // check if we're more than some threshold from the center, don't bother
                let dist = calc_squared_dist_from_middle(row, col);
                if dist > outer_radius_squared {
                    continue;
                }

                // if it's on the inside, it's not on the outside
                if perimeter.get(row, col) {
                    continue;
                }

                // we already know it's on the outside, so skip
                if outside.contains(row, col) {
                    continue;
                }

                outside.set(row, col);

                // push the neighbours
                for next in [
                    (row - 1, col - 1), // top left
                    (row - 1, col),     // top
                    (row - 1, col + 1), // top right
                    (row, col + 1),     // right
                    (row + 1, col + 1), // bottom right
                    (row + 1, col),     // bottom
                    (row + 1, col - 1), // bottom left
                    (row, col - 1),     // left
                ] {
                    next_squares.push(next);
                }
            }

            std::mem::swap(&mut curr_squares, &mut next_squares);
            next_squares.clear();
        }

        // now it's time to find the biggest rectangle possible
        let mut max_area = 0;
        for i in 0..coords.len() {
            let (x1, y1) = coords[i];

            // first find how far we can go in any direction before we hit the outside.
            // this will speed things up b/c we can eliminate rectangles cornering other
            // squares w/out having to check every square
            let mut min_x = x1;
            while !outside.contains(min_x, y1) {
                min_x -= 1;
            }
            min_x += 1;

            let mut max_x = x1;
            while !outside.contains(max_x, y1) {
                max_x += 1;
            }
            max_x -= 1;

            let mut min_y = y1;
            while !outside.contains(x1, min_y) {
                min_y -= 1;
            }
            min_y += 1;

            let mut max_y = y1;
            while !outside.contains(x1, max_y) {
                max_y += 1;
            }
            max_y -= 1;

            #[allow(clippy::needless_range_loop)]
            for j in i + 1..coords.len() {
                let (x2, y2) = coords[j];

                if x2 < min_x || x2 > max_x {
                    continue;
                }

                if y2 < min_y || y2 > max_y {
                    continue;
                }

                // now we need to check that everything in the rectangle is inside!
                let area = (x1.max(x2) - x1.min(x2) + 1) * (y1.max(y2) - y2.min(y2) + 1);
                if area > max_area {
                    let mut contains_outside = false;
                    for row in x1.min(x2)..x1.max(x2) {
                        if contains_outside {
                            break;
                        }
                        for col in y1.min(y2)..y1.max(y2) {
                            if outside.contains(row, col) {
                                contains_outside = true;
                                break;
                            }
                        }
                    }
                    if contains_outside {
                        continue;
                    }

                    max_area = area
                }
            }
        }

        Ok(max_area)
    }
}

struct GiantBitmapGrid {
    rows: usize,
    cols: usize,
    data: Vec<u8>,
}

impl GiantBitmapGrid {
    fn new(rows: usize, cols: usize) -> Self {
        let len = ((rows + 1) * cols) / 8 + 1;

        Self {
            rows,
            cols,
            data: vec![0; len],
        }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        self.cols * row + col
    }

    fn get(&self, row: usize, col: usize) -> bool {
        let index = self.index(row, col);
        let bit = index % 8;
        let index = index / 8;
        self.data[index] & (1 << bit) > 0
    }

    fn set(&mut self, row: usize, col: usize) {
        let index = self.index(row, col);
        let bit = index % 8;
        let index = index / 8;
        self.data[index] |= 1 << bit;
    }

    fn fill_range(&mut self, from: (usize, usize), to: (usize, usize)) {
        if from.0 == to.0 {
            let min = from.1.min(to.1);
            let max = from.1.max(to.1);
            for col in min..=max {
                self.set(from.0, col);
            }
        } else if from.1 == to.1 {
            let min = from.0.min(to.0);
            let max = from.0.max(to.0);
            for row in min..max {
                self.set(row, from.1);
            }
        } else {
            panic!("bad coords passed {from:?} {to:?}")
        }
    }
}

struct Outside {
    // coordinates for the rectangle that's cut into the circle
    rect_row_min: usize,
    rect_row_max: usize,
    rect_col_min: usize,
    rect_col_max: usize,

    // grid that contains other coordinates we found are outside perimeter
    grid: GiantBitmapGrid,
}

impl Outside {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            // consts found using excel
            rect_row_min: 2308,
            rect_row_max: 94861,
            rect_col_min: 48447,
            rect_col_max: 50326,

            // bounds found using excel
            grid: GiantBitmapGrid::new(rows, cols),
        }
    }

    fn set(&mut self, row: usize, col: usize) {
        self.grid.set(row, col)
    }

    fn contains(&self, row: usize, col: usize) -> bool {
        // check if it's in the rectangle of hell
        if row >= self.rect_row_min
            && row <= self.rect_row_max
            && col >= self.rect_col_min
            && col <= self.rect_col_max
        {
            return true;
        }

        self.grid.get(row, col)
    }
}
