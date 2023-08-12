use minigrep::Config;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    println!("Usage: minigrep <filename> <query>");
    return;
  }
  let config = Config::new(&args);
  println!(
    "Searching for \"{}\" in \"{}\"",
    config.query, config.filename
  );
  minigrep::run(config)
}
