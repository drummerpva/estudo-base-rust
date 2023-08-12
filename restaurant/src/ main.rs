use std::collections::HashMap;
// use std::fmt;
// use std::io;
use rand::Rng;
use std::fmt::Result;
use std::io::Result as IOResult;

// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
use std::io::{self, Write};

use std::collections::*;

fn main() {
    // let mut map = std::collections::HashMap::new();
    let mut map = HashMap::new();
    map.insert(1, 2);

    let _secret_number = rand::thread_rng().gen_range(1..101);
}

// fn _function1() -> fmt::Result {}
// fn _function2() -> io::Result {}
fn _function1() -> Result {}
fn _function2() -> IOResult {}
