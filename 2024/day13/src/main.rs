
fn main() {
    let input = include_str!("../../inputs/day13/real.txt");

    let games = input.split("\n\n");

    let mut total_tokens = 0.0;
    for game in games {
        println!("\n{}", game);
        let lines = game.split("\n").collect::<Vec<_>>();
        let a = lines.get(0).unwrap().split(" ").collect::<Vec<_>>();
        let xa = parse_button_num(a.get(2).unwrap());
        let ya = parse_button_num(a.get(3).unwrap());
        let b = lines.get(1).unwrap().split(" ").collect::<Vec<_>>();
        let xb = parse_button_num(b.get(2).unwrap());
        let yb = parse_button_num(b.get(3).unwrap());

        let prize = lines.get(2).unwrap().split(" ").collect::<Vec<_>>();
        let xc = parse_prize_num(prize.get(1).unwrap());
        let yc = parse_prize_num(prize.get(2).unwrap());

        let b = (-1.0 * xa * yc / (xb * ya) + xc / xb) / (1.0 - xa * yb / (xb * ya));
        let a = (-1.0 * b * yb + yc) / ya;
        println!("a = {}\nb={}", a, b);

        if !is_basically_integer(b) || !is_basically_integer(a) {
            println!("No winners");
            continue;
        }

        let tokens = 3.0 * a + 1.0 * b;
        println!("tokens = {}", tokens);

        total_tokens += tokens;
    }

    println!("\n~~~~\np1 = {}", total_tokens);
}

fn is_basically_integer(f: f64) -> bool {
    let m = f % 1.0;
    let threshold = 0.0001;
    if m < threshold {
        return true
    }

    if 1.0 - m < threshold {
        return true
    }

    return false
}

fn parse_button_num(s: &str) -> f64 {
    s.split("+").last().unwrap().trim_end_matches(",").parse().unwrap()
}

fn parse_prize_num(s: &str) -> f64 {
    s.split("=").last().unwrap().trim_end_matches(",").parse().unwrap()
}
