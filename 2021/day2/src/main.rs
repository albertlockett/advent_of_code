use std::fs;
use std::str::FromStr;

fn main() {
  println!("now it is day 2 I am in the boat");

  let mut h_pos = 0i64;
  let mut v_pos1 = 0i64;
  let mut v_pos2 = 0i64;
  let mut aim = 0i64;

  // read my instructions from the input
  let instructions = match fs::read_to_string("input.txt") {
    Ok(v) => v,
    Err(e) => {
      eprintln!("Error biffed it reading from the file: {:?}", e);
      std::process::exit(1);
    }
  };


  for line in instructions.split("\n") {
    let mut words = line.split(" ");
    let direction = match words.next() {
      Some(v) => v,
      None => {
        eprintln!("Error line is invalid '{}'", line);
        std::process::exit(1);
      }
    };
    let magnitude = match words.next() {
      Some(v) => i64::from_str(v).expect("error parding the argument!!"),
      None => {
        eprintln!("Error line is invalid '{}'", line);
        std::process::exit(1);
      }
    };

    match direction {
      "forward" => {
        incr(&mut h_pos, magnitude);
        incr(&mut v_pos2, aim * magnitude);
      },
      "up" => {
        incr(&mut v_pos1, -magnitude);
        incr(&mut aim, -magnitude);
      }
      "down" => {
        incr(&mut v_pos1, magnitude);
        incr(&mut aim, magnitude);
      }
      _ => {} // OOPS!
    }
  }

  println!("The final position  caclulated like its part 1 is {} x {} = {}", h_pos, v_pos1, h_pos * v_pos1);
  println!("The final position  caclulated like its part 1 is {} x {} = {}", h_pos, v_pos2, h_pos * v_pos2);
}

fn incr(r: &mut i64, magnitude: i64) {
  *r = *r + magnitude;
}

