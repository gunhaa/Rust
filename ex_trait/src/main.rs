// 프로젝트 이름
use ex_trait::{Summary, Tweet};

fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false, 
        retweet: false,
    };

    println!("1 new tweet : {}" , tweet.summarize());

    notify(&tweet);
}

pub fn notify(item: &impl Summary){
    print!("Breaking news! {}" , item.summarize())
}