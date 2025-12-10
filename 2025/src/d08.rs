use std::io::{Read, Result};

use crate::Challenge;

#[derive(Default)]
pub struct Day08 {}

impl Challenge for Day08 {
    fn do_p1(&mut self, input: &str) -> Result<usize> {
        let parser = Parser::new(Self::read_input_iter(input)?.bytes());

        let mut xs = Vec::new();
        let mut ys = Vec::new();
        let mut zs = Vec::new();
        for (x, y, z) in parser {
            xs.push(x);
            ys.push(y);
            zs.push(z);
        }

        let len = xs.len();
        let mut dist_calc = DistCal3D::new(xs, ys, zs);

        let mut top_k = TopK::<1000>::new();

        for i in 0..len {
            dist_calc.calc_dist(i);

            for j in (i + 1)..len {
                let junc_dist = JunctionDist {
                    from: i,
                    to: j,
                    dist: dist_calc.result[j] as usize,
                };
                top_k.insert(junc_dist);
            }
        }

        let mut circuit_id = 1usize;
        let mut circuits = vec![0; len];

        for connection in top_k.values {
            let from_cid = circuits[connection.from];
            let to_cid = circuits[connection.to];

            // new circuit
            if from_cid == 0 && to_cid == 0 {
                circuits[connection.from] = circuit_id;
                circuits[connection.to] = circuit_id;
                circuit_id += 1;
                continue;
            }

            // to joins circuit
            if from_cid != 0 && to_cid == 0 {
                circuits[connection.to] = from_cid;
                continue;
            }

            // from joins circuit
            if from_cid == 0 && to_cid != 0 {
                circuits[connection.from] = to_cid;
                continue;
            }

            // already connected
            if from_cid == to_cid {
                continue;
            }

            // now we're joining two circuits, need to update all the to to be from
            #[allow(clippy::needless_range_loop)]
            for i in 0..len {
                if circuits[i] == to_cid {
                    circuits[i] = from_cid;
                }
            }
        }

        let mut circuit_counts = vec![0usize; circuit_id];
        for cid in circuits {
            if cid != 0 {
                circuit_counts[cid] += 1;
            }
        }

        circuit_counts.sort();
        circuit_counts.reverse();

        let top3 = circuit_counts[0..3].iter().product();
        Ok(top3)
    }

    fn do_p2(&mut self, _input: &str) -> Result<usize> {
        Ok(0)
    }
}

#[derive(Debug)]
struct TopK<const K: usize> {
    max: usize,
    max_idx: usize,
    values: [JunctionDist; K],
}

impl<const K: usize> TopK<K> {
    fn new() -> Self {
        let values = (0..K)
            .map(|_| JunctionDist {
                dist: usize::MAX,
                from: 0,
                to: 0,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self {
            values,
            max: usize::MAX,
            max_idx: 0,
        }
    }

    fn insert(&mut self, new: JunctionDist) {
        if new.dist < self.max {
            self.values[self.max_idx] = new;
        }

        let mut new_max = 0;
        let mut new_max_idx = 0;
        for i in 0..K {
            if self.values[i].dist > new_max {
                new_max = self.values[i].dist;
                new_max_idx = i;
            }
        }

        self.max = new_max;
        self.max_idx = new_max_idx;
    }
}

#[derive(Debug)]
struct JunctionDist {
    dist: usize,
    from: usize,
    to: usize,
}

struct DistCal3D {
    x: DistCalc,
    y: DistCalc,
    z: DistCalc,
    len: usize,
    result: Vec<i64>,
}

impl DistCal3D {
    fn new(x: Vec<i64>, y: Vec<i64>, z: Vec<i64>) -> Self {
        let len = x.len();
        Self {
            x: DistCalc::new(x),
            y: DistCalc::new(y),
            z: DistCalc::new(z),
            len,
            result: vec![0; len],
        }
    }

    fn calc_dist(&mut self, idx: usize) {
        self.x.calc_dist(idx);
        self.y.calc_dist(idx);
        self.z.calc_dist(idx);

        for i in 0..self.len {
            self.result[i] = self.x.result[i] + self.y.result[i] + self.z.result[i]
        }
    }
}

struct DistCalc {
    len: usize,
    input: Vec<i64>,
    result: Vec<i64>,
}

impl DistCalc {
    fn new(input: Vec<i64>) -> Self {
        let len = input.len();
        Self {
            len,
            input,
            result: vec![0; len],
        }
    }

    fn calc_dist(&mut self, idx: usize) {
        let from = self.input[idx];

        for i in 0..self.len {
            self.result[i] = self.input[i] - from;
        }

        for i in 0..self.len {
            self.result[i] = self.result[i] * self.result[i]
        }
    }
}

struct Parser<T> {
    inner: T,
    exhausted: bool,
}

impl<T> Parser<T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            exhausted: false,
        }
    }
}

impl<T> Iterator for Parser<T>
where
    T: Iterator<Item = Result<u8>>,
{
    type Item = (i64, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }

        let mut next_x = None;
        let mut next_y = None;
        let mut curr = 0;

        loop {
            match self.inner.next().transpose().unwrap() {
                Some(b) => match b {
                    b',' => {
                        if next_x.is_none() {
                            next_x = Some(curr)
                        } else if next_y.is_none() {
                            next_y = Some(curr)
                        }
                        curr = 0;
                    }
                    b'\n' => return Some((next_x.unwrap(), next_y.unwrap(), curr)),
                    num => {
                        curr *= 10;
                        curr += (num - b'0') as i64;
                    }
                },
                None => {
                    self.exhausted = true;
                    return Some((next_x.unwrap(), next_y.unwrap(), curr));
                }
            }
        }
    }
}
