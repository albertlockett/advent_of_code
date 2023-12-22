use std::fs::File;
use std::io::prelude::*;

fn hash(s: &str) -> u128 {
    let mut curr_val = 0;
    for c in s.chars() {
        curr_val += c as u128;
        curr_val *= 17;
        curr_val %= 256;
    }
    return curr_val;
}

struct Lense {
    id: String,
    focal_len: u64,
}

struct Box {
    lenses: Vec<Option<Lense>>,
}

impl Box {
    fn new() -> Box {
        Box { lenses: Vec::new() }
    }

    fn insert_lense(&mut self, lense: Lense) {
        let mut idx = None;
        for (i, l) in self.lenses.iter().enumerate() {
            if let Some(l) = l {
                if lense.id == l.id {
                    idx = Some(i);
                    break;
                }
            }
        }
        if let Some(idx) = idx {
            self.lenses[idx] = Some(lense);
        } else {
            self.lenses.push(Some(lense));
        }
    }

    fn remove_lense(&mut self, lense: Lense) {
        let mut idx = None;
        for (i, l) in self.lenses.iter().enumerate() {
            if let Some(l) = l {
                if lense.id == l.id {
                    idx = Some(i);
                    break;
                }
            }
        }
        if let Some(idx) = idx {
            self.lenses[idx] = None;
        }
    }

    fn power(&self) -> u64 {
        let mut power = 0;
        for (i, lense) in self.lenses.iter().filter(|l| l.is_some()).enumerate() {
            if let Some(lense) = lense {
                power += (i + 1) as u64 * lense.focal_len
            }
        }
        return power;
    }
}

enum Command {
    Insert(Lense),
    Remove(Lense),
}

impl Command {
    fn new(cmd_raw: &str) -> Self {
        for (idx, c) in cmd_raw.chars().enumerate() {
            if c == '-' {
                let id = cmd_raw[0..idx].to_string();
                return Self::Remove(Lense {
                    id: id,
                    focal_len: 0,
                });
            }
            if c == '=' {
                let id = cmd_raw[0..idx].to_string();
                let focal_len = cmd_raw[idx + 1..].parse::<u64>().unwrap();
                return Self::Insert(Lense {
                    id: id,
                    focal_len: focal_len,
                });
            }
        }

        panic!("Invalid command: {}", cmd_raw);
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let p1_total: u128 = contents
        .split("\n")
        .flat_map(|s| s.split(","))
        .map(|s| hash(s))
        .sum();
    println!("p1 total = {}", p1_total);

    let mut boxes = (0..256)
        .into_iter()
        .map(|_| Box::new())
        .collect::<Vec<Box>>();
    contents
        .split("\n")
        .flat_map(|s| s.split(","))
        .for_each(|s| {
            let cmd = Command::new(s);
            match cmd {
                Command::Insert(lense) => {
                    boxes[hash(&lense.id) as usize].insert_lense(lense);
                }
                Command::Remove(lense) => {
                    boxes[hash(&lense.id) as usize].remove_lense(lense);
                }
            }
        });

    let p2_total: u64 = boxes
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) as u64 * b.power())
        .sum();
    println!("p2 total = {}", p2_total);
}
