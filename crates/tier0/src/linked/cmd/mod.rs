use ::core::{
	ffi::{
		CStr, c_int, c_float,
	},
	marker::PhantomData,
	num::NonZero,
	ptr::null,
};
use ::rse_cpp::{
	c_str::opt_c_str_from_ptr,
	virtual_call,
	RefMut, RefConst,
};

pub mod cppdef;
use cppdef::*;

use crate::{
	Tier0WithCommandLine, Tier0CommandLine,
};

use super::LinkedTier0;

#[repr(transparent)]
pub struct LinkedTier0CommandLine<'a> {
	ptr: RefMut<ICommandLine>,
	_life: PhantomData<&'a ICommandLine>,
}

impl<'a> LinkedTier0CommandLine<'a> {
	pub const fn as_mut_ptr(&mut self) -> RefMut<ICommandLine> {
		self.ptr
	}

	pub const unsafe fn as_ptr(&self) -> RefConst<ICommandLine> {
		self.ptr
	}

	const unsafe fn as_cpp(&self) -> &ICommandLine {
		unsafe { self.as_ptr().as_ref() }
	}
}

pub unsafe fn with_command_line<R, F: FnOnce(LinkedTier0CommandLine<'_>) -> R>(f: F) -> R {
	let cx = LinkedTier0CommandLine {
		ptr: unsafe { CommandLine_Tier0() },
		_life: PhantomData,
	};
	f(cx)
}

impl Tier0WithCommandLine for LinkedTier0 {
	type CommandLine<'a> = LinkedTier0CommandLine<'a> where Self: 'a;
	unsafe fn with_command_line<R, F: FnOnce(Self::CommandLine<'_>) -> R>(&self, f: F) -> R {
		unsafe { with_command_line(f) }
	}
}

impl Tier0CommandLine for LinkedTier0CommandLine<'_> {
	fn check_parm<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		let mut value = null();
		unsafe {
			virtual_call!(self.as_cpp() => check_parm(key.as_ptr(), &mut value));
			opt_c_str_from_ptr(value)
		}
	}

	fn parm_str<'a>(&'a self, key: &CStr) -> Option<&'a CStr> {
		unsafe { opt_c_str_from_ptr(virtual_call!(self.as_cpp() => parm_value_str(key.as_ptr(), null()))) }
	}
	fn parm_str_or<'a, 'def: 'a>(&'a self, key: &CStr, default: &'def CStr) -> &'def CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_cpp() => parm_value_str(key.as_ptr(), default.as_ptr()))) }
	}
	fn parm_int_or(&self, key: &CStr, default: c_int) -> c_int {
		unsafe { virtual_call!(self.as_cpp() => parm_value_int(key.as_ptr(), default)) }
	}
	fn parm_float_or(&self, key: &CStr, default: c_float) -> c_float {
		unsafe { virtual_call!(self.as_cpp() => parm_value_float(key.as_ptr(), default)) }
	}

	fn parm_count(&self) -> c_int {
		unsafe { virtual_call!(self.as_cpp() => parm_count()) }
	}
	fn find_parm(&self, s: &CStr) -> Option<NonZero<c_int>> {
		NonZero::new(unsafe { virtual_call!(self.as_cpp() => find_parm(s.as_ptr())) })
	}
	fn get_parm(&self, index: c_int) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_cpp() => get_parm(index))) }
	}
	fn has_parm(&self, s: &CStr) -> bool {
		unsafe { virtual_call!(self.as_cpp() => has_parm(s.as_ptr())) }
	}
}
