use std::collections::HashMap;

fn main() {
    let mut vector1: Vec<i32> = Vec::new();
    vector1.push(10);
    println!("vector1: {:?}", vector1);
    let mut vector2 = vec![1, 2, 3];
    vector2.pop();
    println!("vector2: {:?}", vector2);

    let first_element = &vector2[0];
    println!("first_element: {first_element}");

    match vector2.get(2) {
        Some(value) => println!("the value is: {value}"),
        None => println!("Not exists"),
    }
    if let Some(value) = vector2.get(1) {
        println!("the value is: {value}");
    }
    let value = match vector2.get(2) {
        Some(value) => value,
        None => &0,
    };

    println!("the value is: {value}");

    for v in &vector2 {
        println!("loop: the value is: {v}");
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("value of v is :{:?}", v);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(2.2),
    ];
    println!("value of row is :{:?}", &row);
    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("Integer: {value}"),
            SpreadsheetCell::Float(value) => println!("Float: {value}"),
            SpreadsheetCell::Text(value) => println!("String: {value}"),
        }
    }
    println!("-----------------------------------------");
    let data = "initian content";
    // let data = String::from("initian content");
    let mut s = data.to_string();
    s.push_str(" (Ç)");
    println!("String: {s}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("{s3}");

    let s1 = String::from("077");
    let s2 = String::from("135");
    let s3 = String::from("309");
    let s4 = String::from("08");
    let cpf_formated = format!("{s1}.{s2}.{s3}-{s4}");
    println!("{cpf_formated}, s1:{s1}");

    for c in cpf_formated.chars() {
        println!("{c}");
    }
    println!("-----------------------------------------");
    println!("Initial HashMap");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("scores: {:?}", scores);
    // println!("score Blue: {}", scores.get(&String::from("Blue")));
    // if let Some(score) = scores.get(&String::from("Blue")) {
    if let Some(score) = scores.get("Blue") {
        println!("score Blue: {score}");
    }
    match scores.get("Red") {
        Some(value) => println!("score Red: {value}"),
        None => (),
    }
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 30];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    dbg!(scores);
    println!();
    println!("-----------------------------------------");
    println!("HashMap");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(37);
    println!("modified: {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("count words: {:?}", map);
}
