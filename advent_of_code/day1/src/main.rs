use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let filename = "input.txt";
    let file = File::open(filename).unwrap();
    let buf_reader = BufReader::new(file);
    let mut answer = 0;
    
    let mut prev: u64 = 0;
    let mut sums = [0, 0, 0, 0];

    for (idx, line) in buf_reader.lines().enumerate() {
        let l = line.unwrap();
        let cur = l.parse::<u64>().unwrap();
        let i = idx as u64;
        let ridx = i % 4;

        match i {
            0 => {
                sums[0] += cur;
                continue;
            },
            1 => {
                sums[0] += cur;
                sums[1] += cur;
                continue;
            },
            _ => {
                for si in 0..4 as u64 {
                    if (ridx + 1) % 4 == si { continue; }
                    sums[si as usize] += cur;
                }
            },
        }
        let sidx = match ridx {
            2 => 0,
            3 => 1,
            0 => 2,
            1 => 3,
            _ => 0,
        } as usize;
        let part_sum = sums[sidx];
        if prev != 0 {
            if prev < part_sum {
                answer += 1;
            }
        }
        prev = part_sum;
        sums[sidx] = 0;
    }
    
    println!("{}", answer);
}
