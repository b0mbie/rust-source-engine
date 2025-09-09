use ::core::ffi::{
	CStr, c_int, c_float,
};

pub trait Tier0GetCommandLine {
	type CommandLine<'a>: Tier0CommandLine where Self: 'a;
	fn command_line(&self) -> Self::CommandLine<'_>;
}

pub trait Tier0CommandLine {
	fn check_parm<'a>(&'a self, key: &CStr) -> Option<&'a CStr>;

	fn parm_str<'a>(&'a self, key: &CStr) -> Option<&'a CStr>;
	fn parm_str_or<'a>(&'a self, key: &CStr, default: &'a CStr) -> &'a CStr;
	fn parm_int_or(&self, key: &CStr, default: c_int) -> c_int;
	fn parm_float_or(&self, key: &CStr, default: c_float) -> c_float;
}
