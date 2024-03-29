use std::fs::File;
use std::hash::Hasher;
use std::io::prelude::*;
use std::num::NonZeroUsize;

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
                    }
                    Some('O') => {
                        curr_section.num_rocks += 1;
                    }
                    Some('.') => {}
                    _ => panic!("Invalid character in input"),
                }
            }
            sections.push((curr_offset, curr_section));

            Column {
                sections,
                length: rows.len() as i16,
            }
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

        Row {
            roll_mask,
            hash_mask,
        }
    }
}

// returns a bitmap of the rolling rocks for from and to after transfer
#[inline(always)]
fn transfer(from_roll_mask: u128, to_hash_mask: u128, to_roll_mask: u128) -> (u128, u128) {
    // where we're blocked from transfering rolls
    let to_mask = to_hash_mask | to_roll_mask;

    // what rolls to next mask
    let transfer_mask = from_roll_mask & !to_mask;

    // what remains after transfer
    let from_remains = from_roll_mask & !transfer_mask;

    return (from_remains, transfer_mask | to_roll_mask);
}

macro_rules! rotate_right {
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
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hasher.write(&vec![dir]);
    for i in 0..rolls.len() {
        hasher.write(&rolls[i].to_le_bytes());
    }
    hasher.finish()
}

fn calculate_weight(mut rolls1: Vec<u128>, width: usize) -> u32 {
    let mut rolls = rolls1.clone();
    rotate_right!(rolls1, rolls, width);
    let mut total_weight = 0;
    for digit in 0..width {
        for row_idx in 0..width {
            let bit = rolls[row_idx] & 1;
            if bit == 1 {
                total_weight += width - digit;
            }
            rolls[row_idx] >>= 1;
        }
    }

    total_weight as u32
}

fn main() {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file");

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

    let mut rows = vec![];
    rows.append(&mut lines.iter().map(|s| Row::new(&s)).collect::<Vec<Row>>());

    // bitmaps for each row: 1 =  the rolling rocks 'O' / the square (hash) rocks '#'
    let mut rolls = rows.iter().map(|r| r.roll_mask).collect::<Vec<u128>>();
    let mut hashes = rows.iter().map(|r| r.hash_mask).collect::<Vec<u128>>();

    // when we rotate the rows we're going to rotate them into these scratch buffers
    // to avoid doing a 2nd allocation
    let mut scratch_rolls = vec![0 as u128; width];
    let mut scratch_hashes = vec![0 as u128; width];

    // the mask for the end of the grid .. all 1s
    let mut wall_mask = 0;
    for _ in 0..width {
        wall_mask <<= 1;
        wall_mask |= 1;
    }

    // debug macro for printing the rows
    macro_rules! _print_rolls {
        () => {
            for row in 0..width {
                let mut chars = vec![];
                let mut roll = rolls[row];
                let mut hash = hashes[row];

                for _ in 0..width {
                    let roll_bit = roll & 1;
                    let hash_bit = hash & 1;
                    roll >>= 1;
                    hash >>= 1;
                    match (roll_bit, hash_bit) {
                        (1, 1) => chars.push('#'),
                        (1, 0) => chars.push('O'),
                        (0, 1) => chars.push('#'),
                        (0, 0) => chars.push('.'),
                        _ => panic!("invalid bit"),
                    }
                }
                println!("{}", chars.iter().rev().collect::<String>());
            }
        };
    }

    // cache for configurations we've seen before
    let mut cache = LruCache::<u64, u64>::new(NonZeroUsize::new(500_000_000).unwrap());

    let iters = 1000000000; // 1 billion
    let mut cycle_break = None;
    for i in 0..iters {
        // rotate all directions
        for j in 0..4 {
            // roll the rocks as far as they can go
            let mut prev_hash = 0;
            let mut curr_hash = hash_rolls(&rolls, j);
            while prev_hash != curr_hash {
                let mut prev_rolls = wall_mask;
                let mut prev_hashes = wall_mask;

                for i in 0..width {
                    let (from_remains, to_rolls) = transfer(rolls[i], prev_hashes, prev_rolls);
                    rolls[i] = from_remains;
                    if i > 0 {
                        rolls[i - 1] = to_rolls;
                    }
                    prev_rolls = rolls[i];
                    prev_hashes = hashes[i];
                }
                prev_hash = curr_hash;
                curr_hash = hash_rolls(&rolls, j);
            }

            // we'll check if we've seen this state before, if so found a loop
            let hash = hash_rolls(&rolls, j);
            let cached_val = cache.get(&hash);
            if cached_val.is_some() && cycle_break.is_none() {
                // figure out the cycle of the loop we'd be on at the end of the iters
                // and then break out of loop at that part of cycle
                let cached_iter = cached_val.unwrap();
                let loop_len = i - cached_iter;
                let remaining_iters = iters - i;
                let loop_iter_at_end = remaining_iters % loop_len;
                cycle_break = Some(i + loop_iter_at_end - 1);
            }
            cache.put(hash, i);

            rotate_right!(rolls, scratch_rolls, width);
            let tmp_rolls = rolls;
            rolls = scratch_rolls;
            scratch_rolls = tmp_rolls;

            rotate_right!(hashes, scratch_hashes, width);
            let tmp_hashes = hashes;
            hashes = scratch_hashes;
            scratch_hashes = tmp_hashes;
        }

        if Some(i) == cycle_break {
            break;
        }
    }

    // calculate weight
    let total_weight = calculate_weight(rolls, width);
    println!("p2 total = {}", total_weight);
}
