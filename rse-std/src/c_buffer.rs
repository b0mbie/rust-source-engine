use ::core::{
	ffi::{
		CStr, c_char, c_float, c_double,
	},
	fmt,
};
use ::libc::snprintf;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct CBuffer<const N: usize> {
	bytes: [u8; N],
}

impl<const N: usize> CBuffer<N> {
	pub const fn new() -> Self {
		Self {
			bytes: [0; _],
		}
	}

	pub const fn capacity(&self) -> usize {
		self.bytes.len()
	}

	pub const fn as_ptr(&self) -> *const c_char {
		self.bytes.as_ptr() as _
	}

	pub const fn as_mut_ptr(&mut self) -> *mut c_char {
		self.bytes.as_mut_ptr() as _
	}

	pub const fn as_c_str(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.as_ptr()) }
	}

	pub fn print_float(&mut self, value: c_float) {
		unsafe {
			snprintf(self.as_mut_ptr(), self.capacity(), c"%f".as_ptr(), value as c_double);
		}
	}
}

impl<const N: usize> fmt::Debug for CBuffer<N> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_c_str().fmt(f)
	}
}
