#![recursion_limit = "1024"]

#![allow(unused)]
#![feature(proc_macro)]

extern crate rand;
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate tempdir;
extern crate url;
extern crate walkdir;
extern crate json;
extern crate semver;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate tar;
extern crate flate2;
extern crate toml;
#[macro_use]
extern crate scopeguard;
extern crate libc;

#[macro_use]
mod log;
mod errors;
mod toolchain;
mod compare;
mod registry;
mod lists;
mod file;
mod dl;
mod gh;
mod util;
mod run;
mod crates;
mod git;
mod checkpoint;
mod ex;
mod ex_run;
mod toml_frobber;
mod model;
mod gh_mirrors;
mod report;
mod docker;
mod dirs;
mod bmk;
mod job;
mod blobject;
mod home;

use clap::{App, Arg, AppSettings, SubCommand, ArgMatches};
use errors::*;
use std::panic;
use std::env;
use std::process;
use dirs::*;

fn main() {
    log::init();
    let success = match panic::catch_unwind(main_) {
        Ok(Ok(())) => {
            true
        }
        Ok(Err(e)) => {
            use std::error::Error;
            util::report_error(&e);
            false
        }
        Err(e) => {
            util::report_panic(&*e);
            false
        }
    };
    log!("{}", if success { "command succeeded" } else { "command failed" });
    log::finish();
    process::exit(if success { 0 } else { 1 });
}

fn main_() -> Result<()> {
    let ref matches = cli().get_matches();

    run_cmd(matches)?;

    Ok(())
}

fn cli() -> App<'static, 'static> {
    App::new("cargobomb")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Kaboom!")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::SubcommandRequiredElseHelp)

        .subcommands(model::conv::clap_cmds())
}

fn run_cmd(m: &ArgMatches) -> Result<()> {
    let cmd = model::conv::clap_args_to_cmd(m)?;
    let state = model::state::GlobalState::init();
    let _ = bmk::run(state, cmd)?;

    Ok(())
}

