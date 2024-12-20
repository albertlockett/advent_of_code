use itertools::Itertools;

type Coord = (i32, i32);

pub fn doit() -> (usize, usize) {
    let input = include_bytes!("../../inputs/day08/real.txt");
    let mut coord = vec![Vec::<Coord>::new(); 26 + 26 + 10];

    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    input.iter().copied().for_each(|b| match b {
        b'A'..b'[' => {
            coord[(b - b'A') as usize].push((x, y));
            x += 1;
        }
        b'a'..b'{' => {
            coord[(b - b'a' + 26) as usize].push((x, y));
            x += 1;
        }
        b'0'..b':' => {
            let index = b - b'0' + 52;
            coord[(index) as usize].push((x, y));
            x += 1;
        }
        b'\n' => {
            max_x = max_x.max(x);
            x = 0;
            y += 1;
        }
        _ => {
            x += 1;
        }
    });
    max_x -= 1;
    let max_y = y - 1;

    let mut antinodes_p1 = Vec::with_capacity(2000);
    let mut antinodes_p2 = Vec::with_capacity(2000);
    coord.iter().for_each(|coords| {
        let mut tmp = find_antinodes(coords, max_x, max_y);
        antinodes_p1.append(&mut tmp);
        let mut tmp = find_antinode_p2(coords, max_x, max_y);
        antinodes_p2.append(&mut tmp);
    });

    (
        antinodes_p1.iter().unique().count(),
        antinodes_p2.iter().unique().count(),
    )
}

#[inline]
fn find_antinodes(coords: &[Coord], max_x: i32, max_y: i32) -> Vec<Coord> {
    let mut antinodes = vec![];

    coords.iter().for_each(|a| {
        coords.iter().for_each(|b| {
            if a.0 == b.0 && a.1 == b.1 {
                return;
            }
            let antinode = find_antinode(a, b);
            if is_valid_coord(&antinode, max_x, max_y) {
                antinodes.push(antinode);
            }
        });
    });

    antinodes
}

#[inline]
fn find_antinode(a: &Coord, b: &Coord) -> Coord {
    let x = a.0 - b.0;
    let y = a.1 - b.1;

    (x + a.0, y + a.1)
}

#[inline]
fn find_antinode_p2(coords: &[Coord], max_x: i32, max_y: i32) -> Vec<Coord> {
    let mut antinodes = vec![];
    if coords.is_empty() {
        return antinodes;
    }

    for i in 0..coords.len() - 1 {
        let a = coords[i];
        for b in coords.iter().skip(i + 1) {
            // let b = coords[j];
            let dx = a.0 - b.0;
            let dy = a.1 - b.1;
            let mut z = (a.0 + dx, a.1 + dy);
            let mut this_pair_antinodes = 0;
            while is_valid_coord(&z, max_x, max_y) {
                antinodes.push(z);
                z.0 += dx;
                z.1 += dy;
                this_pair_antinodes += 1;
            }

            let mut z = (a.0 - dx, a.1 - dy);
            while is_valid_coord(&z, max_x, max_y) {
                antinodes.push(z);
                z.0 -= dx;
                z.1 -= dy;
                this_pair_antinodes += 1;
            }

            if this_pair_antinodes >= 1 {
                antinodes.push(a);
            }
        }
    }

    antinodes
}

#[inline]
fn is_valid_coord(coord: &Coord, max_x: i32, max_y: i32) -> bool {
    !(coord.0 < 0 || coord.1 < 0 || coord.0 > max_x || coord.1 > max_y)
}

// fn print_debug_grid(max_x: i32, max_y: i32, coords: &Vec<Coord>, antinodes: &Vec<Coord>, c: char) {
//     for y in 0..(max_y) {
//         for x in 0..(max_x) {
//             if coords.iter().any(|coord| coord.0 == x && coord.1 == y) {
//                 print!("{}", c);
//                 continue;
//             }
//             if antinodes.iter().any(|coord| coord.0 == x && coord.1 == y) {
//                 print!("#");
//                 continue;
//             }
//             print!(".")
//         }
//         print!("\n")
//     }
//     print!("\n")
// }
