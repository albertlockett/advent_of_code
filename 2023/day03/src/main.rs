use std::fs::File;
use std::io::prelude::*;

type State = Option<usize>;

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
    for y in 0..height {
        // print!("\x1b[0m");
        state = None;
        // print!("\n---\n");
        // print!("\n");

        for x in 0..width+1 {
            let c = chars.next().unwrap();
            // print!("{}", c);
            // print!("x = {}, y = {}, c = {}\n", x, y, c);
            if c.is_digit(10) {
                if state.is_none() {
                    // print!("\x1b[93m");
                    state = Some(x);
                }
            } else {
                if state.is_some() {
                    let start_offset = y * (width + 1) + state.unwrap();
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
                    if is_part_no {
                        // parse int
                        let part_no = &contents[start_offset..end_offset].parse::<u64>().unwrap();;
                        total += part_no;
                    }
                    // print!("\x1b[0m");
                    state = None;
                }
            }
       }
        // chars.next(); // skip newline
    }


    println!("part 1 total: {}", total);

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
