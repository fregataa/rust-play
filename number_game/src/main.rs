extern crate rand;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Question {
    number: u64,
    question: String,
    next: Option<u64>,
}

fn read_file(filename: String) -> std::io::Result<String> {
    // file must be located in the same level with src directory.

    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn update_qcands(map: &mut HashMap<u64, Question>, n: u64, q: String) -> &mut HashMap<u64, Question> {
    let new_q = Question {
        number: n,
        question: q,
        next: None,
    };

    map.insert(n, new_q);
    return map;
}

fn make_qmap_from_file(filename: String) -> std::io::Result<HashMap<u64, Question>> {
    let mut question_cands: HashMap<u64, Question> = HashMap::new();

    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let mut map_idx = 0;
    
    for (idx, line) in buf_reader.lines().enumerate() {
        let l = line?;
        let splited = l.split('-').collect::<Vec<_>>();
        if splited.len() == 1 {
            update_qcands(&mut question_cands, map_idx, splited[0].to_string());
        } else {
            let i = idx as u64;
            let q_idx = splited[0].parse::<u64>().unwrap();
            match q_idx > i {
                true => if q_idx > map_idx {map_idx = q_idx},
                false => if i > map_idx {map_idx = i},
            };
            update_qcands(&mut question_cands, map_idx, splited[1].to_string());
        }
        map_idx += 1;
    }

    return Ok(question_cands);
}

fn remove_qcand(map: &mut HashMap<u64, Question>, n: u64) -> Result<(), String> {
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
                return Ok(());
            },
        }
    }
    return Err(format!("No key {n} in the map").to_string());
}

fn get_qcand(map: &HashMap<u64, Question>, n: u64) -> Result<&Question, String> {
    let mut idx = n;
    while let Some(q) = map.get(&idx) {
        match (*q).next {
            Some(nxt) => {
                idx = nxt;
                continue;
            },
            None => {
                return Ok(q);
            },
        }
    }
    return Err(format!("No key {n} in the map").to_string());
}

fn main() {
    let mut question_cands = make_qmap_from_file("test.txt".to_string()).unwrap();
    println!("{:?}", question_cands);
}