fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let val: u32 = input.trim().parse().unwrap();
    print!("{}", val - 543);
}
