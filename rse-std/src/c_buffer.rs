use ::core::{
	ffi::{
		CStr, c_char, c_float, c_int, c_double,
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

	pub const fn len(&self) -> usize {
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

	pub const fn set(&mut self, value: &CStr) {
		let bytes = value.to_bytes();

		let mut i = 0;
		let i_max = if bytes.len() <= (N - 1) {
			bytes.len()
		} else {
			N - 1
		};
		while i < i_max {
			self.bytes[i] = bytes[i];
			i += 1;
		}
		self.bytes[i] = 0;
	}
	
	pub fn print_float(&mut self, value: c_float) {
		unsafe {
			snprintf(self.as_mut_ptr(), self.len(), c"%f".as_ptr(), value as c_double);
		}
	}

	pub fn print_int(&mut self, value: c_int) {
		unsafe {
			snprintf(self.as_mut_ptr(), self.len(), c"%d".as_ptr(), value);
		}
	}
}

impl<const N: usize> fmt::Debug for CBuffer<N> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.as_c_str().fmt(f)
	}
}
