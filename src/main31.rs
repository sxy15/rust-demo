use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for NewsArticle {

    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// pub fn notify (item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify<T: Summary> (item: T) {
//     println!("Breaking news! {}", item.summarize());
// }

// pub fn notify (item: impl Summary + Display) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("{}", a.summarize())
}

pub fn notify2<T, U>(a: T, b: U) -> String
    where T: Summary + Display,
          U: Clone + Debug
{
    format!("{}", a.summarize())
}

fn main() {
    let new_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best")
    };

    println!("New article available! {}", new_article.summarize());
}