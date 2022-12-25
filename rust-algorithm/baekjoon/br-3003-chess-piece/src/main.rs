// fn main() {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     let nums: Vec<i16> = input
//         .split_whitespace()
//         .map(|s| s.parse().unwrap())
//         .collect();
    
//     let l = [1, 1, 2, 2, 2, 8];
//     let mut ans = [0; 6];
//     for n in 0..6 {
//         ans[n] = l[n] - nums[n];
//     }
//     print!("{}", ans.map(|x| x.to_string()).join(" "));
// }

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    [1, 1, 2, 2, 2, 8]
        .iter()
        .zip(buf.split_whitespace().map(|s| s.parse::<i32>().unwrap()))
        .for_each(|(x, y)| {
            print!("{} ", x - y);
        });
}
