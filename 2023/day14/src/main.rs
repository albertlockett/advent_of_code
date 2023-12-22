use std::fs::File;
use std::num::NonZeroUsize;
use std::io::prelude::*;
use std::hash::{Hash, Hasher};

use lru::LruCache;

mod part1 {
    struct Section {
        num_rocks: u32,
    }

    pub struct Column {
        length: i16,
        sections: Vec<(i16, Section)>,
    }

    impl Column {
        pub fn new(rows: &Vec<String>, column_num: usize) -> Self {
            let mut curr_offset: i16 = -1;
            let mut curr_section = Section { num_rocks: 0 };
            let mut sections = Vec::new();

            for (row_num, row) in rows.iter().enumerate() {
                match row.chars().nth(column_num) {
                    Some('#') => {
                        sections.push((curr_offset, curr_section));
                        curr_section = Section { num_rocks: 0 };
                        curr_offset = row_num as i16;
                    },
                    Some('O') => {
                        curr_section.num_rocks += 1;
                    },
                    Some('.') => {},
                    _ => panic!("Invalid character in input"),
                }    
            }
            sections.push((curr_offset, curr_section));

            Column { sections, length: rows.len() as i16 }
        }

        pub fn calc_weight(&self) -> u32 {
            let mut weight = 0;
            for (offset, section) in self.sections.iter() {
                let mut sect_weight = self.length - offset;
                for _ in 0..section.num_rocks {
                    sect_weight -= 1;
                    weight += sect_weight as u32;
                }
            }

            weight
        }
    }
}


struct Row {
    roll_mask: u128,
    hash_mask: u128,
}

impl Row {
    fn new(row: &str) -> Self {
        let mut roll_mask: u128 = 0;
        let mut hash_mask: u128 = 0;
        for (_, c) in row.chars().enumerate() {
            roll_mask <<= 1;
            hash_mask <<= 1;
            if c == '#' {
                hash_mask |= 1;
            }
            if c == 'O' {
                roll_mask |= 1;
            }
        }

        Row { roll_mask, hash_mask }
    }
}

// returns a bitmap of the rolling rocks for from and to after transfer
fn transfer(from: &Row, to: &Row) -> (u128, u128) {
    // where we're blocked from transfering rolls
    let to_mask = to.hash_mask | to.roll_mask; 

    // what rolls to next mask
    let transfer_mask = from.roll_mask & !to_mask;

    // what remains after transfer
    let from_remains = from.roll_mask & !transfer_mask;

    return (from_remains, transfer_mask | to.roll_mask);
}

fn rotate_right(mut rows: Vec<u128>, width: usize) -> Vec<u128> {
    let mut new_rows = vec![0 as u128; width];

    for col in 0..width {
        let mut new_row = 0;
        for i in 0..rows.len() {
            let bit = rows[i] & 1;
            new_row += bit << i;
            rows[i] >>= 1;
        }
        new_rows[width - col - 1] = new_row;
    } 

    new_rows
}

// returns a bitmap of the rolling rocks for from and to after transfer
#[inline(always)]
fn transfer_2(from_roll_mask: u128, to_hash_mask: u128, to_roll_mask: u128) -> (u128, u128) {
    // where we're blocked from transfering rolls
    let to_mask = to_hash_mask | to_roll_mask; 

    // what rolls to next mask
    let transfer_mask = from_roll_mask & !to_mask;

    // what remains after transfer
    let from_remains = from_roll_mask & !transfer_mask;

    return (from_remains, transfer_mask | to_roll_mask);
}

fn rotate_right_2(mut from_rows: &mut Vec<u128>, mut into_rows: &mut Vec<u128>, width: usize) {
    for col in 0..width {
        let mut new_row = 0;
        for i in 0..from_rows.len() {
            let bit = from_rows[i] & 1;
            new_row += bit << i;
            from_rows[i] >>= 1;
        }
        into_rows[width - col - 1] = new_row;
    }
}

macro_rules! rotate_right_3 {
    ($from_rows:ident, $into_rows:ident, $width:ident) => {
        for col in 0..$width {
            let mut new_row = 0;
            for i in 0..$from_rows.len() {
                let bit = $from_rows[i] & 1;
                new_row += bit << i;
                $from_rows[i] >>= 1;
            }
            $into_rows[$width - col - 1] = new_row;
        }
    };
}

fn hash_rolls(rolls: &Vec<u128>, dir: u8) -> u64 {
    let mut hasher =  std::collections::hash_map::DefaultHasher::new();
    hasher.write(&vec![dir]);
    for i in 0..rolls.len() {
        hasher.write(&rolls[i].to_le_bytes());
    }
    hasher.finish()
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");

    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    let width = lines[0].len();

    // part 1
    let mut columns = vec![];
    for i in 0..width {
        columns.push(part1::Column::new(&lines, i));
    }
    let mut p1_total = 0;
    for column in columns.iter() {
        p1_total += column.calc_weight();
    }
    println!("p1 total = {}", p1_total);

    // part 2
    let mut wall_mask = 0;
    for _ in 0..width {
        wall_mask <<= 1;
        wall_mask |= 1;
    }
    let wall = Row{roll_mask: 0, hash_mask: wall_mask};

    let mut rows = vec![];
    rows.append(&mut lines.iter().map(|s| Row::new(&s)).collect::<Vec<Row>>());

    let mut rolls = rows.iter().map(|r| r.roll_mask).collect::<Vec<u128>>();
    let mut hashes = rows.iter().map(|r| r.hash_mask).collect::<Vec<u128>>();

    let mut scratch_rolls = vec![0 as u128; width];
    let mut scratch_hashes = vec![0 as u128; width];
    
    let mut cache = LruCache::<u64, (u8, Vec<u128>)>::new(NonZeroUsize::new(500_000_000).unwrap());

    let iters = 1000000000; // 1 billion
    let start = std::time::Instant::now();


    for i in 0..iters {
        if i % 10000 == 0 {
            let elapsed = start.elapsed().as_millis();
            let iter_per_second = i as f64 / elapsed as f64 * 1000.0;
            let finish_seconds = (iters - i) as f64 / iter_per_second;
            let percent_done = i as f64 / iters as f64 * 100.0;
            println!("{:.5}%: {} / {} iters = {:.5} iter/second  finish in {:.5}s = {:.5}hours", percent_done, i, iters, iter_per_second, finish_seconds, finish_seconds / 3600.0);
        }

        let mut cache_hit = false;

        for j in 0..4 {
            // roll the rocks as far as they can go
            let mut prev_hash = 0;
            let mut curr_hash = hash_rolls(&rolls, j);
            while prev_hash != curr_hash {
                let mut prev_rolls = wall_mask;
                let mut prev_hashes = wall_mask;

                for i in 0..width {
                    let (from_remains, to_rolls) = transfer_2(rolls[i], prev_hashes, prev_rolls);
                    rolls[i] = from_remains;
                    if i > 0 {
                        rolls[i -1] = to_rolls;
                    }
                    prev_rolls = rolls[i];
                    prev_hashes = hashes[i];
                }
                prev_hash = curr_hash;
                curr_hash = hash_rolls(&rolls, j);
            }

            let hash = hash_rolls(&rolls, j);
            let cached_val = cache.get(&hash);
            if cached_val.is_some() {
                println!("found cached value at iter {} dir {}", i, j);
                cache_hit = true;
            }
            cache.put(hash, (j, rolls.clone()));

            rotate_right_3!(rolls, scratch_rolls, width);
            let tmp_rolls = rolls;
            rolls = scratch_rolls;
            scratch_rolls = tmp_rolls;

            rotate_right_3!(hashes, scratch_hashes, width);
            let tmp_hashes = hashes;
            hashes = scratch_hashes;
            scratch_hashes = tmp_hashes;
        }

        if cache_hit {
            break;
        }
    }

    // roll everything back to the top
    let mut prev_hash = 0;
    let mut curr_hash = hash_rolls(&rolls, 0);
    while prev_hash != curr_hash {
        let mut prev_rolls = wall_mask;
        let mut prev_hashes = wall_mask;

        for i in 0..width {
            let (from_remains, to_rolls) = transfer_2(rolls[i], prev_hashes, prev_rolls);
            rolls[i] = from_remains;
            if i > 0 {
                rolls[i -1] = to_rolls;
            }
            prev_rolls = rolls[i];
            prev_hashes = hashes[i];
        }
        prev_hash = curr_hash;
        curr_hash = hash_rolls(&rolls, 0);
    }


    for roll in &rolls {
        println!("{:#012b}", roll);
    }

    // calculate weight
    rotate_right_3!(rolls, scratch_rolls, width);
    let mut total_weight = 0;
    for digit in 0..width {
        for row_idx in 0..width {
            let bit = scratch_rolls[row_idx] & 1;
            if bit == 1 {
                total_weight += width - digit;
            }
            scratch_rolls[row_idx] >>= 1;
        }
    }
    println!("p2 total weight = {}", total_weight);

    println!("done");

}

#[cfg(test)]
mod test {

    #[test]
    fn test_new_row() {
        let row = super::Row::new("..O#O.#");
        assert_eq!(row.roll_mask, 0b010100);
        assert_eq!(row.hash_mask, 0b001001);
    }

    #[test]
    fn test_transfer() {
        let fr = super::Row::new("...OOO###");
        let to = super::Row::new("....#O.#O");
        let expected_from_remain = 0b011000;
        let expected_to_rollssss = 0b101001;
        let (from_remains, to_rolls) = super::transfer(&fr, &to);
        assert_eq!(from_remains, expected_from_remain);
        assert_eq!(to_rolls, expected_to_rollssss);
    }

    #[test]
    fn test_rotate() {
        let rows = vec![
            0b1001,
            0b1100,
            0b1010,
            0b1111,
        ];
        for row in rows.iter() {
            println!("{:b}", row);
        }
        let expected = vec![
            0b1111,
            0b1010,
            0b1100,
            0b1001,
        ];
        let rotated = super::rotate_right(rows.clone(), 4);

        println!("\n");
        for row in rotated.iter() {
            println!("{:b}", row);
        }
        assert_eq!(rotated, expected);
    }
}