use ::core::{
	ffi::{
		CStr, c_float, c_double, c_int,
	},
	ptr::null_mut,
};
use ::libc::snprintf;
use ::rse_utl::CString;

macro_rules! print_value {
	($buffer:expr, $format:literal, $value:expr $(,)?) => {
		unsafe {
			const FORMAT: &CStr = $format;
			let buffer: &mut CString = $buffer;
			let value = $value;

			let len = snprintf(null_mut(), 0, FORMAT.as_ptr(), value) as usize;

			let bytes = buffer.alloc_to(len);
			snprintf(bytes.as_mut_ptr() as _, len, FORMAT.as_ptr(), value);
		}
	};
}

pub fn print_float(buffer: &mut CString, value: c_float) {
	print_value!(buffer, c"%f", value as c_double)
}

pub fn print_int(buffer: &mut CString, value: c_int) {
	print_value!(buffer, c"%d", value)
}
