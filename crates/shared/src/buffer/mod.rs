use ::core::{
	ffi::CStr,
	ptr::{
		null, null_mut,
	},
};
use ::rse_cpp::{
	c_str::opt_c_str_as_ptr,
	transparent_wrapper,
};

use crate::cppdef::{
	bf_read, bf_write,
};

mod borrowed;
pub use borrowed::*;

transparent_wrapper! {
	/// # Layout
	/// This type has the exact same layout and ABI as [`bf_read`].
	pub struct BfRead for bf_read as "bf_read";
}

impl BfRead {
	pub const fn empty() -> Self {
		Self::named_empty(None)
	}

	pub const fn named_empty(name: Option<&'static CStr>) -> Self {
		Self(bf_read {
			data: null(),
			data_bytes: 0, data_bits: -1,
			current_bit: 0,
			overflow: false, assert_on_overflow: true,
			debug_name: opt_c_str_as_ptr(name),
		})
	}

	pub const fn reset(&mut self) {
		self.0.current_bit = 0;
		self.0.overflow = false;
	}
}

transparent_wrapper! {
	/// # Layout
	/// This type has the exact same layout and ABI as [`bf_write`].
	pub struct BfWrite for bf_write as "bf_write";
}

impl BfWrite {
	pub const fn empty() -> Self {
		Self::named_empty(None)
	}

	pub const fn named_empty(name: Option<&'static CStr>) -> Self {
		Self(bf_write {
			data: null_mut(),
			data_bytes: 0, data_bits: -1,
			current_bit: 0,
			overflow: false, assert_on_overflow: true,
			debug_name: opt_c_str_as_ptr(name),
		})
	}

	pub const fn reset(&mut self) {
		self.0.current_bit = 0;
		self.0.overflow = false;
	}
}
