struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value)
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let new_value = 101;

    // BAD
    /* if new_value < 1 || new_value > 100 {
        panic!("Guess value must be between 1 and 100, got {}", new_value);
    } */

    let _guess = Guess::new(new_value);
}
