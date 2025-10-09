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
	Command, COMMAND_MAX_LENGTH,
};

::rse_cpp::transparent_wrapper! {
	/// Transparent wrapper for a parsed command invocation (`CCommand`).
	/// 
	/// # Layout
	/// This type has the exact same layout and ABI as [`Command`].
	pub struct Invocation for Command as "Command";
}

impl Invocation {
	pub const MAX_COMMAND_LENGTH: usize = COMMAND_MAX_LENGTH - 1;
	
	/// Returns the number of arguments.
	pub const fn n_args(&self) -> usize {
		self.0.argc as _
	}

	/// Returns an iterator over [`CStr`]s of arguments.
	/// 
	/// This is a convenience function for iterating over [`args`](Self::args)
	/// while mapping them to C strings.
	pub fn iter(&self) -> ArgIter<'_> {
		Arg::iter(self.args())
	}

	/// Returns an immutable slice of all the [`Arg`]s.
	pub const fn args(&self) -> &[Arg] {
		unsafe {
			from_raw_parts(self.0.argv.as_ptr() as *const Arg, self.n_args())
		}
	}

	/// Returns a mutable slice of all the [`Arg`]s.
	pub const fn args_mut(&mut self) -> &mut [Arg] {
		unsafe {
			from_raw_parts_mut(self.0.argv.as_mut_ptr() as *mut Arg, self.n_args())
		}
	}

	/// Returns the whole argument string, if available.
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

/// Transparent wrapper around an [`Invocation`] argument C string.
#[repr(transparent)]
pub struct Arg {
	ptr: *const c_char,
}

impl Arg {
	/// Returns the [`CStr`] that is stored in this argument.
	pub const fn as_c_str(&self) -> &CStr {
		// SAFETY: The inner pointer is always a valid C string.
		unsafe { CStr::from_ptr(self.ptr) }
	}

	/// Returns an iterator over [`Arg`] that maps each one to a [`CStr`].
	pub fn iter(slice: &[Arg]) -> ArgIter<'_> {
		ArgIter {
			inner: slice.iter(),
		}
	}
}

impl fmt::Debug for Arg {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_c_str().fmt(f)
	}
}

/// Iterator over [`Arg`]s that maps them to [`CStr`]s.
#[repr(transparent)]
pub struct ArgIter<'a> {
	inner: ::core::slice::Iter<'a, Arg>,
}
impl<'a> Iterator for ArgIter<'a> {
	type Item = &'a CStr;
	fn next(&mut self) -> Option<Self::Item> {
		let arg = self.inner.next()?;
		Some(arg.as_c_str())
	}
}
