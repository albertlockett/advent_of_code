use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let first_line = contents.split("\n").into_iter().take(1).next().unwrap();
    let width: usize = first_line.len();

    let mut idx: usize = 0;
    let mut start_idx: usize = 0;
    let grid = Grid::new(
        width,
        contents.chars().filter_map(|c| {
            idx += 1;
            match c {
                '\n' => {
                    idx -= 1;
                    None
                }
                'S' => {
                    start_idx = idx - 1;
                    Some(Pipe::new(c))
                }
                _ => Some(Pipe::new(c)),
            }
        }),
    );

    let mut path_len_grid: Grid<Option<usize>> = Grid::new(width, vec![None; width * width]);

    let start_y = start_idx / width;
    let start_x = start_idx % width;

    let (fwd_dir, rev_dir) = get_start_dirs(&grid, start_x, start_y);

    let mut fwd = Traversal {
        x: start_x,
        y: start_y,
        dir: fwd_dir,
        length: 0,
    };

    let mut rev = Traversal {
        x: start_x,
        y: start_y,
        dir: rev_dir,
        length: 0,
    };

    fwd.advance();
    rev.advance();

    let mut traversal_len: usize = 0;

    loop {
        // get pipe at location
        let pipe = grid.get(fwd.x, fwd.y);

        // advance forward traversal
        fwd.dir = pipe.next_dir(fwd.dir);
        fwd.advance();

        // check if we found a location the reverse traversal passed over
        if path_len_grid.get(fwd.x, fwd.y).is_some() {
            traversal_len = fwd.length;
            break;
        }
        path_len_grid.set(fwd.x, fwd.y, Some(fwd.length));

        // do the reverse traversal
        let pipe = grid.get(rev.x, rev.y);
        rev.dir = pipe.next_dir(rev.dir);
        rev.advance();

        // check if we found a location the reverse traversal passed over
        let dist = path_len_grid.get(rev.x, rev.y);
        if dist.is_some() {
            traversal_len = dist.unwrap();
            break;
        }
        path_len_grid.set(rev.x, rev.y, Some(rev.length));
    }

    println!("p1 results = {}", traversal_len)
}

// TODO -- this needs tests
fn get_start_dirs(grid: &Grid<Pipe>, start_x: usize, start_y: usize) -> (Dir, Dir) {
    let mut fwd_dir = None;
    let mut rev_dir = None;

    // look for the forward direction
    let top = grid.get(start_x, start_y - 1);
    if *top == Pipe::VBar || *top == Pipe::TR || *top == Pipe::TL {
        fwd_dir = Some(Dir::Up);
    }
    if fwd_dir.is_none() {
        let right = grid.get(start_x + 1, start_y);
        if *right == Pipe::HBar || *right == Pipe::TR || *right == Pipe::BR {
            fwd_dir = Some(Dir::Right);
        }
    }
    if fwd_dir.is_none() {
        let bottom = grid.get(start_x, start_y + 1);
        if *bottom == Pipe::VBar || *bottom == Pipe::BR || *bottom == Pipe::BL {
            fwd_dir = Some(Dir::Down);
        }
    }
    if fwd_dir.is_none() {
        let left = grid.get(start_x - 1, start_y);
        if *left == Pipe::HBar || *left == Pipe::TL || *left == Pipe::BL {
            fwd_dir = Some(Dir::Left);
        }
    }

    // look for the reverse direction
    if start_x > 0 {
        // TODO need to add this check of all sides ...
        let left = grid.get(start_x - 1, start_y);
        if *left == Pipe::HBar || *left == Pipe::TL || *left == Pipe::BL {
            rev_dir = Some(Dir::Left);
        }
    }
    if rev_dir.is_none() {
        let bottom = grid.get(start_x, start_y + 1);
        if *bottom == Pipe::VBar || *bottom == Pipe::BR || *bottom == Pipe::BL {
            rev_dir = Some(Dir::Down);
        }
    }
    if rev_dir.is_none() {
        let right = grid.get(start_x + 1, start_y);
        if *right == Pipe::HBar || *right == Pipe::TR || *right == Pipe::BR {
            rev_dir = Some(Dir::Right);
        }
    }
    if rev_dir.is_none() {
        let top = grid.get(start_x, start_y - 1);
        if *top == Pipe::VBar || *top == Pipe::TR || *top == Pipe::TL {
            rev_dir = Some(Dir::Up);
        }
    }

    (fwd_dir.unwrap(), rev_dir.unwrap())
}

struct Traversal {
    x: usize,
    y: usize,
    dir: Dir,
    length: usize,
}

impl Traversal {
    fn advance(&mut self) {
        self.length += 1;
        match self.dir {
            Dir::Up => self.y -= 1,
            Dir::Down => self.y += 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        }
    }
}

struct Grid<T> {
    width: usize,
    data: Vec<T>,
}

macro_rules! offset {
    ($x:expr, $y:expr, $width:expr) => {
        $y * $width + $x
    };
}

impl<T> Grid<T> {
    fn new<I>(width: usize, vals: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let data = vals.into_iter().collect();
        Grid { width, data }
    }

    fn get(&self, x: usize, y: usize) -> &T {
        &self.data[offset!(x, y, self.width)]
    }

    fn set(&mut self, x: usize, y: usize, val: T) {
        self.data[offset!(x, y, self.width)] = val;
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq)]
enum Pipe {
    HBar,
    VBar,
    TR,
    BR,
    TL,
    BL,
    Ground,
    Start,
}

impl Pipe {
    fn new(c: char) -> Self {
        match c {
            '|' => Self::VBar,
            '-' => Self::HBar,
            'F' => Self::TL,
            'L' => Self::BL,
            '7' => Self::TR,
            'J' => Self::BR,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("invalid char {}", c),
        }
    }

    fn next_dir(&self, curr_dir: Dir) -> Dir {
        match self {
            Pipe::HBar => match curr_dir {
                Dir::Left => Dir::Left,
                Dir::Right => Dir::Right,
                _ => panic!("invalid dir"),
            },
            Pipe::VBar => match curr_dir {
                Dir::Up => Dir::Up,
                Dir::Down => Dir::Down,
                _ => panic!("invalid dir"),
            },
            Pipe::TR => match curr_dir {
                Dir::Up => Dir::Left,
                Dir::Right => Dir::Down,
                _ => panic!("invalid dir"),
            },
            Pipe::TL => match curr_dir {
                Dir::Up => Dir::Right,
                Dir::Left => Dir::Down,
                _ => panic!("invalid dir"),
            },
            Pipe::BR => match curr_dir {
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Up,
                _ => panic!("invalid dir"),
            },
            Pipe::BL => match curr_dir {
                Dir::Down => Dir::Right,
                Dir::Left => Dir::Up,
                _ => panic!("invalid dir"),
            },
            _ => panic!("invalid dir"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_grid() {
        let values = vec![1, 2, 3, 4, 5, 6];
        let grid = Grid::new(3, values);
    }
}
