use ::core::{
	ffi::{
		c_float, c_int,
	},
	fmt::Write,
};
use ::rse_utl::CString;

use crate::fmt_util::{
	SliceWrite, formatted_len,
};

macro_rules! print_value {
	($buffer:expr, $($arg:tt)*) => {{
		let buffer: &mut CString = $buffer;

		let args = ::core::format_args!($($arg)*);
		let len = formatted_len(args);

		unsafe {
			let bytes = buffer.alloc_to(len);
			let _ = SliceWrite(bytes).write_fmt(args);
		}
	}};
}

pub fn print_float_to_utl(buffer: &mut CString, value: c_float) {
	print_value!(buffer, "{value}")
}

pub fn print_int_to_utl(buffer: &mut CString, value: c_int) {
	print_value!(buffer, "{value}")
}