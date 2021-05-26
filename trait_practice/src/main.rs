fn main() {
    let a: NewsArticle = NewsArticle {
        headline: "A".to_string(),
        location: "B".to_string(),
        author: "C".to_string(),
        content: "D".to_string(),
    };

    println!("{}", a.summarize());

    let b: Tweet = Tweet {
        username: "A".to_string(),
        content: "B".to_string(),
        reply: true,
        retweet: true,
    };

    println!("{}", b.summarize());
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
