#![no_std]

use ::core::fmt::{
	self, Write,
};
use ::rse_tier0::{
	fmt_adapters::AdaptToFmt,
	linked::LinkedTier0,
	Logger, Tier0Spew,
};

pub use ::log;
use log::{
	Log, LevelFilter, Record, Metadata, Level, SetLoggerError,
};

static LOGGER: LinkedTier0Logger = LinkedTier0Logger;
pub fn install_tier0_logger() -> Result<(), SetLoggerError> {
	log::set_logger(&LOGGER)?;
	log::set_max_level(LevelFilter::Trace);
	Ok(())
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinkedTier0Logger;

impl LinkedTier0Logger {
	pub fn log_with<L: for<'a> Logger<&'a str>>(&self, record: &Record, logger: L) {
		let args = record.args();
		let _ = match record.level() {
			Level::Error | Level::Warn => writeln!(logger.fmt_warning(), "{args}"),
			Level::Info => writeln!(logger.fmt_log(), "{args}"),
			Level::Debug => writeln!(logger.fmt_msg(), "{args}"),
			Level::Trace => writeln!(logger.fmt_warning(), "{args}"),
		};
	}
}

impl Log for LinkedTier0Logger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		let _ = metadata;
		true
	}
	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return
		}

		match (record.level(), record.target()) {
			(_, "console") => self.log_with(record, Tier0Spew::<&str>::con_group(&LinkedTier0)),
			(Level::Debug, _) | (Level::Trace, _) | (_, "developer") => {
				self.log_with(record, Tier0Spew::<&str>::dev_group(&LinkedTier0))
			}
			_ => self.log_with(record, Tier0Spew::<&str>::con_group(&LinkedTier0)),
		}
	}
	fn flush(&self) {}
}

