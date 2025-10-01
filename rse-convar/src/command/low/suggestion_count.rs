use ::core::{
	ffi::c_int,
	hint::assert_unchecked,
};

use crate::cppdef::COMMAND_COMPLETION_MAX_ITEMS;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SuggestionCount(c_int);
impl SuggestionCount {
	pub const ZERO: Self = Self(0);
	pub const ONE: Self = Self(1);
	pub const MAX: Self = Self(Self::MAX_INT);

	const MAX_INT: c_int = COMMAND_COMPLETION_MAX_ITEMS as _;

	pub const fn new(n: c_int) -> Option<Self> {
		if n <= Self::MAX_INT {
			unsafe { Some(Self::new_unchecked(n)) }
		} else {
			None
		}
	}

	/// # Safety
	/// `n` must be less than or equal to [`COMMAND_COMPLETION_MAX_ITEMS`].
	pub const unsafe fn new_unchecked(n: c_int) -> Self {
		unsafe {
			assert_unchecked(n <= Self::MAX_INT);
			Self(n)
		}
	}

	pub const fn get(self) -> c_int {
		self.0
	}
}

impl From<SuggestionCount> for c_int {
	fn from(value: SuggestionCount) -> Self {
		value.0
	}
}
