use ::core::fmt;

use crate::{
	Color,
	Logger, ColorLogger,
};

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

#[macro_export]
macro_rules! color_msg {
	($logger:expr, $color_provider:expr) => {
		$crate::Logger::<&str>::color_msg(&$logger, $color_provider, "\n")
	};

	($logger:expr, $color_provider:expr, $($arg:tt)*) => {{
		use ::core::fmt::Write as _;
		let _ = ::core::writeln!($crate::fmt_adapters::AdaptToFmt::fmt_color_msg(&$logger, $color_provider), $($arg)*);
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
	fn fmt_color_msg<C: ColorProvider>(&self, color_provider: C) -> ColorMessage<'_, Self, C> {
		ColorMessage {
			logger: self,
			color_provider,
		}
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
pub struct ColorMessage<'a, L: ?Sized, C: ColorProvider> {
	pub logger: &'a L,
	pub color_provider: C,
}
impl<L: for<'a> ColorLogger<&'a str>, C: ColorProvider> fmt::Write for ColorMessage<'_, L, C> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.logger.color_msg(self.color_provider.get_color(), s);
		Ok(())
	}
}

pub trait ColorProvider {
	fn get_color(&self) -> &Color;
}
impl<C: ColorProvider> ColorProvider for &C {
	fn get_color(&self) -> &Color {
		C::get_color(*self)
	}
}

impl ColorProvider for Color {
	fn get_color(&self) -> &Color {
		self
	}
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConstColor<const R: u8, const G: u8, const B: u8, const A: u8 = 255>;
impl<const R: u8, const G: u8, const B: u8, const A: u8> ColorProvider for ConstColor<R, G, B, A> {
	fn get_color(&self) -> &Color {
		&Color {
			r: R,
			g: G,
			b: B,
			a: A,
		}
	}
}
