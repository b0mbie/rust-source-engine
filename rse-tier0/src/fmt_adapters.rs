use ::core::fmt;

use crate::Logger;

#[macro_export]
macro_rules! msg {
	($logger:expr) => {
		$crate::Logger::<&str>::msg(&$logger, "\n")
	};

	($logger:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::writeln!($crate::fmt_adapters::AdaptToFmt::fmt_msg(&$logger), $($arg)*);
	}};
}

#[macro_export]
macro_rules! warn {
	($logger:expr) => {
		$crate::Logger::<&str>::warning(&$logger, c"\n")
	};

	($logger:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::writeln!($crate::fmt_adapters::AdaptToFmt::fmt_warning(&$logger), $($arg)*);
	}};
}

#[macro_export]
macro_rules! log {
	($logger:expr) => {
		$crate::Logger::<&str>::log(&$logger, c"\n")
	};

	($logger:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::writeln!($crate::fmt_adapters::AdaptToFmt::fmt_log(&$logger), $($arg)*);
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
