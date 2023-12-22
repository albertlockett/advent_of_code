use std::fs::File;
use std::io::prelude::*;

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

fn main() {
    let mut file = File::open("input_test.txt").expect("File not found");
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

    let mut rows = vec![wall];
    rows.append(&mut lines.iter().map(|s| Row::new(&s)).collect::<Vec<Row>>());
    
    
    let iters = 1000000000;

    for i in 0..iters {
        if i % 10000 == 0 {
            println!("{} / {}", i, iters);
        }

        for i in 1..rows.len() {
            let (from_remains, to_rolls) = transfer(&rows[i - 1], &rows[i]);
            rows[i - 1].roll_mask = from_remains;
            rows[i].roll_mask = to_rolls;
        }
        let next_rolls = rotate_right(rows.iter().skip(1).map(|r| r.roll_mask).collect(), width);
        let next_hashes = rotate_right(rows.iter().skip(1).map(|r| r.hash_mask).collect(), width);

        for i in 0..width {
            rows[i + 1].roll_mask = next_rolls[i];
            rows[i + 1].hash_mask = next_hashes[i];
        }
    }

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