use ::core::{
	ffi::{
		CStr, c_char,
	},
	ptr::null,
};

pub const fn c_str_ptr(s: Option<&CStr>) -> *const c_char {
	match s {
		Some(s) => s.as_ptr(),
		None => null(),
	}
}
