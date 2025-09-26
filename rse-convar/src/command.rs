use ::core::{
	ffi::{
		CStr, c_char,
	},
	fmt,
	mem::size_of,
	slice::{
		from_raw_parts, from_raw_parts_mut,
	},
};

use crate::cppdef::{
	Command as CCommand, COMMAND_MAX_LENGTH,
};

::rse_cpp::transparent_wrapper! {
	pub struct Command for CCommand as "Command";
}

impl Command {
	pub const MAX_COMMAND_LENGTH: usize = COMMAND_MAX_LENGTH - 1;
	
	pub const fn n_args(&self) -> usize {
		self.0.argc as _
	}

	pub const fn args(&self) -> &[Arg] {
		unsafe {
			from_raw_parts(self.0.argv.as_ptr() as *const Arg, self.n_args())
		}
	}

	pub const fn args_mut(&mut self) -> &mut [Arg] {
		unsafe {
			from_raw_parts_mut(self.0.argv.as_mut_ptr() as *mut Arg, self.n_args())
		}
	}

	pub fn arg_string(&self) -> Option<&CStr> {
		if self.0.argc != 0 {
			let c_str_bytes = unsafe {
				from_raw_parts(
					self.0.arg_string_buffer.as_ptr() as *const u8,
					self.0.arg_string_buffer.len() * size_of::<c_char>() / size_of::<u8>(),
				)
			};
			// SAFETY: `arg_string_buffer` contains a valid C string.
			let c_str = unsafe { CStr::from_bytes_until_nul(c_str_bytes).unwrap_unchecked() };
			Some(c_str)
		} else {
			None
		}
	}
}

#[repr(transparent)]
pub struct Arg {
	ptr: *const c_char,
}

impl Arg {
	pub const fn as_c_str(&self) -> &CStr {
		// SAFETY: The inner pointer is always a valid C string.
		unsafe { CStr::from_ptr(self.ptr) }
	}
}

impl fmt::Debug for Arg {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_c_str().fmt(f)
	}
}
