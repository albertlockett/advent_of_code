use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let max_r = 12;
    let max_g = 13;
    let max_b = 14;

    let p1_result: u64 = contents
        .split("\n")
        .into_iter()
        .map(Game::new)
        .filter(|game| game.is_possible(max_r, max_b, max_g))
        .map(|game| game.id)
        .sum();

    println!("The result for part 1 is {:?}", p1_result);


    let p2_result: u64 = contents.split("\n")
        .into_iter()
        .map(Game::new)
        .map(|game| game.power())
        .sum();

    println!("The result for part 2 is {:?}", p2_result);

    Ok(())
}

struct Trial {
    b: u16,
    r: u16,
    g: u16,
}

impl Trial {
    fn new(desc: &str) -> Self {
        let mut trial = Trial { b: 0, r: 0, g: 0 };
        desc.split(",").for_each(|s| {
            let mut i = s.trim().split(" ");
            let val = i.next().unwrap().parse::<u16>().unwrap();
            let color = i.next();
            match color {
                Some("blue") => trial.b += val,
                Some("red") => trial.r += val,
                Some("green") => trial.g += val,
                _ => {},
            };
        });

        trial
    }
}

#[test]
fn test_trial_parse() {
    let desc = " 3 blue, 4 red";
    let t = Trial::new(desc);
    assert!(t.r == 4);
    assert!(t.b == 3);
    assert!(t.g == 0);
}

struct Game {
    id: u64,
    trials: Vec<Trial>,
}

impl Game {
    fn new(line: &str) -> Self {
        let mut iter = line.split(":").into_iter();
        let game_desc = iter.next().unwrap();
        let game_id = game_desc
            .split(" ")
            .into_iter()
            .skip(1)
            .map(|s| s.parse::<u64>().unwrap())
            .next()
            .unwrap();

        let trials = iter
            .next()
            .unwrap()
            .split(";")
            .into_iter()
            .map(Trial::new)
            .collect::<Vec<Trial>>();

        Game {
            id: game_id,
            trials: trials,
        }
    }

    fn is_possible(&self, max_r: u16, max_b: u16, max_g: u16) -> bool {
        for trial in &self.trials {
            if max_r < trial.r || max_b < trial.b || max_g < trial.g {
                return false
            }
        };

        true
    }

    fn power(&self) -> u64 {
        let mut max_b = 0;
        let mut max_g = 0;
        let mut max_r: u16 = 0;

        for trial in &self.trials {
            if trial.r > max_r {
                max_r = trial.r;
            }
            if trial.b > max_b {
                max_b = trial.b
            }
            if trial.g > max_g {
                max_g = trial.g
            }
        }; 

        return max_b as u64 * max_g as u64 * max_r as u64;
    }
}


#[test]
fn test_parse_game() {
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let g = Game::new(line);
    assert!(g.id == 1);
    assert!(g.trials.len() == 3);
}

#[test]
fn test_power() {
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let g = Game::new(line);

    assert!(g.power() == 48);
}