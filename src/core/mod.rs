mod arguments;
mod exit;
mod help;
mod license;
mod main;
mod version;

#[cfg(all(unix, test))]
mod tests;

use std::{convert::TryFrom, ffi::OsString};

use crate::core::{
	arguments::{Args, Mode},
	exit::Exit,
};

pub fn run(args: Vec<OsString>) -> Exit {
	match Args::try_from(args) {
		Err(err) => err,
		Ok(args) => {
			match *args.mode() {
				Mode::Help => help::run(),
				Mode::Version => version::run(),
				Mode::License => license::run(),
				Mode::Normal => main::run(&args),
			}
		},
	}
}
