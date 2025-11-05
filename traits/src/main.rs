use std::fmt::Display;

struct NewsArticle {
    headline: String,
    author: String,
    content: String,
}
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {}", self.headline, self.author)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}
struct Tweet {
    username: String,
    content: String,
    retweet: bool,
    reply: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_2(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    println!("New article available! {}", news_article.summarize());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        retweet: false,
        reply: true,
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(&news_article);
    notify(&tweet);
}
