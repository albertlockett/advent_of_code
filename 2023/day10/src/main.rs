use std::fs::File;
use std::io::prelude::*;

fn main() { 
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // ****************************************************************************
    // PART 1
    // ****************************************************************************

    // create data structure that holds grid of characters
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

    // create data structure that holds the length of the path to each location
    let mut path_len_grid: Grid<Option<usize>> = Grid::new(width, vec![None; width * width]);

    // find the start location and directions ...
    let start_y = start_idx / width;
    let start_x = start_idx % width;
    let (fwd_dir, rev_dir) = get_start_dirs(&grid, start_x, start_y);
    path_len_grid.set(start_x, start_y, Some(0));

    // we traverse around the loop in fwd & backward directions 
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

    let traversal_len: usize;

    loop {
        // get pipe at location
        let pipe = grid.get(fwd.x, fwd.y);

        // advance forward traversal for the direction of pipe
        fwd.dir = pipe.next_dir(fwd.dir);
        fwd.advance();

        // check if we found a location the reverse traversal passed over
        if path_len_grid.get(fwd.x, fwd.y).is_some() {
            traversal_len = fwd.length;
            break;
        }
        path_len_grid.set(fwd.x, fwd.y, Some(fwd.length));

        // do the same thing for reverse traversal
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


    // ****************************************************************************
    // PART 2
    // ****************************************************************************

    // for this we're going to just traverse over all the locations that are attached to
    // the side of the grid that aren't on the path, and then iterate to each neighbour
    // that's not on the path, and so on, until we've visited all the locations that
    // reachable. 
    //
    // to make it so neighbours that are separated by a pipe are reachable, we're going
    // to expand the grid by a factor of 2, and then add ground squares in between 
    // adjacent path edges.. then we'll reconnect the path edges in the extra rows/cols. 

    // make the expanded grid ...
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
                
                // connect the pipes in the expanded rows 
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

    // the start location also needs to have it's edges connected to the path
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

    // here we're going to keep track of the locations we've visited while traversing
    // the expanded grid
    let mut visited_grid: Grid<bool> = Grid::new(expanded_grid.width, vec![false; expanded_grid.width * expanded_grid.width]);

    // initialize the grid with every square around the outside that's not part of the path
    let mut queue: Vec<OutsideTraversalNode> = vec![];
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

    // keep track of the edges of the grid ...
    let max_coords = (
        expanded_grid.width - 1,
        expanded_grid.data.len() / expanded_grid.width - 1
    );

    while queue.len() > 0 {
        let node = queue.pop().unwrap();

        let neighbours = node.neighbours(&max_coords);

        for coords in neighbours {
            // check if we've already visited this node
            let visited = visited_grid.get(coords.0, coords.1);
            if true == *visited {
                continue;
            }

            // check if node is on the path
            let pipe = expanded_grid.get(coords.0, coords.1);
            if pipe == &Pipe::Ground {
                queue.push(OutsideTraversalNode::Outside(coords));
            }
        }

        match &node {
            OutsideTraversalNode::Outside(coords) => {
                visited_grid.set(coords.0, coords.1, true);
            }
        }
    }

    // print expanded grid for debugging ...
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
    // print inside/outside grid for debugging ...
    let mut i_count = 0;
    for y in 0..max_coords.1 + 1 {

        for x in 0..max_coords.0 + 1 {
            if y % 2 == 1 || x % 2 == 1 {
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
            continue
        }
        print!("\n");
    }

    println!("i_count = {}", i_count); // part 2 result here <--
}

// TODO -- this needs tests
fn get_start_dirs(grid: &Grid<Pipe>, start_x: usize, start_y: usize) -> (Dir, Dir) {
    let mut fwd_dir = None;
    let mut rev_dir = None;

    // look for the forward direction
    if start_y > 0 {
        // TODO ^^^ need to add this check of all sides ..., not just have hard-coded for
        // the few test cases 
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
        // TODO ^^^ need to add this check of all sides ..., not just have hard-coded for
        // the few test cases 
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


type Coords = (usize, usize);

// type shenanigans so can have methods that manipulate coords ...
enum OutsideTraversalNode {
    Outside(Coords),
}

impl OutsideTraversalNode {
    fn neighbours(&self, grid_max: &Coords) -> Vec<Coords> {
        // originally I had many many cases where needed to add a neighbour to the list
        // so I made some macros to make it easier to add neighbours, but then I 
        // refactored and ended up not needing them, but didn't wanna pull these out

        macro_rules! push_left {
            ($x: expr, $y: expr, $res: expr) => {
                if $x > 0 {
                    $res.push(($x - 1, $y));
                }
            };
        }

        macro_rules! push_right {
            ($x: expr, $y: expr, $res: expr) => {
                if $x < grid_max.0 {
                    $res.push(($x + 1, $y));
                }
            };
        }

        macro_rules! push_top {
            ($x: expr, $y: expr, $res: expr) => {
                if $y > 0 {
                    $res.push(($x, $y - 1));
                }
            };
        }

        macro_rules! push_bottom {
            ($x: expr, $y: expr, $res: expr) => {
                if $y < grid_max.1 {
                    $res.push(($x, $y + 1));
                }
            };
        }

        match self {
            OutsideTraversalNode::Outside(coords) => {
                let mut candidates = vec![];
                let x = coords.0;
                let y = coords.1;

                push_left!(x, y, candidates);
                push_right!(x, y, candidates);
                push_top!(x, y, candidates);
                push_bottom!(x, y, candidates);

                candidates
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_outside_traversal_node_next_candidates() {
        let grid_max = (2,2);

        // check node in middle of grid
        let node = OutsideTraversalNode::Outside((1,1));
        let candidates = node.neighbours(&grid_max);
        assert_eq!(candidates.len(), 4);
        assert_eq!(candidates[0], (0,1)); // left
        assert_eq!(candidates[1], (2,1));  // right
        assert_eq!(candidates[2], (1,0));  // top
        assert_eq!(candidates[3], (1,2));    // bottom
        
        // check node at top left corner
        let node = OutsideTraversalNode::Outside((0,0));
        let candidates = node.neighbours(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], (1,0));  // right
        assert_eq!(candidates[1], (0,1));    // bottom
        
        // check node at bottom right corner
        let node = OutsideTraversalNode::Outside((2,2));
        let candidates = node.neighbours(&grid_max);
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0], (1,2)); // left
        assert_eq!(candidates[1], (2,1));  // top
    }
}
