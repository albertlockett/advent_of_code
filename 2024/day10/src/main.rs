fn main() {
    let (p1_total, p2_total) = day10::doit();

    println!("p1 = {}", p1_total);
    println!("p2 = {}", p2_total);

    assert_eq!(p1_total, 557);
    assert_eq!(p2_total, 1062);
}
