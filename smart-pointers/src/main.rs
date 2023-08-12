enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;

use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPoint {
    data: String,
}
impl Drop for CustomSmartPoint {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data`{}`", self.data);
    }
}

fn main() {
    // let b = Box::new(5);
    // println!("b = {b}");
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let a = 5;
    let b = Box::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);

    let c = 5;
    let d = MyBox::new(c);
    assert_eq!(5, c);
    assert_eq!(5, *d);
    assert_eq!(5, *(d.deref()));

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    let _e = CustomSmartPoint {
        data: String::from("my stuff"),
    };
    //_e.drop(); //not allowed
    drop(_e);
    let _f = CustomSmartPoint {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created");
    //----------------------
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
