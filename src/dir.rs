use crate::database;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use std::time::{SystemTime, UNIX_EPOCH};
static HOME: &'static str = "/Users/dmix/";

#[derive(Debug)]
pub struct Dir {
    pub path: String,
    pub timestamp: u32,
    pub rank: f32,
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

pub fn open_z(filename: &String) -> Result<(String), Box<dyn Error>> {
    let contents = fs::read_to_string(&filename)?;

    Ok(contents)
}

pub fn parse_z<'a>(contents: &'a String) -> Vec<Dir> {
    // let x = db::init;

    let mut results = Vec::new();

    for line in contents.lines() {
        let dir = Dir::parse_z(String::from(line));
        if valid_file(&dir.path) {
            results.push(dir);
        }
    }

    // results.sort_by_key(|x| x.rank as u32);
    // results.sort_by_key(|x| x.timestamp);
    results
}

pub fn load_z(database: &database::Database, filename: &String) {
    match open_z(filename) {
        Ok(contents) => {
            let entries = parse_z(&contents);
            println!("Loading {} entries into database", entries.len());
            database::add_entries(&database, entries);
        }
        Err(e) => println!("Error: {}", e),
    }
}

// iconv -f UTF-8 -t UTF-8//IGNORE .bash_history > .bash_history-utf8
// iconv -f UTF-8 -t UTF-8//IGNORE .zsh_history > .zsh_history-utf8

pub fn track(database: &database::Database, dir_path: &String) {
    match Path::new(&dir_path).exists() {
        true => {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let entries = vec![Dir::new(&dir_path, time.as_secs() as u32)];
            database::add_entries(&database, entries);
        }
        false => println!("Directory path doesn't exist!: {}", &dir_path),
    }
}
