use std::{io::Result, ops::Deref};

pub trait Display {
    fn fmt(&self) -> char;
}

pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    fn index(&self, row: usize, col: usize) -> usize {
        self.cols * row + col
    }

    /// will panic if you call it out of bounds
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[self.index(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, item: T) {
        let index = self.index(row, col);
        self.data[index] = item;
    }

    pub fn iter_neighbours(&self, row: usize, col: usize) -> NeighboursIter<'_, T> {
        NeighboursIter::new(self, row, col)
    }
}

impl<T> Grid<Option<T>>
where
    T: From<GridChar>,
{
    /// helper constructor for building this thing from bytes
    pub fn try_from_byte_iter<I: Iterator<Item = Result<u8>>>(iter: I) -> Result<Self> {
        let mut rows = 0;
        let mut data = Vec::new();
        for b in iter {
            match b? {
                b'\n' => {
                    rows += 1;
                }
                b => {
                    let char: GridChar = b.into();
                    data.push(char.into_optional());
                }
            };
        }
        rows += 1;

        let cols = data.len() / rows;
        Ok(Self { cols, rows, data })
    }
}

impl<T> Grid<T>
where
    T: Display,
{
    // used for debugging
    #[allow(dead_code)]
    pub fn display(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{}", self.get(row, col).fmt())
            }
            println!();
        }
    }
}

impl<T> Display for Option<T>
where
    T: Display,
{
    fn fmt(&self) -> char {
        match self {
            None => '.',
            Some(t) => t.fmt(),
        }
    }
}

// thin wrapper around byte to help implement construction and to cheese coherence rules
pub struct GridChar {
    inner: u8,
}

impl Deref for GridChar {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<u8> for GridChar {
    fn from(inner: u8) -> Self {
        Self { inner }
    }
}

impl GridChar {
    fn into_optional<T>(self) -> Option<T>
    where
        T: From<GridChar>,
    {
        match *self {
            b'.' => None,
            _ => Some(T::from(self)),
        }
    }
}

const NEIGHBOUR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), // top left
    (-1, 0),  // top
    (-1, 1),  // top right
    (0, 1),   // right
    (1, 1),   // bottom right
    (1, 0),   // bottom
    (1, -1),  // bottom left
    (0, -1),  // left
];

pub struct NeighboursIter<'a, T> {
    grid: &'a Grid<T>,
    target_row: usize,
    target_col: usize,
    curr_offset: usize,
}

impl<'a, T> NeighboursIter<'a, T> {
    fn new(grid: &'a Grid<T>, row: usize, col: usize) -> Self {
        Self {
            grid,
            target_row: row,
            target_col: col,
            curr_offset: 0,
        }
    }
}

impl<'a, T> Iterator for NeighboursIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.curr_offset >= NEIGHBOUR_OFFSETS.len() {
                return None;
            }

            let (r, c) = &NEIGHBOUR_OFFSETS[self.curr_offset];
            self.curr_offset += 1;

            // TODO we could maybe avoid casting in/out of i32s here would it help perf?
            let row = (self.target_row as i32) + r;
            let col = (self.target_col as i32) + c;

            if row >= 0 && row < self.grid.rows as i32 && col >= 0 && col < self.grid.cols as i32 {
                return Some(self.grid.get(row as usize, col as usize));
            }
        }
    }
}
