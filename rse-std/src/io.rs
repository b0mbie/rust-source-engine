pub use ::rse_tier0::{
	Color,
	dev_msg, dev_warn, con_msg, con_warn,
};

use ::core::fmt::{
	Write as _,
	Arguments,
};
use ::rse_tier0::{
	fmt_adapters::{
		Log, Message, Warning, ColorMessage,
	},
	linked::spew::{
		LinkedTier0Con, LinkedTier0Dev,
	},
	Logger as LoggerImpl, ColorLogger as ColorLoggerImpl,
};

pub const fn con() -> Logger<LinkedTier0Con> {
	Logger(LinkedTier0Con)
}

pub const fn dev() -> Logger<LinkedTier0Dev> {
	Logger(LinkedTier0Dev)
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Logger<L>(pub L);
impl<L> Logger<L> {
	pub fn msg(&self, args: Arguments<'_>)
	where
		L: for<'a> LoggerImpl<&'a str>,
	{
		let _ = Message(&self.0).write_fmt(args);
	}
	pub fn warn(&self, args: Arguments<'_>)
	where
		L: for<'a> LoggerImpl<&'a str>,
	{
		let _ = Warning(&self.0).write_fmt(args);
	}
	pub fn log(&self, args: Arguments<'_>)
	where
		L: for<'a> LoggerImpl<&'a str>,
	{
		let _ = Log(&self.0).write_fmt(args);
	}
	pub fn color_msg(&self, color: &Color, args: Arguments<'_>)
	where
		L: for<'a> ColorLoggerImpl<&'a str>,
	{
		let _ = ColorMessage {
			logger: &self.0,
			color,
		}.write_fmt(args);
	}

	pub fn msg_raw<T>(&self, t: T)
	where
		L: LoggerImpl<T>,
	{
		self.0.msg(t)
	}
	pub fn warn_raw<T>(&self, t: T)
	where
		L: LoggerImpl<T>,
	{
		self.0.warning(t)
	}
	pub fn log_raw<T>(&self, t: T)
	where
		L: LoggerImpl<T>,
	{
		self.0.log(t)
	}
	pub fn color_msg_raw<T>(&self, color: &Color, t: T)
	where
		L: ColorLoggerImpl<T>,
	{
		self.0.color_msg(color, t)
	}
}

