use aoc::collections::grid::Grid;
use std::collections::HashSet;

#[derive(Clone, PartialEq)]
enum Position {
    Empty,
    Occupied,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GuardDir {
    N,
    E,
    S,
    W,
}

impl GuardDir {
    fn turn(self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
}

pub fn doit() -> (u32, u32) {
    let input = include_bytes!("../../inputs/day06/real.txt");

    let mut rows = vec![];
    let mut curr_row = vec![];
    let mut pos_x = 0i32;
    let mut pos_y = 0i32;
    for b in input {
        match b {
            b'.' => {
                curr_row.push(Position::Empty);
            }
            b'#' => curr_row.push(Position::Occupied),
            b'^' => {
                pos_x = curr_row.len() as i32;
                pos_y = rows.len() as i32;
                curr_row.push(Position::Empty);
            }
            b'\n' => {
                rows.push(curr_row.clone());
                curr_row.clear()
            }
            _ => {}
        }
    }

    // e.g. if there's no newline at end of file .
    if !curr_row.is_empty() {
        rows.push(curr_row);
    }

    let mut obstacles = Grid::from(rows);
    let mut traversed = Grid::new(obstacles.width(), obstacles.height(), 0u8);
    traversed.set(pos_x as usize, pos_y as usize, 1);
    let mut curr_dir = GuardDir::N;

    let start_x = pos_x;
    let start_y = pos_y;

    loop {
        let (next_x, next_y) = match curr_dir {
            GuardDir::N => (pos_x, pos_y - 1),
            GuardDir::E => (pos_x + 1, pos_y),
            GuardDir::S => (pos_x, pos_y + 1),
            GuardDir::W => (pos_x - 1, pos_y),
        };

        // check if guard went OB
        if next_x < 0 || next_x as usize >= obstacles.width() {
            break;
        }
        if next_y < 0 || next_y as usize >= obstacles.height() {
            break;
        }

        // check if theres an obstacle at the next position
        if *obstacles.get(next_x as usize, next_y as usize).unwrap() == Position::Occupied {
            curr_dir = curr_dir.turn()
        } else {
            pos_x = next_x;
            pos_y = next_y;
            traversed.set(pos_x as usize, pos_y as usize, 1);
        }
    }

    let mut p1 = 0;
    for i in 0..traversed.height() {
        for j in 0..traversed.width() {
            p1 += *traversed.get(j, i).unwrap() as u32;
        }
    }

    // part 2
    let mut p2 = 0;
    let mut curr_check = 1;
    let total_checks = obstacles.height() * obstacles.width();
    for y in 0..obstacles.height() {
        for x in 0..obstacles.width() {
            if x == start_x as usize && y == start_y as usize {
                continue;
            }
            if *obstacles.get(x, y).unwrap() == Position::Occupied {
                continue;
            }

            obstacles.set(x, y, Position::Occupied);
            if does_loop(&obstacles, start_x, start_y, &GuardDir::N) {
                p2 += 1;
            }
            obstacles.set(x, y, Position::Empty);

            curr_check += 1;
            println!("p2 = {}/{}", curr_check, total_checks);
        }
    }

    (p1, p2)
}

fn does_loop(obstacles: &Grid<Position>, pos_x: i32, pos_y: i32, dir: &GuardDir) -> bool {
    let mut viewed_positions = HashSet::<(GuardDir, i32, i32)>::new();
    let mut dir = dir.clone();
    let mut pos_x = pos_x;
    let mut pos_y = pos_y;
    loop {
        let next_entry = (dir.clone(), pos_x, pos_y);
        if viewed_positions.contains(&next_entry) {
            return true;
        }
        viewed_positions.insert(next_entry);

        let (next_x, next_y) = match dir {
            GuardDir::N => (pos_x, pos_y - 1),
            GuardDir::E => (pos_x + 1, pos_y),
            GuardDir::S => (pos_x, pos_y + 1),
            GuardDir::W => (pos_x - 1, pos_y),
        };

        // check if guard went OB
        if next_x < 0 || next_x as usize >= obstacles.width() {
            break;
        }
        if next_y < 0 || next_y as usize >= obstacles.height() {
            break;
        }

        if *obstacles.get(next_x as usize, next_y as usize).unwrap() == Position::Occupied {
            dir = dir.turn()
        } else {
            pos_x = next_x;
            pos_y = next_y;
        }
    }

    false
}

fn debug_grid(
    traversed: &Grid<u8>,
    positions: &Grid<Position>,
    pos_x: i32,
    pos_y: i32,
    dir: &GuardDir,
) {
    let mut debug_grid = Grid::new(traversed.width(), traversed.height(), ".");
    for x in 0..traversed.width() {
        for y in 0..traversed.height() {
            let t = traversed.get(x, y).unwrap();
            let p = positions.get(x, y).unwrap_or(&Position::Empty);
            match (t, p) {
                (1, Position::Occupied) => {
                    panic!("uh ob")
                }
                (0, Position::Occupied) => {
                    debug_grid.set(x, y, "#");
                }
                (1, Position::Empty) => {
                    debug_grid.set(x, y, "X");
                }
                (_, _) => {}
            }

            if x == pos_x as usize && y == pos_y as usize {
                match dir {
                    GuardDir::N => debug_grid.set(x, y, "^"),
                    GuardDir::E => debug_grid.set(x, y, ">"),
                    GuardDir::S => debug_grid.set(x, y, "v"),
                    GuardDir::W => debug_grid.set(x, y, "<"),
                }
            }
        }
    }

    debug_grid.print();
}
