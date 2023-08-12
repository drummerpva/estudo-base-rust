// type clousure_to_returns = dyn Fn(i32) -> i32;
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
fn main() {}
