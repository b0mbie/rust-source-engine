use ::core::ffi::{
	CStr, c_int, c_float,
};
use ::rse_tier0::{
	linked::cmd::{
		LinkedTier0CommandLine, with_command_line,
	},
	Tier0CommandLine,
};

#[repr(transparent)]
pub struct Parms<'a> {
	cmd: LinkedTier0CommandLine<'a>,
}

impl<'a> Parms<'a> {
	pub fn str(&'a self, key: &CStr) -> Option<&'a CStr> {
		self.cmd.parm_str(key)
	}

	pub fn str_or<'def: 'a>(&self, key: &CStr, default: &'def CStr) -> &'def CStr {
		self.cmd.parm_str_or(key, default)
	}

	pub fn float_or(&self, key: &CStr, default: c_float) -> c_float {
		self.cmd.parm_float_or(key, default)
	}

	pub fn int_or(&self, key: &CStr, default: c_int) -> c_int {
		self.cmd.parm_int_or(key, default)
	}
}

pub fn with_parms<F: FnOnce(Parms<'_>) -> R, R>(f: F) -> R {
	unsafe {
		with_command_line(move |cmd| f(Parms {
			cmd,
		}))
	}
}

/// Returns `true` if the command line contains `parm`.
pub fn has_parm(parm: &CStr) -> bool {
	unsafe { with_command_line(move |c| c.has_parm(parm)) }
}

