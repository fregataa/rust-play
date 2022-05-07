extern crate rand;

use rand::prelude::*;

enum GameMode {
    Normal,
    Stupid,
}

struct GameInfo {
    session_id: u64,
    users: Vec<String>,
    num_liers: u64,
    liers: Vec<String>,
    mode: GameMode,
    topic: String,
    word: String,
    lie: String,
}

fn get_word(mode: &GameMode, topic: &str) -> (String, String) {
    // Hard coded examples.

    let word = "Coffee".to_owned();
    let lie = "Curry".to_owned();

    return (word, lie);
}

fn get_mode() -> GameMode {
    // Hard coded examples.

    let mode = GameMode::Normal;
    return mode;
}

fn get_users() -> Vec<String> {
    // Hard coded examples.

    let users: Vec<String> = vec![
        "Lee".to_owned(), "Kim".to_owned(),
        "Park".to_owned(), "Choi".to_owned(),
    ];
    return users;
}

fn get_num_liers() -> u64 {
    // Hard coded examples.

    return 1;
}

fn get_topic() -> String {
    // Hard coded examples.

    return "Something to eat or drink".to_owned();
}

fn init_game(
    mode: GameMode,
    users: Vec<String>,
    num_liers: u64,
    topic: &str,
) -> GameInfo {
    let session_id = 0; // Need to use a real session id.
    let mut rng = rand::thread_rng();
    let mut lier_index: Vec<u64> = (0..num_liers).collect();
    lier_index.shuffle(&mut rng);

    let _mode = &mode;
    let (word, lie) = get_word(_mode, topic);

    let mut liers: Vec<String> = Vec::new();
    for lier in lier_index {
        let idx = lier as usize;
        liers.push(users[idx].to_owned());
    }

    let game_info = GameInfo {
        session_id,
        num_liers,
        liers,
        mode,
        word,
        lie,
        // users: users.into_iter().map(|s| s.to_owned()).collect(),
        users,
        topic: topic.to_owned(),
    };

    return game_info;
}

fn get_lier_pick() -> Vec<String> {
    return vec!["Lee".to_owned()];
}


fn pick_lier(game_info: &GameInfo) -> bool {
    let pick = get_lier_pick(); // Hard coded pick lier.
    let liers = &game_info.liers;

    for p in pick {
        if let None = liers.iter().position(|lier| *lier == p) {
            return false;
        }
    }
    return true;
}

fn finish_game(result: bool) {
    match result {
        true => println!("Correct !!"),
        false => println!("Wrong ..."),
    }
}

fn main() {
    let users = get_users();
    let mode = get_mode();
    let num_liers = get_num_liers();
    let topic = get_topic();

    let game_info = init_game(mode, users, num_liers, &topic[..]);

    let _game_info = &game_info;
    let result = pick_lier(_game_info);
    finish_game(result);
}
