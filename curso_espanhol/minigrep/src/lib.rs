use std::fs;

pub struct Config {
  pub filename: String,
  pub query: String,
}

impl Config {
  pub fn new(args: &[String]) -> Self {
    Self {
      filename: args[1].clone(),
      query: args[2].clone(),
    }
  }
}

pub fn run(config: Config) {
  let file_content = fs::read_to_string(config.filename).expect("File not found");
  let found = search(&config.query, &file_content);
  found.iter().for_each(|line| println!("{}", line));
  // println!("{:?}", found);
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in content.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}
