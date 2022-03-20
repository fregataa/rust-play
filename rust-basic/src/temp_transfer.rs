// F = 9 / 5 * C + 32
// C = (F-32) * 5 / 9

use std::io;

fn enter_num() -> i32 {
    let mut num = String::new();
    io::stdin()
    .read_line(&mut num)
    .expect("Failed to read an input.");

    let num: i32 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Type a number.");
            return -999;
            // Should raise an error or... handle this.
        },
    };
    return num;
}

fn to_far() {
    println!("Enter Celcius degree.")
    let c = enter_num();
    let f = c * 9 / 5 + 32;
    println!("{} Celcius is {} Farenheit.", c, f)
}

fn to_cel() {
    println!("Enter Farenheit degree.")
    let f = enter_num();
    let c = (f - 32) * 5 / 9;
    println!("{} Farenheit is {} Celcius.", f, c)
}

fn main(){
    println!("What do you want?");
    println!("1. From Celcius to Farenheit.");
    println!("2. From Farenheit to Celcius.");

    let menu = enter_num();

    match menu {
        1 => to_far(),
        2 => to_cel(),
        _ => println!("You must type 1 or 2 to choose menu."),
    }
}
