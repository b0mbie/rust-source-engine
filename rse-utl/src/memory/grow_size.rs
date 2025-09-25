use ::core::{
	ffi::c_int,
	hint::assert_unchecked,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct GrowSize(c_int);
impl GrowSize {
	pub const DEFAULT: Self = Self::new(0).unwrap();

	pub const fn new(x: c_int) -> Option<Self> {
		if x >= 0 {
			unsafe { Some(Self::new_unchecked(x)) }
		} else {
			None
		}
	}
	
	/// # Safety
	/// `x` must not be negative.
	pub const unsafe fn new_unchecked(x: c_int) -> Self {
		unsafe {
			assert_unchecked(x >= 0);
			Self(x)
		}
	}

	pub const fn get(self) -> c_int {
		self.0
	}
}

impl Default for GrowSize {
	fn default() -> Self {
		Self::DEFAULT
	}
}

impl From<GrowSize> for c_int {
	fn from(value: GrowSize) -> Self {
		value.get()
	}
}
