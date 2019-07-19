//! Portal Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `jump`: jump to a directory matching your query
//! - `version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod jump;
mod version;

use self::{jump::JumpCommand, version::VersionCommand};
use crate::config::PortalConfig;
use abscissa_core::{
    config::Override, Command, Configurable, FrameworkError, Help, Options, Runnable,
};
use std::path::PathBuf;

/// Portal Configuration Filename
pub const CONFIG_FILE: &str = "portal.toml";

/// Portal Subcommands
#[derive(Command, Debug, Options, Runnable)]
pub enum PortalCommand {
    /// The `help` subcommand
    #[options(help = "get usage information")]
    Help(Help<Self>),

    /// The `jump` subcommand
    #[options(help = "jump to a directory")]
    Jump(JumpCommand),

    /// The `version` subcommand
    #[options(help = "display version information")]
    Version(VersionCommand),
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<PortalConfig> for PortalCommand {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        // Check if the config file exists, and if it does not, ignore it.
        // If you'd like for a missing configuration file to be a hard error
        // instead, always return `Some(CONFIG_FILE)` here.
        let filename = PathBuf::from(CONFIG_FILE);

        if filename.exists() {
            Some(filename)
        } else {
            None
        }
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(&self, config: PortalConfig) -> Result<PortalConfig, FrameworkError> {
        match self {
            PortalCommand::Jump(cmd) => cmd.override_config(config),
            _ => Ok(config),
        }
    }
}
