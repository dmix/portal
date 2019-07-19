//! `jump` subcommand - example of how to write a subcommand

extern crate tantivy;

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::PortalConfig;
use abscissa_core::{config, Command, FrameworkError, Options, Runnable};

/// `jump` subcommand
///
/// The `Options` proc macro generates an option parser based on the struct
/// definition, and is defined in the `gumdrop` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/gumdrop/>
#[derive(Command, Debug, Options)]
pub struct JumpCommand {
    /// To whom are we saying jump?
    #[options(free)]
    query: Vec<String>,
}

impl Runnable for JumpCommand {
    /// Jump the application.
    fn run(&self) {
        let config = app_config();
        println!("Hello, {}!", &config.jump.query);

        // if &config.query == "search" {
        //     search(&config.query);
        // }
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
