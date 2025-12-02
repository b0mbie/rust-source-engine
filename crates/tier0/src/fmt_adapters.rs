use ::core::fmt;

use crate::{
	Color,
	Logger, ColorLogger,
};

#[macro_export]
macro_rules! msgln {
	($logger:expr $(,)?) => {
		$crate::Logger::<&::core::ffi::CStr>::msg(&$logger, c"\n")
	};

	($logger:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::writeln!($crate::fmt_adapters::AdaptToFmt::fmt_msg(&$logger), $($arg)*);
	}};
}

#[macro_export]
macro_rules! msg {
	($logger:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::write!($crate::fmt_adapters::AdaptToFmt::fmt_msg(&$logger), $($arg)*);
	}};
}

#[macro_export]
macro_rules! warnln {
	($logger:expr $(,)?) => {
		$crate::Logger::<&::core::ffi::CStr>::warning(&$logger, c"\n")
	};

	($logger:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::writeln!($crate::fmt_adapters::AdaptToFmt::fmt_warning(&$logger), $($arg)*);
	}};
}

#[macro_export]
macro_rules! warn {
	($logger:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::write!($crate::fmt_adapters::AdaptToFmt::fmt_warning(&$logger), $($arg)*);
	}};
}

#[macro_export]
macro_rules! logln {
	($logger:expr $(,)?) => {
		$crate::Logger::<&::core::ffi::CStr>::log(&$logger, c"\n")
	};

	($logger:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::writeln!($crate::fmt_adapters::AdaptToFmt::fmt_log(&$logger), $($arg)*);
	}};
}

#[macro_export]
macro_rules! log {
	($logger:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::write!($crate::fmt_adapters::AdaptToFmt::fmt_log(&$logger), $($arg)*);
	}};
}

#[macro_export]
macro_rules! color_msg {
	($logger:expr => $(($color_provider:expr, $($arg:tt)*)),+ $(,)?) => {{
		use ::core::fmt::Write as _;
		let logger = &$logger;
		$(
			let _ = ::core::write!(
				$crate::fmt_adapters::AdaptToColorFmt::fmt_color_msg(logger, $color_provider),
				$($arg)*
			);
		)+
	}};
}

pub trait AdaptToFmt: for<'a> Logger<&'a str> {
	fn fmt_msg(&self) -> Message<'_, Self> {
		Message(self)
	}
	fn fmt_warning(&self) -> Warning<'_, Self> {
		Warning(self)
	}
	fn fmt_log(&self) -> Log<'_, Self> {
		Log(self)
	}
}
impl<T: for<'a> Logger<&'a str>> AdaptToFmt for T {}

pub trait AdaptToColorFmt: for<'a> ColorLogger<&'a str> {
	fn fmt_color_msg<'c>(&self, color: &'c Color) -> ColorMessage<'_, 'c, Self> {
		ColorMessage {
			logger: self,
			color,
		}
	}
}
impl<T: for<'a> ColorLogger<&'a str>> AdaptToColorFmt for T {}

#[repr(transparent)]
pub struct Message<'a, L: ?Sized>(pub &'a L);
impl<L: for<'a> Logger<&'a str>> fmt::Write for Message<'_, L> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.0.msg(s);
		Ok(())
	}
}

#[repr(transparent)]
pub struct Warning<'a, L: ?Sized>(pub &'a L);
impl<L: for<'a> Logger<&'a str>> fmt::Write for Warning<'_, L> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.0.warning(s);
		Ok(())
	}
}

#[repr(transparent)]
pub struct Log<'a, L: ?Sized>(pub &'a L);
impl<L: for<'a> Logger<&'a str>> fmt::Write for Log<'_, L> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.0.log(s);
		Ok(())
	}
}
pub struct ColorMessage<'l, 'c, L: ?Sized> {
	pub logger: &'l L,
	pub color: &'c Color,
}
impl<L: for<'a> ColorLogger<&'a str>> fmt::Write for ColorMessage<'_, '_, L> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.logger.color_msg(self.color, s);
		Ok(())
	}
}
