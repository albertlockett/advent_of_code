use std::fs::File;
use std::io::prelude::*;

type State = Option<usize>;

struct Symbol {
    start: i16,
    end: i16,
    value: u64,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut line_iter = contents.split("\n").into_iter();
    let first_line = line_iter.next().unwrap();

    let width: usize = first_line.len();
    let height: usize = 1 + line_iter.count();
    println!("width: {}, height: {}", width, height);

    // this just makes the algorithm below easier
    contents = contents + "\n";

    let mut total: u64 = 0;
    let mut state: State = None;
    let mut chars = contents.chars().into_iter();

    let mut number_locations = Vec::<Vec<Symbol>>::new();
    let mut gear_locations = Vec::<Vec<i16>>::new();

    for y in 0..height {
        let mut row_numbers = Vec::<Symbol>::new();
        let mut row_gears = Vec::<i16>::new();
        state = None;

        for x in 0..width + 1 {
            let c = chars.next().unwrap();

            if c == '*' {
                row_gears.push(x as i16);
            }

            if c.is_digit(10) {
                if state.is_none() {
                    state = Some(x);
                }
            } else {
                if state.is_some() {
                    let start = state.unwrap();
                    let start_offset = y * (width + 1) + start;
                    let end_offset = y * (width + 1) + x;
                    let is_part_no = is_part_number(
                        &contents,
                        height,
                        width,
                        state.unwrap(),
                        y,
                        x - state.unwrap(),
                    );

                    println!(
                        "found part number end at ({}, {}): {:?} - {:?}",
                        y,
                        x,
                        &contents[start_offset..end_offset],
                        is_part_no
                    );

                    let value = contents[start_offset..end_offset].parse::<u64>().unwrap();
                    let symbol = Symbol {
                        start: start as i16,
                        end: x as i16,
                        value: value,
                    };
                    row_numbers.push(symbol);

                    if is_part_no {
                        // parse int
                        let part_no = value;
                        total += part_no;
                    }
                    // print!("\x1b[0m");
                    state = None;
                }
            }
        }

        number_locations.push(row_numbers);
        gear_locations.push(row_gears);
    }

    println!("part 1 total: {}", total);

    let mut p2_total: u64 = 0;
    for y in 0..height {
        let row_gears = &gear_locations[y];
        for loc in row_gears {
            let adjacent_numbers = find_adjacent_numbers(&number_locations, *loc, y);
            if adjacent_numbers.len() == 2 {
                let ratio: u64 = adjacent_numbers.iter().product();
                p2_total += ratio;
            }
        }
    }

    println!("part 2 total: {}", p2_total);

    Ok(())
}

fn is_part_number(
    contents: &str,
    height: usize,
    width: usize,
    part_no_x: usize,
    part_no_y: usize,
    no_len: usize,
) -> bool {
    // check top left
    if part_no_x > 0 && part_no_y > 0 {
        let offset = (part_no_y - 1) * (width + 1) + part_no_x - 1;
        let char = contents.chars().nth(offset).unwrap();
        if is_symbol(char) {
            return true;
        };
    }

    // check top right
    if (no_len + part_no_x) < width - 1 && part_no_y > 0 {
        let offset = (part_no_y - 1) * (width + 1) + part_no_x + no_len;
        let char = contents.chars().nth(offset).unwrap();
        if is_symbol(char) {
            return true;
        };
    }

    // check bottom left
    if part_no_x > 0 && part_no_y < height - 1 {
        let offset = (part_no_y + 1) * (width + 1) + part_no_x - 1;
        let char = contents.chars().nth(offset).unwrap();
        if is_symbol(char) {
            return true;
        };
    }

    // check bottom right
    if (part_no_x + no_len) < width - 1 && part_no_y < height - 1 {
        let offset = (part_no_y + 1) * (width + 1) + part_no_x + no_len;
        let char = contents.chars().nth(offset).unwrap();
        if is_symbol(char) {
            return true;
        };
    }

    // check the column to the left
    if part_no_x > 0 {
        let offset = (part_no_y) * (width + 1) + part_no_x - 1;
        let char = contents.chars().nth(offset).unwrap();
        if is_symbol(char) {
            return true;
        };
    }

    // check the column to the right
    if (part_no_x + no_len) < width - 1 {
        let offset = (part_no_y) * (width + 1) + part_no_x + no_len;
        let char = contents.chars().nth(offset).unwrap();
        if is_symbol(char) {
            return true;
        };
    }

    // check the row above
    if part_no_y > 0 {
        for i in 0..no_len {
            let offset = (part_no_y - 1) * (width + 1) + part_no_x + i;
            let char = contents.chars().nth(offset).unwrap();
            if is_symbol(char) {
                return true;
            };
        }
    };

    // check the row below
    if part_no_y < height - 1 {
        for i in 0..no_len {
            let offset = (part_no_y + 1) * (width + 1) + part_no_x + i;
            let char = contents.chars().nth(offset).unwrap();
            if is_symbol(char) {
                return true;
            };
        }
    };

    return false;
}

#[test]
fn test_is_part_no() {
    let contents = "...
.1.
...";
    let height = 4;
    let width = 3;
    let part_no_x = 1;
    let part_no_y = 1;
    let no_len = 1;

    assert_eq!(
        is_part_number(contents, height, width, part_no_x, part_no_y, no_len),
        false
    );

    let contents = "*..
    .1.
    ...";
    assert_eq!(
        is_part_number(contents, height, width, part_no_x, part_no_y, no_len),
        true
    );

    let contents = ".*.
    .1.
    ...";
    assert_eq!(
        is_part_number(contents, height, width, part_no_x, part_no_y, no_len),
        true
    );
}

fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c == '.')
}

fn find_adjacent_numbers(number_locations: &Vec<Vec<Symbol>>, x: i16, y: usize) -> Vec<u64> {
    let mut adjacent_numbers = Vec::<u64>::new();

    // look for numbers above
    if y > 0 {
        let row_numbers = &number_locations[y - 1];
        for number in row_numbers {
            if x >= number.start - 1 && x <= number.end {
                adjacent_numbers.push(number.value);
            }
        }
    }

    // look for numbers below
    if y < number_locations.len() - 1 {
        let row_numbers = &number_locations[y + 1];
        for number in row_numbers {
            if x >= number.start - 1 && x <= number.end {
                adjacent_numbers.push(number.value);
            }
        }
    }

    // look for numbers to the left and right
    let row_numbers = &number_locations[y];
    for number in row_numbers {
        if number.end == x || number.start == x + 1 {
            adjacent_numbers.push(number.value);
        }
    }

    adjacent_numbers
}

#[test]
fn test_find_adjacent_numbers() {
    /*
        0 1 2 3
        -------
    0 | 1 . . 2
    1 | 3 . . 4
     */
    let number_locations = vec![
        vec![
            Symbol {
                start: 0,
                end: 1,
                value: 1,
            },
            Symbol {
                start: 3,
                end: 4,
                value: 2,
            },
        ],
        vec![
            Symbol {
                start: 0,
                end: 1,
                value: 3,
            },
            Symbol {
                start: 3,
                end: 4,
                value: 4,
            },
        ],
    ];

    // check find above
    let result = find_adjacent_numbers(&number_locations, 2, 1);
    assert_eq!(result, vec![1, 2, 4]);

    // check find below
    let result = find_adjacent_numbers(&number_locations, 2, 0);
    assert_eq!(result, vec![3, 4, 2]);
}
