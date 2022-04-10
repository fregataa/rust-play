extern crate rand;

use std::fs::File;
use std::io::{prelude::*, Error};
use std::collections::HashMap;

#[derive(Debug)]
struct Question {
    number: u64,
    question: String,
    next: Option<u64>,
}

fn update_qcands(map: &mut HashMap<u64, Question>, n: u64, q: String) -> HashMap<u64, Question> {
    let new_q = Question {
        number: n,
        question: q,
        next: None,
    };

    map.insert(n, new_q);
    return map;
}

fn remove_qcand(map: &mut HashMap<u64, Question>, n: u64) -> Result<HashMap<u64, Question>, String> {
    let length = map.keys().len() as u64;
    let mut idx = n;
    let mut given_q: Option<String> = None;
    while let Some(q) = map.get_mut(&idx) {
        if let None = given_q {
            given_q = Some((*q).question.to_string());
            idx = (n + 1) % length;
            continue;
        }
        match (*q).next {
            Some(nxt) => {
                idx = nxt;
                continue;
            },
            None => {
                let updated = Question {
                    number: n,
                    question: given_q.unwrap(),
                    next: Some(idx),
                };
                map.insert(n, updated);
                return Ok(map);
            },
        }
    }
    return Err(format!("No key {n} in the map").to_string());
}

fn read_file(filename: String) -> Result<File, Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    return Ok(file);
}

fn main() {
    let mut question_cands: HashMap<u64, Question> = HashMap::new();
    update_qcands(&mut question_cands, 0, "Name".to_string());
    update_qcands(&mut question_cands, 1, "Gender".to_string());
    update_qcands(&mut question_cands, 2, "Age".to_string());

    remove_qcand(&mut question_cands, 2);
    
    println!("{:?}", question_cands)
}