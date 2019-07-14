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

    match portal::run(config) {
        Ok(contents) => {
            let results = portal::search("cd ", &contents);
            for x in results.iter() {
                println!("> {}", x);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    println!("-- DONE --")
}
