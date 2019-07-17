extern crate tantivy;

mod db;
// use crate::db;
// use portal::Config;
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
    println!("SEARCHING {}", &query);
    match db::init() {
        Ok(database) => {
            let results = db::query(&database, &query);

            for dir in results {
                println!("{}", dir.path);
            }
        }
        Err(err) => println!("Error initializing db! {:?}", err),
    };
}

fn main() {
    // println!("-- PORTAL --");

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let load = String::from("load");
    let callpixels = String::from("callpixels");

    if &config.query == "load" {
        load_z(&config.filename);
    } else {
        search(&config.query);
    }
}
