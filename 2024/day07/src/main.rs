

#[derive(Debug)]
enum Operation {
    Add,
    Mult,
}

struct OpSeqIter {
    len: usize,
    curr: u16,
}

impl OpSeqIter {
    fn new(len: usize) -> Self {
        Self { len, curr: 0 }
    }
}

impl Iterator for OpSeqIter {
    type Item = Vec<Operation>;

    fn next(&mut self) -> Option<Vec<Operation>> {
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
        self.curr += 1;

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
        // print!("{}", b);
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
                // println!("\n{}", curr_num);
                curr_num *= 10;
                curr_num += *x as u64 - 48;
            }
        }
    }

    
    let mut p1 = 0;
    for line in lines {
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
                }

                if curr > line.test {
                    break
                }
            }

            if curr == line.test {
                println!("found matching line {:?}", line);
                p1 += line.test;
                break;
            }
        }
    }

    println!("{}", p1)
}
