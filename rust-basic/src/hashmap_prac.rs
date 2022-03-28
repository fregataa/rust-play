use std::collections::HashMap;

#[derive(Debug)]
struct Question {
    number: u64,
    question: String,
    next: Option<u64>,
}

fn update_map(map: &mut HashMap<u64, Question>, n: u64, q: String) {
    let new_q = Question {
        number: n,
        question: q,
        next: None,
    };

    map.insert(n, new_q);
}

fn main() {
    let mut q_map = HashMap::from([
        (1, Question {
            number: 1,
            question: "Name".to_string(),
            next: None,
        }),
        (2, Question {
            number: 2,
            question: "Age".to_string(),
            next: None,
        }),
    ]);

    update_map(&mut q_map, 3, "Gender".to_string());
    println!("{:?}", q_map)
}
