use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct Node {
    id: String,
    l: String,
    r: String,
}

impl Node {
    fn new(line: &str) -> Self {
        let id = line.chars().take(3).collect::<String>();
        let l = line.chars().skip(7).take(3).collect::<String>();
        let r = line.chars().skip(12).take(3).collect::<String>();

        Node { id, l, r }
    }

    fn is_p2_start(&self) -> bool {
        self.id.chars().nth(2).unwrap() == 'A'
    }

    fn is_p2_end(&self) -> bool {
        self.id.chars().nth(2).unwrap() == 'Z'
    }
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut line_iter = contents.split("\n").into_iter();

    let input_line = line_iter.next().unwrap();

    line_iter.next(); // skip blank line

    let mut nodes_by_id = HashMap::<String, Node>::new();
    line_iter.map(Node::new).for_each(|node| {
        nodes_by_id.insert(node.id.clone(), node);
    });

    // let p1_moves = do_part1(input_line, &nodes_by_id);
    // println!("P1 Moves: {}", p1_moves);

    let p2_moves = do_part2(input_line, &nodes_by_id);
    println!("P2 Moves: {}", p2_moves);
}

fn do_part1(input: &str, nodes_by_id: &HashMap<String, Node>) -> u32 {
    let mut input_iter = input.chars().into_iter();
    let mut moves = 0;

    let mut curr_node = "AAA";
    while "ZZZ" != curr_node {
        let node = nodes_by_id.get(curr_node).unwrap();
        match input_iter.next() {
            Some('L') => curr_node = &node.l,
            Some('R') => curr_node = &node.r,
            None => {
                input_iter = input.chars().into_iter();
                moves -= 1;
            }
            _ => panic!("Invalid input"),
        }
        moves += 1;
    }

    moves
}

fn do_part2(input: &str, nodes_by_id: &HashMap<String, Node>) -> u128 {
    let starts = nodes_by_id
        .values()
        .filter(|node| node.is_p2_start())
        .collect::<Vec<_>>();
    let mut all_moves = vec![];

    for start in starts {
        // find number of moves for this start
        let mut input_iter = input.chars().into_iter();
        let mut moves = 0;
        let mut curr_id = start.id.as_str();
        while nodes_by_id.get(curr_id).unwrap().is_p2_end() == false {
            let node = nodes_by_id.get(curr_id).unwrap();
            match input_iter.next() {
                Some('L') => curr_id = &node.l,
                Some('R') => curr_id = &node.r,
                None => {
                    input_iter = input.chars().into_iter();
                    moves -= 1;
                }
                _ => panic!("Invalid input"),
            }
            moves += 1;
        }
        all_moves.push(moves);
    }

    fn gcd(a: u128, b: u128) -> u128 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    let moves = all_moves
        .into_iter()
        .reduce(|acc, curr| acc * curr / gcd(acc, curr))
        .unwrap();

    moves
}
