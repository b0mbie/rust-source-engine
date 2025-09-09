use ::core::{
	ffi::{
		CStr, c_char, c_int, c_float,
	},
	ptr::null,
};
use ::rse_cpp::{
	VtObject, virtual_call,
};

pub mod cppdef;
use cppdef::*;

use crate::{
	Tier0GetCommandLine, Tier0CommandLine,
};

use super::LinkedTier0;

pub fn command_line() -> LinkedTier0CommandLine {
	LinkedTier0CommandLine::get()
}

impl Tier0GetCommandLine for LinkedTier0 {
	type CommandLine<'a> = LinkedTier0CommandLine where Self: 'a;
	fn command_line(&self) -> Self::CommandLine<'_> {
		LinkedTier0CommandLine::get()
	}
}

#[derive(Debug)]
#[repr(transparent)]
pub struct LinkedTier0CommandLine(&'static VtObject<CommandLineVt>);
impl LinkedTier0CommandLine {
	pub fn get() -> Self {
		unsafe { Self(VtObject::from_ptr_const(CommandLine_Tier0())) }
	}
}
impl Tier0CommandLine for LinkedTier0CommandLine {
	fn check_parm<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		let mut value = null();
		unsafe {
			virtual_call!(self.0 => check_parm(key.as_ptr(), &mut value));
			c_str_opt(value)
		}
	}

	fn parm_str<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		unsafe { c_str_opt(virtual_call!(self.0 => parm_value_str(key.as_ptr(), null()))) }
	}
	fn parm_str_or<'a>(&'a self, key: &CStr, default: &'a CStr) -> &'a CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.0 => parm_value_str(key.as_ptr(), default.as_ptr()))) }
	}
	fn parm_int_or(&self, key: &CStr, default: c_int) -> c_int {
		unsafe { virtual_call!(self.0 => parm_value_int(key.as_ptr(), default)) }
	}
	fn parm_float_or(&self, key: &CStr, default: c_float) -> c_float {
		unsafe { virtual_call!(self.0 => parm_value_float(key.as_ptr(), default)) }
	}
}

const unsafe fn c_str_opt<'a>(ptr: *const c_char) -> Option<&'a CStr> {
	if !ptr.is_null() {
		unsafe { Some(CStr::from_ptr(ptr)) }
	} else {
		None
	}
}
