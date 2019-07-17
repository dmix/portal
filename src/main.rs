extern crate tantivy;

mod db;
// use crate::db;
// use portal::Config;
use portal::Config;
use std::env;
use std::process;

fn main() {
    println!("-- PORTAL --");

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match db::init() {
        Ok(database) => {
            // let entries = db::seed().unwrap();

            match portal::run(&config) {
                Ok(contents) => {
                    let entries = portal::parse(&contents);
                    db::add_entries(&database, entries);
                    db::query(&database, "dev");

                    // let results = portal::search(&config.query, &contents);
                    // let result = results.last();
                    // for x in results.iter() {
                    //     println!("> {:?}", x);
                    // }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(err) => println!("Error initializing db! {:?}", err),
    };

    println!("-- DONE --")
}
