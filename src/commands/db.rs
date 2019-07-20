//! `db` subcommand

// use crate::config::PortalConfig;
use crate::prelude::*;
use crate::{database, dir};
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};
use std::env;

#[derive(Command, Debug, Options)]
pub struct DBCommand {
    #[options(free)]
    command: Vec<String>,
}

impl Runnable for DBCommand {
    fn run(&self) {
        let config = app_config();
        let db_path = &config.database.path;
        let command = &self.command[0];
        let mut dir_path = String::from(env::current_dir().unwrap().to_str().unwrap());
        if self.command.len() > 1 {
            dir_path = String::from(&self.command[1]);
        }

        match database::init(&db_path) {
            Ok(database) => {
                if command == &String::from("init") {
                    println!("Initializing DB: {}", &db_path);
                    dir::load_z(&database, &config.database.z);
                }

                if command == &String::from("track") {
                    println!("Tracking directory: {}, {}", &command, &dir_path);
                    dir::track(&database, &dir_path);
                }
            }
            Err(err) => println!("Error: initializing db! {:?}", err),
        };
    }
}

// impl config::Override<PortalConfig> for DBCommand {
//     fn override_config(&self, mut config: PortalConfig) -> Result<PortalConfig, FrameworkError> {
//         println!("COMMAND = {}", &self.command[0]);
//         if !self.command.is_empty() {
//             config.database.z = "".to_owned();
//         }
//
//         Ok(config)
//     }
// }

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
