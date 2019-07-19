use std::env;
use std::process;

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::PortalConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};

/// `db` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct DBCommand {
    /// To whom are we saying database?
    #[options(free)]
    path: Vec<String>,
}

impl Runnable for DBCommand {
    fn run(&self) {
        let config = app_config();
        println!("Hello, {}!", &config.database.path);

        // if &config.query == "load" {
        //     load_z(&config.filename);
        // }
    }
}

impl config::Override<PortalConfig> for DBCommand {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, mut config: PortalConfig) -> Result<PortalConfig, FrameworkError> {
        if !self.path.is_empty() {
            config.database.path = self.path.join(" ");
        }

        Ok(config)
    }
}

pub fn load_zsh_history() {}
pub fn load_bash_history() {}
// iconv -f UTF-8 -t UTF-8//IGNORE .bash_history > .bash_history-utf8
// iconv -f UTF-8 -t UTF-8//IGNORE .zsh_history > .zsh_history-utf8

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

// pub fn seed() -> tantivy::Result<Vec<Dir>> {
//     // println!("Database SEED");
//
//     let entries = vec![
//         Dir::new("/Users/dmix/dev/_rust/portal", 1557849352),
//         Dir::new("/Users/dmix/dev/_elixir/issues", 1561657040),
//         Dir::new("/Users/dmix/dev/_nim/karax/examples", 1549258325),
//     ];
//
//     Ok(entries)
// }
