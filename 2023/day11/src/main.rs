use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let width = contents.split("\n").next().unwrap().len();
    let mut empty_columns = vec![Some(()); width];
    let mut empty_rows = vec![Some(()); width];

    let mut galaxies = vec![];
    contents.split("\n").enumerate().for_each(|(y, line)| {
        let mut num_galaxies = 0;
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                '#' => {
                    num_galaxies += 1;
                    empty_columns[x] = None;
                    galaxies.push(GalaxyLocation { x, y });
                },
                _ => {}
            }
        });

        if num_galaxies > 0 {
            empty_rows[y] = None;
        }
    });

    let mut tmp = BTreeMap::new();
    for (col, empty) in empty_columns.into_iter().enumerate() {
        if empty.is_some() {
            tmp.insert(col, ());
        }
    }
    let empty_columns = tmp;

    let mut tmp = BTreeMap::new();
    for (row, empty) in empty_rows.into_iter().enumerate() {
        if empty.is_some() {
            tmp.insert(row, ());
        }
    }
    let empty_rows = tmp;

    let mut total_dist = 0;
    let expand_factor = 1000000 - 1;

    for i in 1..galaxies.len() {
        let from = &galaxies[i];
        for j in 0..i {
            let to = &galaxies[j];

            let mut x = [from.x, to.x];
            x.sort();
            let mut dx = x[1] - x[0];
            if x[0] < x[1] {
                dx += empty_columns.range(x[0] + 1..x[1]).count() * expand_factor;
            }

            let mut y = [from.y, to.y];
            y.sort();
            let mut dy = y[1] - y[0];

            if y[0] < y[1] {
                dy += empty_rows.range(y[0] + 1..y[1]).count() * expand_factor;
            }

            let dist = dx + dy;
            total_dist += dist;

            println!("dist between galaxies {} and {} is {}", j+1, i+1, dist);

        }
    }

    println!("total dist is {}", total_dist);
}

struct GalaxyLocation {
    x: usize,
    y: usize
}