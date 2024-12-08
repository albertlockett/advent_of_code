fn main() {
    let (p1, p2) = day08::doit();

    println!("p1 = {}", p1);

    // 809 = too low
    println!("p2 = {}", p2);

    assert_eq!(p1, 293);
    assert_eq!(p2, 934);
}
