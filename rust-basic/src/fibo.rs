use std::io;

fn fibonacci(n: i32) -> i32 {
    fn fib(n: i32) -> i32 {
        let (mut a, mut b, mut c) = (1, 1, 2);
        for _ in 0..(n-2) {
            c = a + b;
            (a, b) = (b, c);
            println!("a={}, b={}, c={}", a, b, c)
        }
        return c;
    }
    if n < 1 {
        return -999;
    }
    match n {
        1 | 2 => return 1,
        _ => return fib(n),
    }
}

fn main() {
    let mut n = String::new();
    io::stdin()
    .read_line(&mut n)
    .expect("Failed to read input.");

    let n: i32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a number.");
            return;
        },
    };
    
    println!("Result: {}", fibonacci(n))
}
