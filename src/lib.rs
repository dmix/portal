// mod db;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

static HOME: &'static str = "/Users/dmix/";
static HISTORY: &'static str = "/Users/dmix/.z";
// static HISTORY: &'static str = "/Users/dmix/.zsh_history-utf8";

#[derive(Debug)]
pub struct Dir {
    pub path: String,
    pub timestamp: u32,
    rank: f32,
}

impl Dir {
    pub fn new(path: &str, timestamp: u32) -> Dir {
        Dir {
            path: String::from(path),
            timestamp,
            rank: 1.0,
        }
    }

    fn parse_z(zpath: String) -> Dir {
        let p: Vec<_> = zpath.split('|').collect();
        let rank: f32 = FromStr::from_str(&p[1]).unwrap();
        let timestamp: u32 = FromStr::from_str(&p[2]).unwrap();

        Dir {
            path: String::from(p[0]),
            timestamp,
            rank,
        }
    }
}

pub struct Config {
    pub query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please provide a query argument to search directories");
        }

        let query = args[1].clone();
        let filename = String::from(HISTORY);

        Ok(Config { query, filename })
    }
}

pub fn run(config: &Config) -> Result<(String), Box<dyn Error>> {
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    let contents = fs::read_to_string(&config.filename)?;
    // println!("Contents {}", contents);

    Ok(contents)
}

// iconv -f UTF-8 -t UTF-8//IGNORE .bash_history > .bash_history-utf8
// iconv -f UTF-8 -t UTF-8//IGNORE .zsh_history > .zsh_history-utf8

fn home(path: &str) -> String {
    path.replace("~/", HOME)
}

fn valid_file(path: &str) -> bool {
    let blacklist = ["/", ".", "./.", "..", "../", "../..", "../../.."];
    if blacklist.contains(&path) {
        return false;
    }

    let clean_path = home(&path);
    if Path::new(&clean_path).exists() {
        // TODO: ignore paths in current directory
        return true;
    }

    return false;
}

pub fn parse<'a>(contents: &'a String) -> Vec<Dir> {
    // let x = db::init;

    let mut results = Vec::new();

    for line in contents.lines() {
        let dir = Dir::parse_z(String::from(line));
        if valid_file(&dir.path) {
            results.push(dir);
        }
    }

    // results.sort_by_key(|x| x.rank as u32);
    results.sort_by_key(|x| x.timestamp);
    results
}

pub fn search<'a>(query: &str, contents: &'a String) -> Vec<Dir> {
    // let x = db::init;

    let mut results = Vec::new();

    for line in contents.lines() {
        let dir = Dir::parse_z(String::from(line));

        if dir.path.contains(query) {
            if valid_file(&dir.path) {
                results.push(dir);
            }
        }
    }

    // results.sort_by_key(|x| x.rank as u32);
    results.sort_by_key(|x| x.timestamp);
    results
}
