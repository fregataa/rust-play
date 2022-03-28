extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use std::env::current_dir;
// use rand::Rng;

struct Question {
    number: u64,
    question: String,
    next: Option<u64>,
}

fn pick_restored(questions: HashMap<u64, Question>, num: u64) -> &'static Question {
    let q = questions.get(&num).unwrap();
    return q;
}

fn pick_nonrestored(questions: HashMap<u64, Question>, num: u64, q_len: u64) -> &'static Question {
    // If the Question is used, set the question's next to other question's number.
    // If the Question is not used, next is -1 (or None maybe?).
    // If the question's next is not -1 (or not None), then follow the next till its not -1.

    let mut current = num;
    let mut q = questions.get(&current).unwrap();
    loop {
        match q.next {
            None => break,
            Some(nxt) => {
                q = questions.get(&nxt).unwrap();
                current = nxt;
            }
        }
    }
    // let new_q = Question {
    //     number: current,
    //     question: q.question,
    //     next: Some((current + 1) % q_len),
    // };
    questions.insert(
        current,
        Question {
            number: current,
            question: q.question.to_string(),
            next: Some((current + 1) % q_len),
        },
    );
    return q;
}

fn main() {
    let filename = format!(
        "{}/questions.txt",
        current_dir().unwrap().into_os_string().into_string().unwrap()
    );
    let mut file = File::open(filename)
    .expect("Cannot open the file.");

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)
    .expect("Cannot read the file to buffer.");

    println!("With text:\n{}", contents);

    // let question_cands = HashMap::from([

    // ]);
}


// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     let team_name = String::from("Blue");
//     println!("{}", scores.get(&team_name).unwrap());
//     println!("{}", scores.get(&team_name).unwrap());
//     println!("{}", scores.get(&team_name).unwrap());
// }