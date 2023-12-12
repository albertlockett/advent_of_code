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

    path_len_grid.set(start_x, start_y, Some(0));

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

    path_len_grid.set(fwd.x, fwd.y, Some(fwd.length));
    path_len_grid.set(rev.x, rev.y, Some(rev.length));

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

    println!("p1 results = {}", traversal_len);


    // P2 smarter way
    let mut expanded_grid = Grid::new(width * 2, vec![Pipe::Ground; (width * 2) * (width * 2)]);
    for y in 0..width {
        for x in 0..width {
            let dist = path_len_grid.get(x, y);
            if dist.is_none() {
                expanded_grid.set(x * 2, y * 2, Pipe::Ground);
                expanded_grid.set(x * 2 + 1, y * 2, Pipe::Ground);
            } else {
                let pipe = grid.get(x, y);
                expanded_grid.set(x * 2, y * 2, pipe.clone());
                match pipe {
                    Pipe::HBar => {
                        expanded_grid.set(x * 2 + 1, y * 2, Pipe::HBar);
                    },
                    Pipe::VBar => {
                        expanded_grid.set(x * 2 , y * 2 + 1, Pipe::VBar);
                    },
                    Pipe::TR => {
                        expanded_grid.set(x * 2, y * 2 + 1, Pipe::VBar);
                    },
                    Pipe::TL => {
                        expanded_grid.set(x * 2 + 1, y * 2, Pipe::HBar);
                        expanded_grid.set(x * 2, y * 2 + 1, Pipe::VBar);
                    },
                    Pipe::BL => {
                        expanded_grid.set(x * 2 + 1, y * 2, Pipe::HBar);
                    },
                    _ => {
                        expanded_grid.set(x * 2 + 1, y * 2, Pipe::Ground);
                    }
                }
            }
        }
    }

    let (fwd_dir, rev_dir) = get_start_dirs(&grid, start_x, start_y);
    for dir in vec![fwd_dir, rev_dir] {
        match dir {
            Dir::Up => {
                expanded_grid.set(start_x * 2, start_y * 2-1, Pipe::VBar);
            },
            Dir::Down => {
                expanded_grid.set(start_x * 2, start_y * 2+1, Pipe::VBar);
            },
            Dir::Left => {
                expanded_grid.set(start_x * 2-1, start_y * 2, Pipe::HBar);
            },
            Dir::Right => {
                expanded_grid.set(start_x * 2+1, start_y * 2, Pipe::HBar);
            }
        }
    }

    let mut queue: Vec<OutsideTraversalNode> = vec![];
    let mut visited_grid: Grid<bool> = Grid::new(expanded_grid.width, vec![false; expanded_grid.width * expanded_grid.width]);

    // initialize the grid with every square around the outside that's not part of the path
    for x in 0..expanded_grid.width {
        let top = expanded_grid.get(x, 0);
        if *top == Pipe::Ground {
            queue.push(OutsideTraversalNode::Outside((x, 0)));
        }

        let bottom = expanded_grid.get(x, expanded_grid.width - 1);
        if *bottom == Pipe::Ground {
            queue.push(OutsideTraversalNode::Outside((x, expanded_grid.width - 1)));
        }
    }
    for y in 0..expanded_grid.width {
        let left = expanded_grid.get(0, y);
        if *left == Pipe::Ground {
            queue.push(OutsideTraversalNode::Outside((0, y)));
        }

        let right = expanded_grid.get(expanded_grid.width - 1, y);
        if *right == Pipe::Ground {
            queue.push(OutsideTraversalNode::Outside((expanded_grid.width - 1, y)));
        }
    }

    let max_coords = (
        expanded_grid.width - 1,
        expanded_grid.data.len() / expanded_grid.width - 1
    );

    while queue.len() > 0 {
        let node = queue.pop().unwrap();

        let candidates = node.next_candidates(&max_coords);

        for (coords, dir) in candidates {
            // check if we've already visited this node
            let visited = visited_grid.get(coords.0, coords.1);
            if true == *visited {
                continue;
            }

            // check if candidate is on the path
            let pipe = expanded_grid.get(coords.0, coords.1);
            if pipe == &Pipe::Ground {
                queue.push(OutsideTraversalNode::Outside(coords));
            }

            // let dist = path_len_grid.get(coords.0, coords.1);
            // if dist.is_some() {
            //     // let pipe = grid.get(coords.0, coords.1);
            //     // queue.push(OutsideTraversalNode::AlongPath(coords, dir, *pipe));
            // } else {
            //     queue.push(OutsideTraversalNode::Outside(coords));
            // }
        }

        match &node {
            // OutsideTraversalNode::AlongPath(coords, _, _) => {
            //     visited_grid.set(coords.0, coords.1, true);
            // },
            OutsideTraversalNode::Outside(coords) => {
                println!("here");
                visited_grid.set(coords.0, coords.1, true);
            }
            _ => {},
        }
    }

    for y in 0..expanded_grid.width {
        for x in 0..expanded_grid.width {
            let pipe = expanded_grid.get(x, y);
            match pipe {
                Pipe::HBar => {
                    print!("-");
                },
                Pipe::VBar => {
                    print!("|");
                },
                Pipe::TR => {
                    print!("7");
                },
                Pipe::BR => {
                    print!("J");
                },
                Pipe::TL => {
                    print!("F");
                },
                Pipe::BL => {
                    print!("L");
                },
                Pipe::Ground => {
                    print!(".");
                },
                Pipe::Start => {
                    print!("S");
                }
            }
        }
        print!("\n");
    }

    print!("\n\n");
    let mut i_count = 0;
    for y in 0..max_coords.1 + 1 {

        for x in 0..max_coords.0 + 1 {
            if y % 2 == 1 || x % 2 == 1 {
                // print!(".");
                continue;
            }
            let visited = visited_grid.get(x, y);
            let dist = expanded_grid.get(x, y);

            if true == *visited {
                if *dist != Pipe::Ground {
                    print!(",");
                } else {
                    print!("O");
                }
            } else {
                if *dist != Pipe::Ground {
                    print!(",");
                } else {
                    i_count += 1;
                    
                    print!("I");
                }
            }
        }
        if y % 2 == 0 {
            // print!("\n");
            continue
        }
        print!("\n");
    }

    println!("i_count = {}", i_count);

    /* 
    // TODO -- assuming here that 0,0 is not on the path. Need to check this

    let curr_node = OutsideTraversalNode::Outside((0,0));
    let max_coords = (grid.width - 1, grid.data.len() / grid.width - 1);
    let mut queue = vec![curr_node];

    let mut visited_grid: Grid<bool> = Grid::new(grid.width, vec![false; grid.width * grid.width]);
    

    // initialize the grid with every square around the outside that's not part of the path
    for x in 0..max_coords.0 + 1 {
        let top = path_len_grid.get(x, 0);
        if top.is_none() {
            queue.push(OutsideTraversalNode::Outside((x, 0)));
        }

        let bottom = path_len_grid.get(x, max_coords.1);
        if bottom.is_none() {
            queue.push(OutsideTraversalNode::Outside((x, max_coords.1)));
        }
    }
    for y in 0..max_coords.1 + 1 {
        let left = path_len_grid.get(0, y);
        if left.is_none() {
            queue.push(OutsideTraversalNode::Outside((0, y)));
        }

        let right = path_len_grid.get(max_coords.0, y);
        if right.is_none() {
            queue.push(OutsideTraversalNode::Outside((max_coords.0, y)));
        }
    }

    while queue.len() > 0 {
        let node = queue.pop().unwrap();

        let candidates = node.next_candidates(&max_coords);

        for (coords, dir) in candidates {
            // check if we've already visited this node
            let visited = visited_grid.get(coords.0, coords.1);
            if true == *visited {
                continue;
            }

            // check if candidate is on the path
            let dist = path_len_grid.get(coords.0, coords.1);
            if dist.is_some() {
                let pipe = grid.get(coords.0, coords.1);
                queue.push(OutsideTraversalNode::AlongPath(coords, dir, *pipe));
            } else {
                queue.push(OutsideTraversalNode::Outside(coords));
            }
        }

        match &node {
            OutsideTraversalNode::AlongPath(coords, _, _) => {
                visited_grid.set(coords.0, coords.1, true);
            },
            OutsideTraversalNode::Outside(coords) => {
                visited_grid.set(coords.0, coords.1, true);
            }
        }
    }

    let mut i_count = 0;
    for y in 0..max_coords.1 + 1 {
        for x in 0..max_coords.0 + 1 {
            let visited = visited_grid.get(x, y);
            let dist = path_len_grid.get(x, y);
            if true == *visited {
                if dist.is_some() {
                    print!(",");
                } else {
                    print!("O");
                }
            } else {
                if dist.is_some() {
                    print!(",");
                } else {
                    i_count += 1;
                    print!("I");
                }
            }
        }
        print!("\n");
    }

    println!("i_count = {}", i_count);
    */

}

// TODO -- this needs tests
fn get_start_dirs(grid: &Grid<Pipe>, start_x: usize, start_y: usize) -> (Dir, Dir) {
    let mut fwd_dir = None;
    let mut rev_dir = None;

    // look for the forward direction
    if start_y > 0 {
        let top = grid.get(start_x, start_y - 1);
        if *top == Pipe::VBar || *top == Pipe::TR || *top == Pipe::TL {
            fwd_dir = Some(Dir::Up);
        }
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
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

struct OutsideTraverser {

}

type Coords = (usize, usize);

enum OutsideTraversalNode {
    AlongPath(Coords, Dir, Pipe),
    Outside(Coords),
}

impl OutsideTraversalNode {
    // dir = dir that is outside the loop
    fn next_candidates(&self, grid_max: &Coords) -> Vec<(Coords, Dir)> {
        macro_rules! push_left_dir {
            ($x: expr, $y: expr, $res: expr, $dir: expr) => {
                if $x > 0 {
                    $res.push((($x - 1, $y), $dir));
                }
            };
        }

        macro_rules! push_right_dir {
            ($x: expr, $y: expr, $res: expr, $dir: expr) => {
                if $x < grid_max.0 {
                    $res.push((($x + 1, $y), $dir));
                }
            };
        }

        macro_rules! push_left {
            ($x: expr, $y: expr, $res: expr) => {
                push_left_dir!($x, $y, $res, Dir::Right);
            };
        }

        macro_rules! push_right {
            ($x: expr, $y: expr, $res: expr) => {
                push_right_dir!($x, $y, $res, Dir::Left);
            };
        }

        macro_rules! push_top_dir {
            ($x: expr, $y: expr, $res: expr, $dir: expr) => {
                if $y > 0 {
                    $res.push((($x, $y - 1), $dir));
                }
            };
        }

        macro_rules! push_top {
            ($x: expr, $y: expr, $res: expr) => {
                push_top_dir!($x, $y, $res, Dir::Down);
            };
        }

        macro_rules! push_bottom_dir {
            ($x: expr, $y: expr, $res: expr, $dir: expr) => {
                if $y < grid_max.1 {
                    $res.push((($x, $y + 1), $dir));
                }
            };
        }

        macro_rules! push_bottom {
            ($x: expr, $y: expr, $res: expr) => {
                push_bottom_dir!($x, $y, $res, Dir::Up);
            };
        }

        match self {
            OutsideTraversalNode::AlongPath(coords, outside_dir, pipe) => {
                let x = coords.0;
                let y = coords.1;

                match pipe {
                    Pipe::HBar => {
                        let mut result = vec![];
                        push_left_dir!(x, y, result, *outside_dir);
                        push_right_dir!(x, y, result, *outside_dir);
                        match outside_dir {
                            Dir::Up => push_top!(x, y, result),
                            Dir::Down => push_bottom!(x, y, result),
                            _ => panic!("invalid dir for hbar {:?} at coord {},{}", outside_dir, x, y),
                        }

                        result
                    }
                    Pipe::VBar => {
                        let mut result = vec![];
                        push_top_dir!(x, y, result, *outside_dir);
                        push_bottom_dir!(x, y, result, *outside_dir);
                        match outside_dir {
                            Dir::Left => push_left!(x, y, result),
                            Dir::Right => push_right!(x, y, result),
                            _ => {},
                        }

                        result
                    }
                    Pipe::TL => {
                        let mut result = vec![];
                        match outside_dir {
                            Dir::Up | Dir::Left => {
                                push_right_dir!(x, y, result, Dir::Up);
                                push_bottom_dir!(x, y, result, Dir::Left);
                            },
                            _ => {},
                        }
                        result
                    }

                    Pipe::TR => {
                        let mut result = vec![];
                        match outside_dir {
                            Dir::Up | Dir::Right => {
                                push_left_dir!(x, y, result, Dir::Up);
                                push_bottom_dir!(x, y, result, Dir::Right);
                            },
                            _ => {},
                        }
                        result
                    }

                    Pipe::BR => {
                        let mut result = vec![];
                        match outside_dir {
                            Dir::Down | Dir::Right => {
                                push_left_dir!(x, y, result, Dir::Down);
                                push_top_dir!(x, y, result, Dir::Right);
                            },
                            _ => {},
                        }
                        result
                    }

                    Pipe::BL => {
                        let mut result = vec![];
                        match outside_dir {
                            Dir::Down | Dir::Left => {
                                push_right_dir!(x, y, result, Dir::Down);
                                push_top_dir!(x, y, result, Dir::Left);
                            },
                            _ => {},
                        }
                        result
                    }
                    _ => vec![]
                }
            },
            OutsideTraversalNode::Outside(coords) => {
                let mut candidates = vec![];
                let x = coords.0;
                let y = coords.1;

                push_left!(x, y, candidates);
                push_right!(x, y, candidates);
                push_top!(x, y, candidates);
                push_bottom!(x, y, candidates);

                candidates
            }
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

    #[test]
    fn test_outside_traversal_node_next_candidates() {
        let grid_max = (2,2);

        // check node in middle of grid
        let node = OutsideTraversalNode::Outside((1,1));
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 4);
        assert_eq!(candidates[0], ((0,1), Dir::Right)); // left
        assert_eq!(candidates[1], ((2,1), Dir::Left));  // right
        assert_eq!(candidates[2], ((1,0), Dir::Down));  // top
        assert_eq!(candidates[3], ((1,2), Dir::Up));    // bottom
        
        // check node at top left corner
        let node = OutsideTraversalNode::Outside((0,0));
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], ((1,0), Dir::Left));  // right
        assert_eq!(candidates[1], ((0,1), Dir::Up));    // bottom
        

        // check node at bottom right corner
        let node = OutsideTraversalNode::Outside((2,2));
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], ((1,2), Dir::Right)); // left
        assert_eq!(candidates[1], ((2,1), Dir::Down));  // top

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Up, Pipe::HBar);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0], ((0,1), Dir::Up)); // left
        assert_eq!(candidates[1], ((2,1), Dir::Up));  // right
        assert_eq!(candidates[2], ((1,0), Dir::Down));  // top

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Down, Pipe::HBar);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0], ((0,1), Dir::Down)); // left
        assert_eq!(candidates[1], ((2,1), Dir::Down));  // right
        assert_eq!(candidates[2], ((1,2), Dir::Up));  // bottom

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Left, Pipe::VBar);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0], ((1,0), Dir::Left)); // top
        assert_eq!(candidates[1], ((1,2), Dir::Left));  // bottom
        assert_eq!(candidates[2], ((0,1), Dir::Right));  // left

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Right, Pipe::VBar);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0], ((1,0), Dir::Right)); // top
        assert_eq!(candidates[1], ((1,2), Dir::Right));  // bottom
        assert_eq!(candidates[2], ((2,1), Dir::Left));  // right

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Left, Pipe::TL);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], ((2,1), Dir::Up)); // right
        assert_eq!(candidates[1], ((1,2), Dir::Left)); // bottom

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Up, Pipe::TL);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], ((2,1), Dir::Up)); // right
        assert_eq!(candidates[1], ((1,2), Dir::Left)); // bottom

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Right, Pipe::TR);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], ((0,1), Dir::Up)); // left
        assert_eq!(candidates[1], ((1,2), Dir::Right)); // bottom

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Up, Pipe::TR);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], ((0,1), Dir::Up)); // right
        assert_eq!(candidates[1], ((1,2), Dir::Right)); // bottom

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Right, Pipe::BR);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], ((0,1), Dir::Down)); // left
        assert_eq!(candidates[1], ((1,0), Dir::Right)); // top

        let node = OutsideTraversalNode::AlongPath((1,1), Dir::Down, Pipe::BR);
        let candidates = node.next_candidates(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], ((0,1), Dir::Down)); // left
        assert_eq!(candidates[1], ((1,0), Dir::Right)); // top
        
    }
}
