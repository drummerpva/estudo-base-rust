fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using yout favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    let v = vec!["a", "b", "c", "d"];
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    let (x, y, z) = (1, 2, 3);
    println!("x:{x}, y:{y}, z:{z}");

    fn print_corordinates((x, y): &(i32, i32)) {
        println!("Current locations: x = {x}, y = {y}");
    }
    let point = (3, 5);
    print_corordinates(&point);

    //when pattern might fail
    let some_value: Option<&str> = None;
    // let Some(a) = some_value;
    let result = if let Some(a) = some_value {
        a
    } else {
        "invalid"
    };
    println!("result: {result}");

    //patter syntax
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    let y = Some(5);
    let z = 10;

    match y {
        Some(50) => println!("Got 50"),
        Some(z) => println!("Matched, z = {:?}", z),
        _ => println!("Default case, y ={:?}", y),
    }
    println!("at the end: y = {:?} and z = {:?}", y, z);

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one throung five"),
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 3, y: 4 };
    let Point { x, y } = p;
    assert_eq!(3, x);
    assert_eq!(4, y);

    let p = Point { x: 7, y: 3 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the z axis at {y}"),
        Point { x, y } => println!("On neither axis, X:{x}, Y:{y}"),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}")
        }
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g} and blue {b}")
        }
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!("H:{h}, S:{s}, V:{v}"),
        Message2::ChangeColor(Color::Rgb(r, g, b)) => println!("R:{r}, G:{g}, B:{b}"),
        _ => println!("No change color"),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {feet}, inches: {inches}, x: {x}, y: {y}");

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {y}");
    }
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string");
    }
    println!("s:{:?}", s);

    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point3 { x: 0, y: 0, z: 0 };
    match origin {
        Point3 { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 6, 8, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32);

    match numbers {
        (first, .., second_last, last) => {
            println!("Some numbers: {first}, {second_last}, {last}");
        }
    }

    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("the number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum Message3 {
        Hello { id: i32 },
    }
    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        Message3::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message3::Hello { id } => println!("Found some other id: {id}"),
    }
}
