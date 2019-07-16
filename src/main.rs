extern crate tantivy;

mod db;
// use crate::db;
// use portal::Config;
use portal::Config;
use std::env;
use std::process;

fn main() {
    // println!("-- PORTAL --");
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match db::init() {
        Ok((schema, index_path)) => match db::seed(&schema, &index_path) {
            Ok(index) => {
                // db::read(&schema, &index);
            }
            Err(_) => println!("Error seeding db!"),
        },
        Err(_) => println!("Error initializing db!"),
    };

    match portal::run(&config) {
        Ok(contents) => {
            let results = portal::search(&config.query, &contents);
            let result = results.last();
            println!("{}", result.unwrap().path);
            // for x in results.iter() {
            //     println!("> {:?}", x);
            // }
        }
        Err(e) => println!("Error: {}", e),
    }
    //
    // println!("-- DONE --")
}
