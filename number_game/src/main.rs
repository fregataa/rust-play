extern crate rand;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

use rand::Rng;

#[derive(Debug)]
struct Question {
    number: u64,
    question: String,
    next: Option<u64>,
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

fn remove_qcand(map: &mut HashMap<u64, Question>, removed: u64, target: u64) -> Result<(), ()> {
    let mut removed_q = map.get_mut(&removed).unwrap();
    (*removed_q).next = Some(target);
    return Ok(());
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
    return Err(format!("Cannot get key {n} in the map").to_string());
}

fn main() {
    let mut question_cands = make_qmap_from_file("test.txt".to_string()).unwrap();
    let cands_length = question_cands.keys().len() as u64;
    let mut rng = rand::thread_rng();

    loop {
        println!("Press Enter if you want to proceed ...");
        io::stdin().read_line(&mut "".to_string())
        .expect("Failed to proceed");

        let q_len = &cands_length;
        let rand_idx = rng.gen_range(1..*q_len);
        let cands_r = &question_cands;
        let cur_q = get_qcand(cands_r, rand_idx).unwrap();
        let q_num = (*cur_q).number;

        println!("=============\nQuestion {}.", q_num);
        println!("{}", (*cur_q).question);
        println!("");

        let cands_r = &question_cands;
        let q_len = &cands_length;
        let target_qnum = (*get_qcand(cands_r, (q_num + 1) % *q_len).unwrap()).number;
        let cands_r = &mut question_cands;
        if q_num == target_qnum {
            println!("All questions have been used !!");
            println!("Finishing.");
            break;
        }
        remove_qcand(cands_r, q_num, target_qnum).unwrap();
    }
}
