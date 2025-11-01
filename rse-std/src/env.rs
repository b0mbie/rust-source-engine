use ::rse_tier0::{
	linked::cmd::{
		LinkedTier0CommandLine, command_line,
	},
	Tier0CommandLine,
};
use ::std::{
	ffi::{
		CStr, c_int, c_float,
	},
	marker::PhantomData,
	sync::OnceLock,
};

static CMD: OnceLock<&'static LinkedTier0CommandLine> = OnceLock::new();

fn cmd() -> &'static LinkedTier0CommandLine {
	CMD.get_or_init(command_line)
}

#[repr(transparent)]
pub struct Parms<'a> {
	cmd: &'static LinkedTier0CommandLine,
	_life: PhantomData<&'a ()>,
}

impl<'a> Parms<'a> {
	pub fn str(&self, key: &CStr) -> Option<&'a CStr> {
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
	f(Parms {
		cmd: cmd(),
		_life: PhantomData,
	})
}

/// Returns `true` if the command line contains `parm`.
pub fn has_parm(parm: &CStr) -> bool {
	cmd().has_parm(parm)
}

