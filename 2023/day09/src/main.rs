use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let line_iter = contents.split("\n").into_iter();

    let mut p1_total_results = 0;
    let mut p2_total_results = 0;
    for line in line_iter {
        let nums = line
            .split(" ")
            .into_iter()
            .map(|x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();

        let mut polynomial = Polynomial::new();
        polynomial.expand(nums[0]);
        polynomial.expand(nums[1]);

        for i in 2..nums.len() {
            let last_idx = nums.len() - 1;
            let result = polynomial.eval(last_idx as i128);
            if result == nums[last_idx] {
                break;
            }
            polynomial.expand(nums[i]);
        }

        let p1_result = polynomial.eval(nums.len() as i128);
        p1_total_results += p1_result;

        let p2_result = polynomial.eval(-1);
        p2_total_results += p2_result;
    }

    println!("part 1 result = {}", p1_total_results);
    println!("part 2 result = {}", p2_total_results);
}

struct Polynomial {
    y_vals: Vec<i128>,
    lagrange_seq: Vec<(LagrangeNum, LagrangeDem)>,
}

impl Polynomial {
    fn new() -> Self {
        Polynomial {
            y_vals: Vec::new(),
            lagrange_seq: Vec::new(),
        }
    }

    fn expand(&mut self, y_val: i128) {
        let x_val = self.y_vals.len() as i128;
        for (num, dem) in self.lagrange_seq.iter_mut() {
            num.expand(x_val);
            dem.expand(x_val);
        }
        self.y_vals.push(y_val);
        self.lagrange_seq.push((
            LagrangeNum::new(&self.y_vals, self.y_vals.len() - 1),
            LagrangeDem::new(&self.y_vals, self.y_vals.len() - 1),
        ));
    }

    fn eval(&self, x_val: i128) -> i128 {
        let mut result = 0;
        let mut y_vals_iter = self.y_vals.iter();
        for (num, dem) in self.lagrange_seq.iter() {
            let num_val = num.eval(x_val);
            let dem_val = dem.dem_val;
            result += num_val / dem_val * y_vals_iter.next().unwrap();
        }
        result
    }
}

struct LagrangeNum {
    coefficients: Vec<i128>,
}

impl LagrangeNum {
    fn new(sequence: &Vec<i128>, idx: usize) -> Self {
        let mut result = LagrangeNum {
            coefficients: vec![1],
        };

        for i in 0..sequence.len() {
            if i == idx {
                continue;
            }
            result.expand(i as i128);
        }

        result
    }

    fn expand(&mut self, x_val: i128) {
        let next_const = self.coefficients[self.coefficients.len() - 1] * -x_val;
        let mut i = self.coefficients.len() - 1;
        while i > 0 {
            self.coefficients[i] += self.coefficients[i - 1] * -x_val;
            i -= 1;
        }
        self.coefficients.push(next_const);
    }

    fn eval(&self, x_val: i128) -> i128 {
        let mut result = 0;
        for i in 0..self.coefficients.len() {
            let exp = self.coefficients.len() - i - 1;
            let val = self.coefficients[i] * x_val.pow(exp as u32);
            result += val;
        }
        result
    }
}

struct LagrangeDem {
    seq_idx: i128,
    dem_val: i128,
}

impl LagrangeDem {
    fn new(sequence: &Vec<i128>, idx: usize) -> Self {
        let mut result = LagrangeDem {
            seq_idx: idx as i128,
            dem_val: 1,
        };

        for i in 0..sequence.len() {
            if i == idx {
                continue;
            }
            result.expand(i as i128);
        }

        result
    }

    fn expand(&mut self, x_val: i128) {
        self.dem_val *= self.seq_idx - x_val;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_polynomial() {
        let mut polynomial = crate::Polynomial::new();
        polynomial.expand(1);
        polynomial.expand(3);
        polynomial.expand(6);

        // (x0 - x1)(x0 - x2)
        // (0 - 1)(0 - 2)
        // (-1)(-2)
        // 2
        assert_eq!(polynomial.lagrange_seq[0].1.dem_val, 2);

        // (x1 - x0)(x1 - x2)
        // (1 - 0)(1 - 2)
        // (1)(-1)
        // -1
        assert_eq!(polynomial.lagrange_seq[1].1.dem_val, -1);

        // (x2 - x0)(x2 - x1)
        // (2 - 0)(2 - 1)
        // (2)(1)
        // 2
        assert_eq!(polynomial.lagrange_seq[2].1.dem_val, 2);

        // y = (x - 1)(x - 2)
        // y = x^2 - 3x + 2
        assert_eq!(polynomial.lagrange_seq[0].0.coefficients, vec![1, -3, 2]);

        // y = (x - 0)(x - 2)
        // y = x^2 - 2x
        assert_eq!(polynomial.lagrange_seq[1].0.coefficients, vec![1, -2, 0]);

        // y = (x - 0)(x - 1)
        // y = x^2 - x
        assert_eq!(polynomial.lagrange_seq[2].0.coefficients, vec![1, -1, 0]);

        assert_eq!(polynomial.eval(0), 1);
        assert_eq!(polynomial.eval(1), 3);
        assert_eq!(polynomial.eval(2), 6);
        assert_eq!(polynomial.eval(3), 10);
        assert_eq!(polynomial.eval(4), 15);
    }

    #[test]
    fn test_lagrange_num_expand() {
        let mut lagrange_num = crate::LagrangeNum {
            coefficients: vec![1],
        };
        // y = (x - 1) = -1 + x
        lagrange_num.expand(1);
        assert_eq!(lagrange_num.coefficients, vec![1, -1]);

        // // y = (x - 1)(x - 2) = 2 - 3x + x^2
        lagrange_num.expand(2);
        assert_eq!(lagrange_num.coefficients, vec![1, -3, 2]);

        // y = (x - 1)(x - 2)(x - 3)
        //   = (x^2 -3x + 2)(x - 3)
        //   = (x^3 - 3x^2 + 2x) - 3(x^2 - 3x + 2)
        //   = x^3 - 3x^2 + 2x - 3x^2 + 9x - 6
        //   = x^3 - 6x^2 + 11x - 6
        lagrange_num.expand(3);
        assert_eq!(lagrange_num.coefficients, vec![1, -6, 11, -6]);
    }

    #[test]
    fn test_lagrange_num_eval() {
        let lagrange_num = crate::LagrangeNum {
            coefficients: vec![1, 2, 3, 4],
        };

        // y = x^3 + 2x^2 + 3x + 4
        //   = 4 + 3 + 2 + 1
        assert_eq!(lagrange_num.eval(0), 4);

        // y = 1^3 + 2*1^2 + 3*1 + 4
        assert_eq!(lagrange_num.eval(1), 10);

        // y = 2^3 + 2*2^2 + 3*2 + 4
        //   = 8 + 8 + 6 + 4
        //   = 26
        assert_eq!(lagrange_num.eval(2), 26);
    }

    #[test]
    fn test_langrange_dem_expand() {
        let mut lagrange_dem = crate::LagrangeDem {
            seq_idx: 0,
            dem_val: 1,
        };
        // (0 - 1)
        lagrange_dem.expand(1);
        assert_eq!(lagrange_dem.dem_val, -1);

        // (0 - 1)(0 - 2)
        // (-1)(-2)
        // 2
        lagrange_dem.expand(2);
        assert_eq!(lagrange_dem.dem_val, 2);

        // (0 - 1)(0 - 2)(0 - 3)
        // (2)(-3)
        // -6
        lagrange_dem.expand(3);
        assert_eq!(lagrange_dem.dem_val, -6);
    }
}
