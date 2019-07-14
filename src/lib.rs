use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please provide a query argument to search directories");
        }

        let history = String::from("/Users/dmix/.zsh_history-utf8");
        let query = args[1].clone();
        let filename = history;

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(String), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // let contents =
    //     fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    let contents = fs::read_to_string(config.filename)?;

    // println!("Contents {}", contents);
    Ok(contents)
}

// iconv -f UTF-8 -t UTF-8//IGNORE .bash_history > .bash_history-utf8
// iconv -f UTF-8 -t UTF-8//IGNORE .zsh_history > .zsh_history-utf8

fn valid_file(path: &str) -> bool {
    let blacklist = ["/", ".", "./.", "..", "../", "../..", "../../.."];
    let clean_path = &path.replace("~/", "/Users/dmix/");
    if Path::new(clean_path).exists() && !blacklist.contains(&path) {
        // TODO: ignore paths in current directory
        return true;
    }
    return false;
}

pub fn search<'a>(query: &str, contents: &'a String) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            let d: Vec<_> = line.split(' ').collect();
            if d.len() > 2 && valid_file(&d[2]) {
                results.push(d[2]);
            }
        }
    }

    results
}
