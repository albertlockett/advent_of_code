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
  let mut candidates2 = candidates.clone();
  candidates2.sort();
  combination_sum_inner(&candidates2, target, 0)
}

pub fn combination_sum_inner(candidates:&Vec<i32>, target: i32, offset: usize) -> Vec<Vec<i32>> {
  let mut results: Vec<Vec<i32>> = vec!();

  for i in offset..candidates.len() {
    let next_val = candidates[i];

    if next_val > target {
      break;
    }

    let mut stack = SumStack{
      sum: 0,
      vals: vec!(),
    };

    while stack.sum < target as i64 {
      stack.push(next_val);

      let next_target = target - stack.sum as i32;
      if next_target > 0 {
        let inner_results = combination_sum_inner(&candidates, next_target, i+1);
        for inner_result in inner_results {
          results.push(stack.vals.iter().cloned().chain(inner_result.iter().cloned()).collect());
        }
      }
    }

    if stack.sum == target as i64 {
      results.push(stack.vals.clone());
    }

    stack.pop();
    stack.pop();
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