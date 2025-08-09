use ::core::fmt::{
	Arguments, Write,
	Result,
};
use ::rse_tier0::{
	Logger, ColorLogger,
};

use crate::Color;

pub trait Printer<T: PrintTo<Self>> {
	fn print(&self, t: T) {
		t.print_to(self)
	}
}
impl<P, T: PrintTo<P>> Printer<T> for P {}

pub trait PrintTo<P: ?Sized> {
	fn print_to(self, printer: &P);
}

impl<P> PrintTo<P> for Arguments<'_>
where
	P: for<'a> DirectPrinter<&'a str> + DirectPrinter<char>,
{
	fn print_to(self, printer: &P) {
		struct FmtDirectPrinter<'a, P: ?Sized>(pub &'a P);
		impl<P: ?Sized> Write for FmtDirectPrinter<'_, P>
		where
			P: for<'a> DirectPrinter<&'a str> + DirectPrinter<char>,
		{
			fn write_str(&mut self, s: &str) -> Result {
				self.0.direct_print(s);
				Ok(())
			}

			fn write_char(&mut self, c: char) -> Result {
				self.0.direct_print(c);
				Ok(())
			}
		}

		let _ = FmtDirectPrinter(printer).write_fmt(self);
	}
}


pub trait DirectPrinter<T> {
	fn direct_print(&self, t: T);
	fn direct_color_print(&self, color: &Color, t: T) {
		let _ = color;
		self.direct_print(t)
	}
}

impl<T, L: ?Sized + Logger<T> + ColorLogger<T>> DirectPrinter<T> for L {
	fn direct_print(&self, t: T) {
		self.msg(t)
	}
	fn direct_color_print(&self, color: &Color, t: T) {
		self.color_msg(color, t)
	}
}
