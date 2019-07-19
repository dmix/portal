//! `jump` subcommand

use crate::database;
use crate::prelude::*;

use crate::config::PortalConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};

#[derive(Command, Debug, Options)]
pub struct JumpCommand {
    #[options(free)]
    query: Vec<String>,
}

impl Runnable for JumpCommand {
    fn run(&self) {
        let config = app_config();

        match database::init(&config.database.path) {
            Ok(database) => search(&database, &config.jump.query),
            Err(err) => println!("Error initializing db! {:?}", err),
        };
    }
}

impl config::Override<PortalConfig> for JumpCommand {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, mut config: PortalConfig) -> Result<PortalConfig, FrameworkError> {
        if !self.query.is_empty() {
            config.jump.query = self.query.join(" ");
        }

        Ok(config)
    }
}

fn search(database: &database::Database, query: &String) {
    let results = database::query(&database, &query);

    match results.last() {
        Some(dir) => println!("{}", dir.path),
        None => println!("Error: No directories matched {}", &query),
    }
}
