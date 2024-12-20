use itertools::Itertools;

pub fn doit() -> (usize, usize) {
    let input = include_bytes!("../../inputs/day10/real.txt");

    // TODO hard-code this
    // TODO should this be done with split or something
    // let height = 1 + input.iter().split('\n').count();
    // let width = input.len() / height;

    let height = 45;
    let width = 45;

    let grid = Grid {
        width,
        height,
        // TODO should this be done with chunks or something
        // vals: input.iter().filter(|b| **b != b'\n').copied().collect(),
        vals: input
            .chunks(width + 1)
            .flat_map(|f| f.iter().take(width))
            .copied()
            .collect(),
    };

    let mut p1_total = 0;
    let mut p2_total = 0;
    for x in 0..width as i16 {
        for y in 0..height as i16 {
            let val = grid.get(x, y).unwrap();
            if val == b'0' {
                let mut heads = vec![(x, y)];
                let mut next_heads = Vec::with_capacity(100);
                let mut curr_z = b'0';
                while curr_z < b'9' {
                    for (x, y) in heads {
                        for (x, y) in
                            vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].into_iter()
                        {
                            if let Some(z) = grid.get(x, y) {
                                if z == curr_z + 1 {
                                    next_heads.push((x, y));
                                }
                            }
                        }
                    }

                    curr_z += 1;

                    heads = next_heads.to_vec();
                    next_heads.clear();
                }

                p1_total += heads.iter().unique().count();
                p2_total += heads.len();
            }
        }
    }
    (p1_total, p2_total)
}

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
}

// struct Traversal {
//     visited
// }
