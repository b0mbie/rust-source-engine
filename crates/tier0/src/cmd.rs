use ::core::{
	ffi::{
		CStr, c_int, c_float,
	},
	num::NonZero,
};

pub trait Tier0GetCommandLine {
	type CommandLine<'a>: Tier0CommandLine where Self: 'a;
	fn command_line(&self) -> Self::CommandLine<'_>;
}

// TODO: Thread safety?
pub trait Tier0CommandLine {
	fn check_parm<'a>(&'a self, key: &CStr) -> Option<&'a CStr>;

	fn parm_str<'a>(&'a self, key: &CStr) -> Option<&'a CStr>;
	fn parm_str_or<'a, 'def: 'a>(&'a self, key: &CStr, default: &'def CStr) -> &'def CStr;
	fn parm_int_or(&self, key: &CStr, default: c_int) -> c_int;
	fn parm_float_or(&self, key: &CStr, default: c_float) -> c_float;

	fn parm_count(&self) -> c_int;
	fn find_parm(&self, s: &CStr) -> Option<NonZero<c_int>>;
	fn get_parm(&self, index: c_int) -> &CStr;
	fn has_parm(&self, s: &CStr) -> bool;
}

impl<T: ?Sized + Tier0CommandLine> Tier0CommandLine for &T {
	fn check_parm<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		T::check_parm(self, key)
	}
	fn parm_str<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		T::parm_str(self, key)
	}
	fn parm_str_or<'a, 'def: 'a>(&'a self, key: &CStr, default: &'def CStr) -> &'def CStr {
		T::parm_str_or(self, key, default)
	}
	fn parm_int_or(&self, key: &CStr, default: c_int) -> c_int {
		T::parm_int_or(self, key, default)
	}
	fn parm_float_or(&self, key: &CStr, default: c_float) -> c_float {
		T::parm_float_or(self, key, default)
	}
	fn parm_count(&self) -> c_int {
		T::parm_count(self)
	}
	fn find_parm(&self, s: &CStr) -> Option<NonZero<c_int>> {
		T::find_parm(self, s)
	}
	fn get_parm(&self, index: c_int) -> &CStr {
		T::get_parm(self, index)
	}
	fn has_parm(&self, s: &CStr) -> bool {
		T::has_parm(self, s)
	}
}
