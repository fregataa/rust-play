use std::io;
use std::collections::HashMap;

fn main() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut cmd = String::new();
        match io::stdin().read_line(&mut cmd) {
            Ok(_) => {
                let words = cmd.split(" ").collect::<Vec<&str>>();
                match words[0] {
                    "Add" => add(&mut db, words[1], words[words.len()-1]),
                    "List" => list(&mut db, words[1]),
                    "Exit" | _ => {
                        println!("Done");
                        break;
                    },
                }
            },
            Err(_) => {
                println!("Input Error.");
                continue;
            }
        }
        println!();
    }
}

fn add(db: &mut HashMap<String, Vec<String>>, human: &str, dept: &str) {
    let key = String::from(dept);
    let val = String::from(human);
    db.entry(key).or_insert(Vec::<String>::new()).push(val);
    list(db, dept);
}

fn list(db: &mut HashMap<String, Vec<String>>, dept: &str) {
    let key = String::from(dept);
    print!("Department: {}", dept);
    if let Some(v) = db.get(&key) {
        println!("{:?}", v);
    }
}
