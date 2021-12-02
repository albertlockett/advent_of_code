use std::str::FromStr;
use std::env;

fn main() {
  println!("welcome to the deep dark depth finder");

  let mut numbers = Vec::new();

  for arg in env::args().skip(1) {
    numbers.push(u64::from_str(&arg).expect("error parding the argument!!"));
  }

  println!("there are {} numbers",numbers.len());

  let mut num_deeper = 0;
  for i in 1..numbers.len() {
    if numbers[i] > numbers[i - 1] {
      num_deeper = num_deeper + 1;
    }
  }
  
  println!("there are {} what are deeper than the last", num_deeper)
}
