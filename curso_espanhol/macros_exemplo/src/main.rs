macro_rules! tres {
  ($x:expr) => {
    $x + 1
  };
}

fn main() {
  println!("macro: {}", tres!(5));
}
