#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello, {}", name)
}

pub fn make_panic(text: &str) -> bool {
    panic!("{}", text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
        assert!(1 == 1);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 7,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
        assert_eq!(larger.can_hold(&smaller), true);
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 7,
            height: 6,
        };

        assert!(smaller.can_hold(&larger) == false);
        assert_ne!(smaller.can_hold(&larger), true);
    }

    #[test]
    fn ensure_add_two_succeeds() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Douglas");
        assert!(
            result.contains("Douglas"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }
    #[test]
    #[should_panic(expected = "An panic")]
    fn verifying_panic() {
        make_panic("An panic");
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    /*
    #[test]
    fn another() {
        panic!("Make this test fail");
    } */

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn add_two_and_tree() {
        assert_eq!(5, add_two(3));
    }
    //cargo test add - will execute these two tests
    #[test]
    #[ignore] //cargo test -- --ignored or --include-ignored
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
    //cargo test one_hundred
}
