//! Portal Config
//!
//! See instructions in `commands.rs` to specify the path to your
//! application's configuration file and/or command-line options
//! for specifying it.

use abscissa_core::Config;
use serde::{Deserialize, Serialize};

/// Portal Configuration
#[derive(Clone, Config, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PortalConfig {
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
    pub query: String,
}

impl Default for JumpSection {
    fn default() -> Self {
        Self {
            query: ".".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DatabaseSection {
    pub path: String,
    pub zsh_history: String,
    pub bash_history: String,
    pub z: String,
}

impl Default for DatabaseSection {
    fn default() -> Self {
        Self {
            path: "".to_owned(),
            zsh_history: "".to_owned(),
            bash_history: "".to_owned(),
            z: "".to_owned(),
        }
    }
}
