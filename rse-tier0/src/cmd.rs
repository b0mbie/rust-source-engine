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
