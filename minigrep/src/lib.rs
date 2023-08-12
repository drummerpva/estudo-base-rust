use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    /* let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensetive(&config.query, &content)
    }; */
    let results = match config.case_sensitive {
        true => search(&config.query, &content),
        false => search_case_insensetive(&config.query, &content),
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    // let mut result = Vec::new();
    /* for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result */
}
pub fn search_case_insensetive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
    // let query = query.to_lowercase();
    /* let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result */
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensetive() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensetive(query, contents)
        );
    }
}

/* impl Config {
    fn new(args: &[String]) -> Self {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
} */
