
fn main() {
    let tweet = Tweet {
        username: String::from("Lee"),
        content: String::from("Hello world, mom, dad"),
        reply: false,
        retweet: false,
    };
    println!("New Tweet! =====\n{}", tweet.summary());
}