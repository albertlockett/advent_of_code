use std::{error::Error, fmt};
fn main() {
  println!("Hello, world!");
}

/**
 * error for if the board segment already contains field
 */
#[derive(Debug)]
pub struct DuplicateError{}

impl Error for DuplicateError {}

impl fmt::Display for DuplicateError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Oh no, something bad went down")
  }
}

/**
 * One of our segments of the board
 */
#[derive(Copy,Clone)]
pub struct BoardSegment {
  total: u32
}

impl BoardSegment {
  pub fn new() -> BoardSegment {
    BoardSegment {
      total: 1
    }
  }

  fn prime_for_dig(digit: u8) -> u32 {
    match digit {
      1 => 2,
      2 => 3,
      3 => 5,
      4	=> 11,
      5 => 13,
      6 => 17,
      7 => 19,
      8 => 23,
      9 => 29,
      x => {
        println!("unknown digit {}", digit);
        1
      },
    }
  }

  pub fn add(&mut self, c: char) -> Result<(), DuplicateError> {
    match c {
      '.' => Ok(()), // no-op
      n => {
        let digit = n.to_digit(10).unwrap() as u8;
        if self.contains_digit(digit) {
          Err(DuplicateError{})
        } else {
          self.add_digit(digit);
          Ok(())
        }
      }
    }
  }

  fn contains_digit(&self, digit: u8) -> bool {
    self.total % Self::prime_for_dig(digit) == 0
  }

  fn add_digit(&mut self, digit: u8) {
    self.total = self.total * Self::prime_for_dig(digit);
  }
}


pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
  let mut row_segments: [BoardSegment; 9] = [BoardSegment::new(); 9];
  let mut col_segments: [BoardSegment; 9] = [BoardSegment::new(); 9];
  let mut box_segments: [BoardSegment; 9] = [BoardSegment::new(); 9];

  for row_index in 0..board.len() {
    let row = &board[row_index];
    for col_index in 0..row.len() {
      let c = row[col_index];

      let row_segment = &mut row_segments[row_index];
      let col_segment = &mut col_segments[col_index];
        
      let box_index = (row_index / 3) * 3 + col_index / 3;
      let box_segment = &mut box_segments[box_index];
    

      match row_segment.add(c) {
        Ok(_) => {},
        Err(_) => {
          return false;
        }
      };

      match col_segment.add(c) {
        Ok(_) => {},
        Err(_) => {
          return false;
        }
      };

      match box_segment.add(c) {
        Ok(_) => {},
        Err(_) => {
          return false;
        }
      };
    }
  }
  true
}