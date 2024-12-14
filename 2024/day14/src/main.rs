use std::collections::BTreeSet;

fn main() {
    // let input = include_str!("../../inputs/day14/test2.txt");
    // let width = 11;
    // let height = 7;
    // let turns = 7;
    let input = include_str!("../../inputs/day14/real.txt");
    let width = 101;
    let height = 103;
    let turns = 100;

    let mut x = vec![];
    let mut y = vec![];
    let mut dx = vec![];
    let mut dy = vec![];

    input.split("\n").map(|line| {
        let mut tmp = line.split(" ");
        (
            tmp.next().unwrap().split("=").skip(1).next().unwrap().split(","),
            tmp.next().unwrap().split("=").skip(1).next().unwrap().split(",")
        )
    }).for_each(|(mut pos, mut vel)| {
        x.push(pos.next().unwrap().parse::<i32>().unwrap());
        y.push(pos.next().unwrap().parse::<i32>().unwrap());
        dx.push(vel.next().unwrap().parse::<i32>().unwrap());
        dy.push(vel.next().unwrap().parse::<i32>().unwrap());
    });

    // part 1

    let mut q_nw = 0;
    let mut q_ne = 0;
    let mut q_sw = 0;
    let mut q_se = 0;

    for i in 0..x.len() {
        let mut final_x = (x[i] + dx[i] * turns) % width;
        let mut final_y = (y[i] + dy[i] * turns) % height;
        if final_x < 0 {
            final_x = final_x + width;
        }
        if final_y < 0 {
            final_y = final_y + height;
        }

        print!("x={}, y={} ", final_x, final_y);

        if final_y < height / 2 {
            if final_x < width / 2 {
                q_nw += 1;
                print!("nw")
            } else if final_x > width / 2 {
                q_ne += 1;
                print!("ne")
            }
        } else if final_y > height / 2 {
            if final_x < width / 2 {
                q_sw += 1;
                print!("sw")
            } else if final_x > width / 2 {
                q_se += 1;
                print!("se")
            }
        }   
        println!("")     
    }

    println!("nw = {}", q_nw);
    println!("ne = {}", q_ne);
    println!("sw = {}", q_sw);
    println!("se = {}", q_se);
    let p1 = q_ne * q_nw * q_se * q_sw;
    println!("p1 = {}", p1);


    // part 2
    // just kind of guessing that the tree has a bunch of robots lined up horizontally?
    for turn in 0..(width * height) {
        let mut positions = BTreeSet::<(i32, i32)>::new();
        for i in 0..x.len() {
            let mut final_x = (x[i] + dx[i] * turn) % width;
            let mut final_y = (y[i] + dy[i] * turn) % height;
            if final_x < 0 {
                final_x = final_x + width;
            }
            if final_y < 0 {
                final_y = final_y + height;
            }
            positions.insert((final_x, final_y));
        }

        let mut string_as_byte = Vec::<u8>::with_capacity(((width + 1) * height) as usize);
        let mut max_streak = 0;
        let mut curr_streak = 0;
        for y in 0..height {
            for x in 0..width {
                if positions.contains(&(x, y)) {
                    string_as_byte.push(b'#');
                    curr_streak += 1;
                    max_streak = max_streak.max(curr_streak);
                } else {
                    string_as_byte.push(b'.');
                    curr_streak = 0;

                }
            }
            string_as_byte.push(b'\n');
            curr_streak = 0;
        }

        if turn % 100 == 0 {
            println!("TURN = {} / {}", turn, width * height);
        }
        if max_streak > 8 {
            let str = String::from_utf8(string_as_byte).unwrap();
            println!("{}\n", str);

            println!("p2 tree at {}\n", turn);
            break;
        }
    }


    

}
