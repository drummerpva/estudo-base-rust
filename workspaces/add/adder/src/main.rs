use add_one;
use add_two;
use rand;
fn main() {
    let num = 10;
    println!("Hello, world {} plus one is {}", num, add_one::add_one(num));
    let num2 = 10;
    println!(
        "Hello, world {} plus two is {}",
        num2,
        add_two::add_two(num2)
    );
}
