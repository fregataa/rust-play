use trait_basic::*;

fn main() {
    let tweet = Tweet {
        username: String::from("Lee"),
        content: String::from("Hello world, mom, dad"),
        reply: false,
        retweet: false,
    };

    let news = NewsArticle {
        headline: String::from("One small step to the future"),
        location: String::from("Science"),
        author: String::from("Lee"),
        content: String::from("James Webb Telescope finally started functioning today morning. blahhh..."),
    };
    
    // println!("New Tweet! =====\n{}", tweet.summary());
    // println!("New Tweet! =====\n{}", news.summary());
    notify(news);
}
