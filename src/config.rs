//! Portal Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use abscissa_core::Config;
use serde::{Deserialize, Serialize};

// static HISTORY: &'static str = "/Users/dmix/.z";
// static HISTORY: &'static str = "/Users/dmix/.zsh_history-utf8";
//
// pub struct Config {
//     pub query: String,
//     pub filename: String,
// }
//
// impl Config {
//     pub fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 2 {
//             return Err("Please provide a query argument to search directories");
//         }
//
//         let query = args[1].clone();
//         let filename = String::from(HISTORY);
//
//         Ok(Config { query, filename })
//     }
// }
//
// pub fn run(filename: &String) -> Result<(String), Box<dyn Error>> {
//     let contents = fs::read_to_string(&filename)?;
//
//     Ok(contents)
// }

/// Portal Configuration
#[derive(Clone, Config, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PortalConfig {
    /// An example configuration section
    pub jump: JumpSection,
    pub database: DatabaseSection,
}

/// Default configuration settings.
///
/// Note: if your needs are as simple as below, you can
/// use `#[derive(Default)]` on PortalConfig instead.
impl Default for PortalConfig {
    fn default() -> Self {
        Self {
            jump: JumpSection::default(),
            database: DatabaseSection::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JumpSection {
    /// Example configuration value
    pub query: String,
    pub zsh_history: String,
    pub bash_history: String,
    pub z: String,
}

impl Default for JumpSection {
    fn default() -> Self {
        Self {
            query: ".".to_owned(),
            zsh_history: "".to_owned(),
            bash_history: "".to_owned(),
            z: "".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseSection {
    /// Example configuration value
    pub path: String,
}

impl Default for DatabaseSection {
    fn default() -> Self {
        Self {
            path: "".to_owned(),
        }
    }
}
