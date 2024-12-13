use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../inputs/day12/real.txt");

    let height = input.split('\n').count();
    let width = input.len() / height;

    let grid = Grid {
        width,
        height,
        vals: input
            .as_bytes()
            .chunks(width + 1)
            .flat_map(|f| f.iter().take(width))
            .copied()
            .collect(),
    };

    let mut in_sector = BTreeSet::<(i16, i16)>::new();
    let mut sectors = vec![];

    for x in 0..width as i16 {
        for y in 0..height as i16 {
            if in_sector.contains(&(x, y)) {
                continue;
            }

            let sector = find_sector(x, y, &grid);
            for (x, y) in sector.iter() {
                in_sector.insert((*x, *y));
            }
            sectors.push(sector);
        }
    }

    let mut p1_total = 0;
    let mut p2_total = 0;
    for sector in sectors {
        let perimeter = find_perimeter(&sector);
        let area = sector.len();
        p1_total += perimeter * area;

        let sides = count_sides(&sector);
        println!("{:?} p = {}, s = {}", sector, perimeter, sides);
        p2_total += sides * area;
    }

    println!("p1_total = {}", p1_total);
    println!("p2_total = {}", p2_total);
}

fn find_sector(x: i16, y: i16, grid: &Grid) -> BTreeSet<(i16, i16)> {
    let target = grid.get(x, y).unwrap();
    let mut curr = vec![(x, y)];
    let mut sector = BTreeSet::<(i16, i16)>::new();
    sector.insert((x, y));

    while curr.len() != 0 {
        let mut next = vec![];

        for (x, y) in curr {
            if let Some(candidate) = grid.get(x - 1, y) {
                if candidate == target && !sector.contains(&(x - 1, y)) {
                    next.push((x - 1, y));
                    sector.insert((x - 1, y));
                }
            }

            if let Some(candidate) = grid.get(x + 1, y) {
                if candidate == target && !sector.contains(&(x + 1, y)) {
                    next.push((x + 1, y));
                    sector.insert((x + 1, y));
                }
            }

            if let Some(candidate) = grid.get(x, y - 1) {
                if candidate == target && !sector.contains(&(x, y - 1)) {
                    next.push((x, y - 1));
                    sector.insert((x, y - 1));
                }
            }

            if let Some(candidate) = grid.get(x, y + 1) {
                if candidate == target && !sector.contains(&(x, y + 1)) {
                    next.push((x, y + 1));
                    sector.insert((x, y + 1));
                }
            }
        }

        curr = next;
    }

    sector
}

fn find_perimeter(sector: &BTreeSet<(i16, i16)>) -> usize {
    let mut perimeter = 0;
    for (x, y) in sector.iter().copied() {
        let l = (x - 1, y);
        let r = (x + 1, y);
        let u = (x, y - 1);
        let d = (x, y + 1);

        for coord in vec![l, r, u, d].iter() {
            if !sector.contains(coord) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn count_sides(sector: &BTreeSet<(i16, i16)>) -> usize {
    let mut lefts = vec![];
    let mut rights = vec![];
    let mut tops = vec![];
    let mut bottoms = vec![];

    for (x, y) in sector.iter().copied() {
        let l = (x - 1, y);
        let r = (x + 1, y);
        let u = (x, y - 1);
        let d = (x, y + 1);

        if !sector.contains(&l) {
            lefts.push(l);
        }

        if !sector.contains(&r) {
            rights.push(r);
        }

        if !sector.contains(&u) {
            tops.push(u);
        }

        if !sector.contains(&d) {
            bottoms.push(d);
        }
    }

    let t_sides = count_contigous_hotizontally(tops);
    let b_sides = count_contigous_hotizontally(bottoms);
    let l_sides = count_contigous_vertically(lefts);
    let r_sides = count_contigous_vertically(rights);

    t_sides + b_sides + l_sides + r_sides
}

fn count_contigous_hotizontally(coords: Vec<(i16, i16)>) -> usize {
    let mut sides: Vec<Vec<(i16, i16)>> = Vec::new();

    for coord in coords {
        let mut found: Option<usize> = None;
        for i in 0..sides.len() {
            let side = sides.get(i).unwrap();
            for neighbour in side {
                if (neighbour.0 - 1 == coord.0 || neighbour.0 + 1 == coord.0)
                    && neighbour.1 == coord.1
                {
                    found = Some(i);
                }
            }

            if found.is_some() {
                break;
            }
        }

        if let Some(i) = found {
            sides.get_mut(i).unwrap().push(coord);
        } else {
            sides.push(vec![coord]);
        };
    }

    sides.len()
}

fn count_contigous_vertically(coords: Vec<(i16, i16)>) -> usize {
    let mut sides: Vec<Vec<(i16, i16)>> = Vec::new();

    for coord in coords {
        let mut found: Option<usize> = None;
        for i in 0..sides.len() {
            let side = sides.get(i).unwrap();
            for neighbour in side {
                if (neighbour.1 - 1 == coord.1 || neighbour.1 + 1 == coord.1)
                    && neighbour.0 == coord.0
                {
                    found = Some(i);
                }
            }

            if found.is_some() {
                break;
            }
        }

        if let Some(i) = found {
            sides.get_mut(i).unwrap().push(coord);
        } else {
            sides.push(vec![coord]);
        };
    }

    sides.len()
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
}
