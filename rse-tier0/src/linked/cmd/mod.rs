use ::core::{
	ffi::{
		CStr, c_int, c_float,
	},
	num::NonZero,
	ptr::null,
};
use ::rse_cpp::{
	c_str::opt_c_str_from_ptr,
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

impl Tier0CommandLine for LinkedTier0CommandLine {
	fn check_parm<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		let mut value = null();
		unsafe {
			virtual_call!(self.as_object() => check_parm(key.as_ptr(), &mut value));
			opt_c_str_from_ptr(value)
		}
	}

	fn parm_str<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		unsafe { opt_c_str_from_ptr(virtual_call!(self.as_object() => parm_value_str(key.as_ptr(), null()))) }
	}
	fn parm_str_or<'a, 'def: 'a>(&'a self, key: &CStr, default: &'def CStr) -> &'def CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_object() => parm_value_str(key.as_ptr(), default.as_ptr()))) }
	}
	fn parm_int_or(&self, key: &CStr, default: c_int) -> c_int {
		unsafe { virtual_call!(self.as_object() => parm_value_int(key.as_ptr(), default)) }
	}
	fn parm_float_or(&self, key: &CStr, default: c_float) -> c_float {
		unsafe { virtual_call!(self.as_object() => parm_value_float(key.as_ptr(), default)) }
	}

	fn parm_count(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => parm_count()) }
	}
	fn find_parm(&self, s: &CStr) -> Option<NonZero<c_int>> {
		NonZero::new(unsafe { virtual_call!(self.as_object() => find_parm(s.as_ptr())) })
	}
	fn get_parm(&self, index: c_int) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_object() => get_parm(index))) }
	}
	fn has_parm(&self, s: &CStr) -> bool {
		unsafe { virtual_call!(self.as_object() => has_parm(s.as_ptr())) }
	}
}
