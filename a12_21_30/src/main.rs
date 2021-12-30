fn main() {
  println!("Hello, world!");
}

#[derive(Debug)]
struct SumStack {
  sum: i64,
  vals: Vec<i32>
}

impl SumStack {
  fn push(&mut self, val: i32)  {
    self.sum = self.sum + val as i64;
    self.vals.push(val);
  }

  fn pop(&mut self) -> Option<i32> {
    match self.vals.pop() {
      Some(v) => {
        self.sum = self.sum - v as i64;
        Some(v)
      },
      None => None,
    }
  }
}

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
  combination_sum_inner(&candidates, target, false, 0)
}

pub fn combination_sum_inner(candidates:&Vec<i32>, target: i32, sorted: bool, offset: usize) -> Vec<Vec<i32>> {
  /*
  // candidates = [2,3,6,7]
  [2]         // push(2), sum = 2
  [2,2]       // push(2), sum = 4
  [2,2,2]     // push(2), sum = 6
  [2,2,2,2]   // push(2), sum = 8, OVER
  [2,2]       // pop(), pop()
  [2,2,3]     // pop(2), push(3), sum = 7 -> emit
  [2,2,3,3]   // push(3), sum = 10 OVER
  [2,2]       // pop(), pop()
  [2,2,6]     // push(6) sum = 10 OVER
  [2]         // pop(), pop()
  [2,7]       // push(7), sum = 9 OVER
  []          // pop(), pop()
  [3]         // push(3)
    // check here is next push greater than target = break
    // check here if nothing left to push and under target, return
  */

  // copy the list of candidates and sort them
  let mut candidates2 = candidates.clone();
  candidates2.sort();

  let mut results: Vec<Vec<i32>> = vec!();

  for i in offset..candidates2.len() {
    let next_val = candidates2[i];

    if next_val > target {
      break;
    }

    let mut stack = SumStack{
      sum: 0,
      vals: vec!(),
    };

    while stack.sum < target as i64 {
      stack.push(next_val);
      // println!("{:?}", stack);

      let next_target = target - stack.sum as i32;
      if next_target > 0 {
        // println!("\ncalling inner with target {}", next_target);
        let inner_results = combination_sum_inner(&candidates2, next_target, true, i+1);
        // println!("inner target {} {:?}\n", next_target, inner_results);
        for inner_result in inner_results {
          let mut result = stack.vals.clone();
          result.append(&mut inner_result.clone());
          // println!("emit {:?}", result);
          results.push(result);
        }
      }
    }

    if stack.sum == target as i64 {
      // println!("emit {:?}", stack);
      results.push(stack.vals.clone());
    }

    stack.pop();
    stack.pop();
    // println!("pop(), pop()");
    // println!("{:?}", stack);
  }

  return results;
}

#[test]
fn test_01() {
  println!("\n");
  let input = vec!(2,3,6,7);
  println!("{:?}", input);
  let result = combination_sum(input, 7);
  println!("{:?}", result);
}

#[test]
fn test_02() {
  println!("\n");
  let input = vec!(2,3,5);
  println!("{:?}", input);
  let result = combination_sum(input, 8);
  println!("{:?}", result);
}

#[test]
fn test_03() {
  println!("\n");
  let input = vec!(2);
  println!("{:?}", input);
  let result = combination_sum(input, 1);
  println!("{:?}", result);
}