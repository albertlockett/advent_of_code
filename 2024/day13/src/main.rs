fn main() {
    let input = include_str!("../../inputs/day13/real.txt");

    let games = input.split("\n\n");

    let mut p1_tokens = 0.0;
    let mut p2_tokens = 0.0;
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

        // part 1 =
        let b = (-1.0 * xa * yc / (xb * ya) + xc / xb) / (1.0 - xa * yb / (xb * ya));
        let a = (-1.0 * b * yb + yc) / ya;

        if is_basically_integer(b) && is_basically_integer(a) {
            p1_tokens += 3.0 * a + 1.0 * b;
        }

        // part 2
        let gcfs = gcfs(xc as u64 + 10000000000000, yc as u64 + 10000000000000);

        for factor in gcfs {
            let factor = factor as f64;
            let yc = (yc + 10000000000000.0) / factor;
            let xc = (xc + 10000000000000.0) / factor;

            let b = factor * (-1.0 * xa * yc / (xb * ya) + xc / xb) / (1.0 - xa * yb / (xb * ya));
            let a = factor * (-1.0 * b * yb + yc) / ya;

            if !is_basically_integer(b) || !is_basically_integer(a) {
                continue;
            }

            let tokens = 3.0 * a + 1.0 * b;
            p2_tokens += tokens;
            break;
        }
    }

    println!("\n~~~~\np1 = {}\np2 = {}", p1_tokens, p2_tokens);
    assert_eq!(p1_tokens, 36870.0);
    assert_eq!(p2_tokens, 78101482023732.0);
}

fn is_basically_integer(f: f64) -> bool {
    let m = f % 1.0;
    let threshold = 0.001;
    if m < threshold {
        return true;
    }

    if 1.0 - m < threshold {
        return true;
    }

    return false;
}

fn parse_button_num(s: &str) -> f64 {
    s.split("+")
        .last()
        .unwrap()
        .trim_end_matches(",")
        .parse()
        .unwrap()
}

fn parse_prize_num(s: &str) -> f64 {
    s.split("=")
        .last()
        .unwrap()
        .trim_end_matches(",")
        .parse()
        .unwrap()
}

fn gcfs(x: u64, y: u64) -> Vec<u64> {
    let mut result = vec![1];

    let gcd = gcd(x, y);

    for i in 1..((gcd as f64).sqrt().ceil() as u64) {
        if gcd % i == 0 {
            result.push(i);
            result.push(gcd / i);
        }
    }

    result
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y > 0 {
        let tmp = x % y;
        x = y;
        y = tmp;
    }

    x
}
