#[derive(Debug)]
enum IpAddr {
    v4(u8, u8, u8, u8),
    v6(String),
}
/* struct IpAddr {
    kind: IpAddrKind,
    address: String,
} */
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        //
    }
}
fn main() {
    let home = IpAddr::v4(127, 0, 0, 1);
    let local = IpAddr::v6(String::from("::1"));
    println!("home: {:#?}, local: {:#?}", home, local);
    route(home);
    route(local);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    if y.is_none() {
        println!("Sem valor");
    }
    if y.is_some() {
        println!("result: {}", x + y.unwrap())
    }

    match y {
        None => println!("Sem valor"),
        Some(alguma_coisa) => println!("result: {}", x + alguma_coisa),
    }
    let result = match y {
        None => 0,
        Some(alguma_coisa) => x + alguma_coisa,
    };
    println!("result: {result}");
}

fn route(ip_kind: IpAddr) {}
