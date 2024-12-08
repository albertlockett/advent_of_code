use std::collections::HashSet;

type Coord = (i32, i32);

fn main() {
    let input = include_bytes!("../../inputs/day08/real.txt");
    // let empty = 
    let mut coord = vec![Vec::<Coord>::new(); 26 + 26 + 10];

    let mut x = 0;
    let mut y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    input.into_iter().copied().for_each(|b| {
        match b {
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
                println!("{}", index);
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
        }
    });
    max_x -= 1;
    max_y = y - 1;

    // for debugging
    let mut chars = vec![];
    for i in 'A'..'[' {
        chars.push(i);
    }
    for i in 'a'..'{' {
        chars.push(i);
    }
    for i in '0'..':' {
        chars.push(i);
    }

    let mut antinodes = HashSet::new();
    // let mut antinodes = Vec::new();
    coord.iter().enumerate().for_each(|(i, coords)| {
        if coords.len() == 0 {
            return
        }
        let c = chars[i];
        if c == '0' {
            println!("{:?}", coords);
            println!("HERES THE BADD EGG");
        }
        let tmp = find_antinodes(coords, max_x, max_y);
        println!("{}", tmp.len());
        // if tmp.len() != 2 {
        //     panic!("bad number");
        // }
        print_debug_grid(max_x, y, coords, &tmp, chars[i]);
        tmp.into_iter().for_each(|coord| {
            println!("{:?}", coord);
            let x = antinodes.insert(coord);
            // if !x {
            //     panic!("bad insert")
            // }
        });
    });

    // 293 = correct
    // 297 = too high
    // 305 = not right
    // 325 = too high
    // 339 = too high
    println!("p1 = {}", antinodes.len());
}

fn find_antinodes(coords: &Vec<Coord>, max_x: i32, max_y: i32) -> Vec<Coord> {
    let mut antinodes = vec![];

    coords.iter().for_each(|a| {
        coords.iter().for_each(|b| {
            if a.0 == b.0 && a.1 == b.1 {
                return
            }
            let antinode = find_antinode(a, b);
            if is_valid_coord(&antinode, max_x, max_y) {
                antinodes.push(antinode);
            }
        });
    });

    antinodes
}

fn find_antinode(a: &Coord, b: &Coord) -> Coord {
    let x = a.0 - b.0;
    let y = a.1 - b.1;
    let z = (x + a.0, y + a.1);

    let da = (((z.0 - a.0).pow(2) + (z.1 - a.1).pow(2)) as f64).sqrt();
    let db = (((z.0 - b.0).pow(2) + (z.1 - b.1).pow(2)) as f64).sqrt();

    if (db / da) != 2.0 {
        panic!("bad calc");
    }

    return z
}

fn is_valid_coord(coord: &Coord, max_x: i32, max_y: i32) -> bool {
    if coord.0 < 0 || coord.1 < 0 || coord.0 > max_x || coord.1 > max_y {
        false
    } else {
        true
    }
}


fn print_debug_grid(
    max_x: i32,
    max_y: i32,
    coords: &Vec<Coord>,
    antinodes: &Vec<Coord>,
    c: char,
) {
    for y in 0..(max_y)  {
        for x in 0..(max_x) {
            if coords.iter().any(|coord| coord.0 == x && coord.1 == y) {
                print!("{}", c);
                continue;
            }
            if antinodes.iter().any(|coord| coord.0 == x && coord.1 == y) {
                print!("#");
                continue;
            }
            print!(".")
        }
        print!("\n")
    }
    print!("\n")
}