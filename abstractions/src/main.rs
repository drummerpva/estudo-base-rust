use std::fmt::Display;

use abstractions::{notify, notify2, returns_summalizable, NewsArticle, Summary, Tweet};

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn only_for_float(&self) -> f32 {
        self.x.powi(2)
    }
}
struct MixPoint<T, U> {
    x: T,
    y: U,
}
impl<X1, Y1> MixPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixPoint<X2, Y2>) -> MixPoint<X1, Y2> {
        MixPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number
        }
    }
    println!("The largest number is: {largest}");
    let number_list = vec![340, 50, 250, 100, 650];
    let largest = get_largest_i32(&number_list);
    println!("The largest number is: {largest}");

    let char_list = vec!['I', 'M', 'C', 'A'];
    let largest = get_largest_char(&char_list);
    println!("The largest char is: {largest}");

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 5.2, y: 10.1 };
    let _float32 = Point {
        x: 5.2 as f32,
        y: 10.1,
    };
    println!("X is: {}", _float.x());
    println!("OnlyForFloat is: {}", _float32.only_for_float());
    // let _wont_work = Point { x: 4, y: 5.2 };
    let _mix_number = MixPoint { x: 4, y: 5.2 };

    let mix1 = MixPoint { x: 3, y: 3.2 };
    let mix2 = MixPoint { x: 3.2, y: "ABC" };
    let mix3 = mix1.mixup(mix2);
    println!("MIX: x: {}, y:{}", mix3.x, mix3.y);

    let new_article = NewsArticle {
        author: "Douglas Poma".into(),
        content: "Content".into(),
        headline: "HeadLine".into(),
        location: "Papanduva/SC".into(),
    };
    println!("New Article! {}", new_article.summarize());
    notify(&new_article);
    notify2(&new_article);
    let new_tweet = Tweet {
        username: "douglaspoma".into(),
        content: "This is a tweet".into(),
        reply: true,
        retweet: true,
    };
    println!("New Tweet! {}", new_tweet.summarize());

    let trait_returns = returns_summalizable();
    println!("from function: {}", trait_returns.summarize());

    let number_list = vec![340, 50, 250, 100, 650];
    let largest = get_largest(&number_list);
    println!("GENERIC: The largest number is: {largest}");

    let char_list = vec!['I', 'M', 'C', 'A'];
    let largest = get_largest_without_traits(&char_list);
    println!("GENERIC: The largest char is: {largest}");

    let string1 = String::from("teste");
    // let maior;
    {
        let string2 = String::from("teste_maior");
        // maior = longest(string1.as_str(), string2.as_str());
        let maior = longest(string1.as_str(), string2.as_str());
        println!("More longest string: {}", maior);
    }
    // println!("More longest string: {}", maior);

    //
    let movel = String::from("Call me Ishmael. Some years ago...");
    let fist_sentence = movel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: fist_sentence,
    };
    println!("First setencen from struct: '{}'", i.part);
    let part = i.announce_and_return_part("testando anuncio");
    println!("part returned: '{}'", part);

    //
    let _s: &'static str = "I have a static lifetime. saved in program binary";
}

fn get_largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number
        }
    }
    largest
}
fn get_largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}
fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}
fn get_largest_without_traits<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}
/* pub fn _longest_invalid(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Atention for this announce: {}", announcement);
        self.part
    }
}

fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
