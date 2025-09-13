use ::core::{
	ffi::{
		CStr, c_char, c_int, c_float,
	},
	ptr::null,
};
use ::rse_cpp::{
	VtObject, virtual_call,
	vt_object_wrapper,
	AsObject,
};

pub mod cppdef;
use cppdef::*;

use crate::{
	Tier0GetCommandLine, Tier0CommandLine,
};

use super::LinkedTier0;

vt_object_wrapper! {
	pub struct LinkedTier0CommandLine for CommandLineVt;
}
impl AsObject<CommandLineVt> for &LinkedTier0CommandLine {
	fn as_object(&self) -> &VtObject<CommandLineVt> {
		&self.object
	}
}

impl LinkedTier0CommandLine {
	pub fn get() -> &'static Self {
		unsafe { CommandLine_Tier0().cast().as_ref() }
	}
}

pub fn command_line() -> &'static LinkedTier0CommandLine {
	LinkedTier0CommandLine::get()
}

impl Tier0GetCommandLine for LinkedTier0 {
	type CommandLine<'a> = &'static LinkedTier0CommandLine where Self: 'a;
	fn command_line(&self) -> Self::CommandLine<'_> {
		LinkedTier0CommandLine::get()
	}
}

impl<T: AsObject<CommandLineVt>> Tier0CommandLine for T {
	fn check_parm<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		let mut value = null();
		unsafe {
			virtual_call!(self.as_object() => check_parm(key.as_ptr(), &mut value));
			c_str_opt(value)
		}
	}

	fn parm_str<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		unsafe { c_str_opt(virtual_call!(self.as_object() => parm_value_str(key.as_ptr(), null()))) }
	}
	fn parm_str_or<'a>(&'a self, key: &CStr, default: &'a CStr) -> &'a CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_object() => parm_value_str(key.as_ptr(), default.as_ptr()))) }
	}
	fn parm_int_or(&self, key: &CStr, default: c_int) -> c_int {
		unsafe { virtual_call!(self.as_object() => parm_value_int(key.as_ptr(), default)) }
	}
	fn parm_float_or(&self, key: &CStr, default: c_float) -> c_float {
		unsafe { virtual_call!(self.as_object() => parm_value_float(key.as_ptr(), default)) }
	}
}

const unsafe fn c_str_opt<'a>(ptr: *const c_char) -> Option<&'a CStr> {
	if !ptr.is_null() {
		unsafe { Some(CStr::from_ptr(ptr)) }
	} else {
		None
	}
}
