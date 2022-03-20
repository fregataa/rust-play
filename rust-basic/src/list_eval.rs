use std::collections::HashMap;

fn main() {
    let given = vec![1, 2, 2, 3, 3, 5, 5, 6, 6, 6, 7];
    // let given = vec![];
    // let given: Vec<i32> = Vec::new([1, 2, 3]);

    println!("The mean num is {}", match get_mean(&given) {
        Some(result) => result.to_string(),
        None => "No Data".to_string(),
    });

    println!("The medium num is {}", match get_medium(&given) {
        Some(result) => result.to_string(),
        None => "No Data".to_string(),
    });
    println!("The mode num is {}", match get_mode(&given) {
        Some(result) => result.to_string(),
        None => "No Data".to_string(),
    });
}

fn get_mean(v: &Vec<i32>) -> Option<f64> {
    let len = v.len();
    if len == 0 {
        return None;
    }
    let mut total = 0;
    for num in v {
        total += num;
    }

    return Some(total as f64 / len as f64);
}

fn get_medium(v: &Vec<i32>) -> Option<i32> {
    let len = v.len();
    if len == 0 {
        return None;
    }
    let mut cloned_v = v.clone();
    cloned_v.sort();

    return Some(cloned_v[len/2]);
}

fn get_mode(v: &Vec<i32>) -> Option<i32> {
    let len = v.len();
    if len == 0 {
        return None;
    }
    let mut num_cnt = HashMap::new();
    let (mut mode, mut max) = (0, 0);

    for num in v {
        let cnt = num_cnt.entry(num).or_insert(0);
        *cnt += 1;
    }

    for (n, cnt) in num_cnt.into_iter() {
        if cnt > max {
            max = cnt;
            mode = *n;
        }
    }
    return Some(mode);
}