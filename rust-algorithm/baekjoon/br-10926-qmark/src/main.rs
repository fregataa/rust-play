fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let val = input.trim();
    print!("{}??!", val);
}
