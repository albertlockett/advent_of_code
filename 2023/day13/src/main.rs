use std::fs::File;
use std::io::prelude::*;

struct Grid {
    width: u32,
    height: u32,
    data_raw: Vec<String>,
}

impl Grid {
    fn new(full_raw: &str) -> Self {
        let data_raw: Vec<String> = full_raw.split("\n").map(|s| s.to_string()).collect();
        let width = data_raw[0].len() as u32;
        let height = data_raw.len() as u32;
        Grid {
            width,
            height,
            data_raw,
        }
    }

    fn transpose(&mut self) {
        let mut new_data = Vec::<String>::with_capacity(self.width as usize);

        for i in 0..self.width {
            let mut new_row = String::with_capacity(self.height as usize);
            for j in 0..self.height {
                new_row.push(self.data_raw[j as usize].chars().nth(i as usize).unwrap());
            }
            new_data.push(new_row);
        }

        self.data_raw = new_data;
        let tmp = self.width;
        self.width = self.height;
        self.height = tmp;
    }

    fn to_bitmaps(&self) -> Vec<u32> {
        let mut bitmaps: Vec<u32> = Vec::new();
        for (_, row) in self.data_raw.iter().enumerate() {
            let mut bm_row: u32 = 0;
            for (_, c) in row.chars().enumerate() {
                bm_row <<= 1;
                if c == '#' {
                    bm_row |= 1;
                }
            }
            bitmaps.push(bm_row);
        }

        bitmaps
    }

    fn find_reflect(&self) -> Option<usize> {
        let bitmaps = self.to_bitmaps();
        let mut reflect = None;
        for i in 1..(bitmaps.len()) {
            let mut prev: i32 = (i - 1) as i32;
            let mut curr = i;

            if bitmaps[prev as usize] == bitmaps[curr] {
                while prev >= 0 && curr < bitmaps.len() && bitmaps[prev as usize] == bitmaps[curr] {
                    prev -= 1;
                    curr += 1;
                }
                if prev < 0 || curr == bitmaps.len() {
                    reflect = Some(i);
                    break;
                }
            }
        }

        reflect
    }

    fn find_all_reflects(&self) -> Vec<usize> {
        let bitmaps = self.to_bitmaps();
        let mut reflects = Vec::new();
        for i in 1..(bitmaps.len()) {
            let mut prev: i32 = (i - 1) as i32;
            let mut curr = i;

            if bitmaps[prev as usize] == bitmaps[curr] {
                while prev >= 0 && curr < bitmaps.len() && bitmaps[prev as usize] == bitmaps[curr] {
                    prev -= 1;
                    curr += 1;
                }
                if (prev < 0 || curr == bitmaps.len()) {
                    reflects.push(i);
                }
            }
        }
        
        reflects
    }

    fn find_smudges(&mut self) -> bool {
        let orig_reflects = self.find_all_reflects();

        let bitmaps = self.to_bitmaps();
        for i in 0..bitmaps.len() - 1 {
            for j in i + 1..bitmaps.len() {
                let a = bitmaps[i];
                let b = bitmaps[j];
                if let Some(smudge) = smudge_find(a, b) {
                    self.flip_smudge(i, smudge);
                    let new_reflects = self.find_all_reflects();

                    if orig_reflects != new_reflects && new_reflects.len() > 0 {
                        return true;
                    }
                    self.flip_smudge(i, smudge); // flip backs
                }
            }
        }
        return false;
    }

    fn flip_smudge(&mut self, line: usize, offset: usize) {
        let mut row = self.data_raw[line].clone();
        let char_i = row.len() - offset - 1;
        let new_char = match row.chars().nth(char_i).unwrap() {
            '#' => '.',
            '.' => '#',
            _ => panic!("Invalid char"),
        };
        row.replace_range(char_i..char_i + 1, &new_char.to_string());
        self.data_raw[line] = row;
    }

    fn print(&self) {
        let bitmaps = self.to_bitmaps();
        for (i, row) in self.data_raw.iter().enumerate() {
            println!("{}\t{} {}", i, row, bitmaps[i]);
        }
    }
}

fn smudge_find(a: u32, b: u32) -> Option<usize> {
    let mut diff = a ^ b;
    if diff == 0 {
        return None;
    }

    let mut digits = 0;
    while diff > 0 {
        digits += 1;
        diff >>= 1;
    }
    digits -= 1;

    if 1 << digits == a ^ b {
        Some(digits)
    } else {
        None
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");

    let grids = contents
        .split("\n\n")
        .map(|s| Grid::new(s))
        .collect::<Vec<Grid>>();
    println!("{:?}", grids.len());

    let mut p1_total = 0;
    let mut p2_total = 0;
    for mut grid in grids {
        println!("\n\n");
        grid.print();

        // part 1
        grid.transpose();
        let v_reflect = grid.find_reflect();
        if let Some(v_reflect) = v_reflect {
            p1_total += v_reflect as u64;
        }

        grid.transpose();
        let h_reflect = grid.find_reflect();
        if let Some(h_reflect) = h_reflect {
            p1_total += (100 * h_reflect) as u64;
        }

        // reset
        grid.transpose();
        grid.transpose();

        // part 2 ----

        let smudged = grid.find_smudges();
        if !smudged {
            grid.transpose();
            let smudged = grid.find_smudges();
            if !smudged {
                panic!("found no smudes");
            }
            grid.transpose();
        }

        // find all the new reflects after smudges fixed ...

        grid.transpose();
        let v_reflects_2 = grid.find_all_reflects();
        for vr2 in v_reflects_2 {
            if Some(vr2) != v_reflect {
                p2_total += vr2 as u64;
            }
        }

        grid.transpose();
        let h_reflects_2 = grid.find_all_reflects();
        for hr2 in h_reflects_2 {
            if Some(hr2) != h_reflect {
                p2_total += (100 * hr2) as u64;
            }
        }
    }

    println!("Part 1: {}", p1_total);
    println!("Part 2: {}", p2_total);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_grid() {
        let grid = super::Grid::new(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        let reflect = grid.find_reflect();
        assert_eq!(reflect, Some(4));
    }

    #[test]
    fn test_grid2() {
        let mut grid = super::Grid::new(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        let reflect = grid.find_reflect();
        assert_eq!(reflect, None);
        grid.transpose();
        let reflect = grid.find_reflect();
        assert_eq!(reflect, Some(5));
    }

    #[test]
    fn test_find_smudge() {
        let a = 0b101100110;
        let b = 0b001100110;
        let smudge = super::smudge_find(a, b);
        assert_eq!(smudge, Some(8));
    }
}
