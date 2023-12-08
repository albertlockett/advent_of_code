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

        Node {id , l, r}
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
    line_iter
        .map(Node::new)
        .for_each(|node| {
            nodes_by_id.insert(node.id.clone(), node);
        });

    let mut input_iter = input_line.chars().into_iter();
    let mut moves = 0;

    let mut curr_node = "AAA";
    while "ZZZ" != curr_node {
        let node = nodes_by_id.get(curr_node).unwrap();
        match input_iter.next() {
            Some('L') => curr_node = &node.l,
            Some('R') => curr_node = &node.r,
            None => {
                input_iter = input_line.chars().into_iter();
                moves -= 1;
            },
            _ => panic!("Invalid input"),
        }
        moves += 1;
    }

    println!("Moves: {}", moves);

}
