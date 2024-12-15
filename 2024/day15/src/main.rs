fn main() {
    let input = include_str!("../../inputs/day15/real.txt");

    let mut split = input.split("\n\n");
    let input_1 = split.next().unwrap();
    let input_2 = split.next().unwrap();

    let height = input_1.split("\n").count();
    let width = input_1.len() / height;

    let mut grid = Grid {
        width,
        height,
        vals: input_1.as_bytes().iter().copied().filter(|e| *e != b'\n').collect()
    };

    // find the robot start position
    // TODO if I did a smarter parsing I could avoid this 2nd pass I guess
    let mut pos = None;
    for  x in 0..width as i16 {
        for y in 0..height as i16 {
            if grid.get(x, y).unwrap() == b'@' {
                pos = Some((x, y));
            }
        }
    }
    let mut pos = pos.unwrap();
    for b in input_2.bytes().filter(|i| *i != 10) {
        let dir = Dir::from(b);
        if move_if_possible(pos, &dir, &mut grid) {
            pos = dir.next(pos);
        }

        // println!("\nMove {}:", b as char);
        // grid.print_debug();
    }


    // calcualte GPs score
    let mut p1_total = 0u64;
    for x in 0..width as i16 {
        for y in 0..height as i16 {
            match grid.get(x, y).unwrap() {
                b'O' => {
                    p1_total += (100 * y + x) as u64;
                }
                _ => {}
            }
        }
    }

    println!("p1_total = {}", p1_total);

}

fn move_if_possible(pos: (i16, i16), dir: &Dir, grid: &mut Grid) -> bool {
    let next_pos = dir.next(pos);
    match grid.get(next_pos.0, next_pos.1).unwrap() {
        b'#' => {
            false
        }
        b'.' => {
            let val = grid.get(pos.0, pos.1).unwrap();
            grid.set(next_pos.0, next_pos.1, val);
            grid.set(pos.0, pos.1, b'.');
            true
        }
        b'O' => {
            if move_if_possible(next_pos, dir, grid) {
                let val = grid.get(pos.0, pos.1).unwrap();
                grid.set(next_pos.0, next_pos.1, val);
                grid.set(pos.0, pos.1, b'.');
                true
            } else {
                false
            }
        },
        val => {
            panic!("bad val {} at pos {:?}", val, next_pos);
        }
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn next(&self, pos: (i16, i16)) -> (i16, i16) {
        match self {
            Self::Up => (pos.0, pos.1 - 1),
            Self::Down => (pos.0, pos.1 + 1),
            Self::Left => (pos.0 - 1, pos.1),
            Self::Right => (pos.0 + 1, pos.1),
        }
    }
}

impl From<u8> for Dir {
    fn from(val: u8) -> Self {
        match val {
            b'^' => Self::Up,
            b'<' => Self::Left,
            b'>' => Self::Right,
            b'v' => Self::Down,
            _ => panic!("bad byte {} aka {}", val, val as char)
        }
    }
}

#[derive(Debug)]
struct Grid {
    vals: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    #[inline]
    fn is_ob(&self, x: i16, y: i16) -> bool {
        x < 0 || y < 0 || y as usize >= self.height || x as usize >= self.width
    }

    #[inline]
    fn index(&self, x: i16, y: i16) -> usize {
        self.width * y as usize + x as usize
    }

    #[inline]
    fn get(&self, x: i16, y: i16) -> Option<u8> {
        if self.is_ob(x, y) {
            return None;
        }

        Some(*self.vals.get(self.index(x, y)).unwrap())
    }

    #[inline]
    fn set(&mut self, x: i16, y: i16, val: u8) {
        let index = self.index(x, y);
        self.vals[index] = val;
    }

    fn print_debug(&self) {
        for y in 0..self.height as i16 {
            for x in 0..self.width as i16 {
                print!("{}", self.get(x, y).unwrap() as char);
            }
            print!("\n")
        }
    }
}


