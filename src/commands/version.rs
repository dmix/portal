//! `version` subcommand

#![allow(clippy::never_loop)]

use super::PortalCommand;
use abscissa_core::{Command, Options, Runnable};

#[derive(Command, Debug, Default, Options)]
pub struct VersionCommand {}

impl Runnable for VersionCommand {
    fn run(&self) {
        println!("{} {}", PortalCommand::name(), PortalCommand::version());
    }
}
