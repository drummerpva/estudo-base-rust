use std::collections::HashMap;

fn media(numbers: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for number in numbers {
        sum += number
    }
    sum as f64 / numbers.len() as f64
}

fn median(numbers: &Vec<i32>) -> f64 {
    let mut numbers_sorted = numbers.clone();
    numbers_sorted.sort();
    let middle_number = numbers.len() / 2;
    if middle_number % 2 == 0 {
        return media(&vec![
            numbers_sorted[middle_number],
            numbers_sorted[middle_number - 1],
        ]);
    }
    numbers_sorted[middle_number] as f64
}

fn moda(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for number in numbers {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }
    let mut highest_value = 0;
    let mut hihgest_key = 0;

    for (key, value) in map {
        if value > highest_value {
            highest_value = value;
            hihgest_key = *key
        }
    }
    hihgest_key
}

fn main() {
    let numbers = vec![6, 5, 2, 3, 4, 4, 4, 1, 1, 0];

    println!("Media: {}", media(&numbers));

    println!("Median: {}", median(&numbers));
    println!("Moda: {}", moda(&numbers));
}
