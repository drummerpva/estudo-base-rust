use std::fmt::{Debug, Display};

pub trait Summary {
    // fn summarize(&self) -> String;
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
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
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
    /*     fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    } */
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}
pub fn notify3<T: Summary + Display>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

fn _some_function<T, U>(_t: &T, _u: &U) -> u32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}

pub fn returns_summalizable() -> impl Summary {
    Tweet {
        content: "Content test".into(),
        reply: true,
        retweet: false,
        username: "usuario".into(),
    }
}
/* pub fn returns_summalizable_distinct(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            author: "author",
            content: "content",
            headline: "hadline",
            location: "location",
        }
    } else {
        Tweet {
            content: "Content test".into(),
            reply: true,
            retweet: false,
            username: "usuario".into(),
        }
    }
}
 */

pub struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x: {}", self.x);
        } else {
            println!("The largest number is y: {}", self.y);
        }
    }
}
