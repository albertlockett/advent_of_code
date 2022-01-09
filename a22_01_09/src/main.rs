fn main() {
  println!("Hello, world!");
}


fn doit(s: String) -> i32 {
  let literals: Vec<&str> = s.split("").collect();

  let mut last_was_i = false;
  let mut last_was_x = false;
  let mut last_was_c = false;

  let mut result = 0;
  for lit in literals {
    match lit {
      "I" => {
        result += 1;
      }
      "V" => {
        result += 5;
        if last_was_i {
          result -= 1 * 2;
        };
      }
      "X" => {
        result += 10;
        if last_was_i {
          result -= 1 * 2;
        };
      }
      "L" => {
        result += 50;
        if last_was_x {
          result -= 10 * 2;
        };
      }
      "C" => {
        result += 100;
        if last_was_x {
          result -= 10 * 2;
        };
      }
      "D" => {
        result += 500;
        if last_was_c {
          result -= 100 * 2;
        }
      }
      "M" => {
        result += 1000;
        if last_was_c {
          result -= 100 * 2;
        }
      }
      _ => {
        // println!("BIFFED IT!!!!");
      }
    }

    last_was_i = false;
    last_was_x = false;
    last_was_c = false;

    match lit {
      "I" => {
        last_was_i = true;
      }
      "X" => {
        last_was_x = true;
      }
      "C" => {
        last_was_c = true;
      }
      _ => {
        // println!("BIFFED IT!!!!");
      }
    }

    // println!("{} {}", lit, result);
  };

  

  result
}

// #[test]
// fn test1() {
//   let x = doit("III".to_string());
//   assert_eq!(x, 3)
// }


// #[test]
// fn test2() {
//   let x = doit("LVIII".to_string());
//   assert_eq!(x, 58)
// }

#[test]
fn test3() {
  let x = doit("MCMXCIV".to_string());
  assert_eq!(x, 1994)
}