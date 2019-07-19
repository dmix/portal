use std::path::Path;
use std::str::FromStr;

static HOME: &'static str = "/Users/dmix/";

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
    // results.sort_by_key(|x| x.timestamp);
    results
}
