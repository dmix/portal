//! `db` subcommand

use crate::config::PortalConfig;
use crate::prelude::*;
use crate::{database, dir};
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};

#[derive(Command, Debug, Options)]
pub struct DBCommand {
    #[options(free)]
    path: Vec<String>,
}

impl Runnable for DBCommand {
    fn run(&self) {
        let config = app_config();
        let db_path = &config.database.path;
        println!("Initializing DB: {}", &db_path);

        match database::init(&db_path) {
            Ok(database) => dir::load_z(&database, &config.database.z),
            Err(err) => println!("Error: initializing db! {:?}", err),
        };
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
