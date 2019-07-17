extern crate tantivy;

mod db;
use portal::Config;
use std::env;
use std::process;

fn load_z(filename: &String) {
    match db::init() {
        Ok(database) => match portal::run(filename) {
            Ok(contents) => {
                let entries = portal::parse(&contents);
                db::add_entries(&database, entries);
            }
            Err(e) => println!("Error: {}", e),
        },
        Err(err) => println!("Error initializing db! {:?}", err),
    };
}

fn search(query: &String) {
    match db::init() {
        Ok(database) => {
            let results = db::query(&database, &query);

            match results.last() {
                Some(dir) => println!("{}", dir.path),
                None => println!("."),
            }
        }
        Err(err) => println!("Error initializing db! {:?}", err),
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if &config.query == "load" {
        load_z(&config.filename);
    } else {
        search(&config.query);
    }
}
