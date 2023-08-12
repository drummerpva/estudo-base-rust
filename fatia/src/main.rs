fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    let no_first_word = remove_first_word(&s);
    println!("first word: {}", word);
    println!("whithout first word: {}", no_first_word);
    s.clear();
    println!("after clear: {}", s);
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}
fn remove_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }
    &s
}
