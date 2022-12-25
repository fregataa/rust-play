use std::io;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    print!(
        "{}\n{}\n{}\n{}\n{}\n",
        nums[0] + nums[1],
        nums[0] - nums[1],
        nums[0] * nums[1],
        nums[0] / nums[1],
        nums[0] % nums[1]
    );
}
