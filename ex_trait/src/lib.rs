// trait은 다른 언어의 interface역할을 한다
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply:bool,
    pub retweet:bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}