

#[derive(Debug)]
enum Operation {
    Add,
    Mult,
    Concat,
}

struct OpSeqIter {
    len: usize,
    curr: u16,
    concat_curr: u16,
}

impl OpSeqIter {
    fn new(len: usize) -> Self {
        Self { len, curr: 0, concat_curr: 0 }
    }
}

// TODO this emits a an extra full concat sequence too many times
impl Iterator for OpSeqIter {
    type Item = Vec<Operation>;

    fn next(&mut self) -> Option<Vec<Operation>> {
        if self.concat_curr >= (1 << self.len) {
            self.concat_curr = 0;
            self.curr += 1;
        }

        if self.curr >= (1 << self.len) {
            return None
        }

        let mut seq = vec![];
        for i in 0..self.len {
            if self.curr & (1 << i) == 0 {
                seq.push(Operation::Add);
            } else {
                seq.push(Operation::Mult);
            }
        }

        for i in 0..self.len {
            if self.concat_curr & (1 << i) > 0 {
                seq[i] = Operation::Concat;
            }
        }

        self.concat_curr += 1;

        Some(seq)
    }
}

#[derive(Default, Debug)]
struct Line {
    test: u64,
    seq: Vec<u64>
}

fn main() {
    let input = include_bytes!("../../inputs/day07/real.txt");

    let mut lines = vec![];
    let mut curr_line = Line::default();
    let mut curr_num = 0;

    for b in input {
        match b {
            b'\n' => {
                curr_line.seq.push(curr_num);
                curr_num = 0;
                lines.push(curr_line);
                curr_line = Line::default();
            },
            b':' => {
                curr_line.test = curr_num;
                curr_num = 0;
            }
            b' ' => {
                curr_line.seq.push(curr_num);
                curr_num = 0;
            }
            x => {
                curr_num *= 10;
                curr_num += *x as u64 - 48;
            }
        }
    }


    for i in OpSeqIter::new(3) {
        println!("{:?}", i)
    }

    
    let mut p1 = 0;
    let mut lins_processed = 0;
    let total_lines = lines.len();
    for line in lines {
        lins_processed += 1;
        println!("processing line {}/{}", lins_processed, total_lines);

        let op_iter = OpSeqIter::new(line.seq.len() - 1);
        for op_seq in op_iter {
            let mut curr = line.seq[0];

            for i in 0..op_seq.len() {
                let x = line.seq[i + 1];

                match op_seq[i] {
                    Operation::Add => {
                        curr = x + curr;
                    }
                    Operation::Mult => {
                        curr = x * curr;
                    }
                    Operation::Concat => {
                        curr = concat(curr, x);
                    }
                }
            }

            if curr == line.test {
                // println!("found matching line {:?}", line);
                p1 += line.test;
                break;
            }
        }
    }

    println!("{}", p1)
}

fn concat(mut x: u64, mut y: u64) -> u64 {
    let x_s = format!("{x}{y}");
    return x_s.parse().unwrap();
    // while y > 0 {
    //     x *= 10;
    //     x += y % 10;
    //     y /= 10
    // }

    // x
}

// 89168268445976 too low