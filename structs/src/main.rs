#[derive(Debug)]
struct Person {
    id: u64,
    active: bool,
    username: String,
    email: String,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let mut person1 = Person {
        active: true,
        username: String::from("douglaspoma"),
        email: String::from("douglaspoma@yahoo.com"),
        id: 37,
    };
    let person2 = create_user(
        String::from("douglaspoma"),
        String::from("douglaspoma@yahoo.com"),
        true,
        person1.id + 1,
    );
    let person3 = Person {
        active: false,
        ..person1
    };
    person1.email = String::from("douglas.poma@registrallogistica.com.br");
    // println!("{:?}", person1);
    println!("{:#?}", person2);
    println!("{:#?}", person3);

    //struct de tupla
    #[derive(Debug)]
    struct Color(u32, String);
    let azul = Color(100, String::from("Azul"));
    println!("{:#?}", azul);

    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };
    let rect2 = Rectangle {
        height: 10,
        width: 40,
    };
    // dbg!(&rect1);
    println!("can Rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("area: {}", rect1.area());
    if rect1.width() {
        println!("The width of rect1 is: {}", rect1.width);
    }

    let square = Rectangle::square(10);
    dbg!(&square);
    println!("The area of square is: {}", square.area());
}

fn create_user(username: String, email: String, active: bool, id: u64) -> Person {
    return Person {
        active,
        username,
        email,
        id,
    };
}
