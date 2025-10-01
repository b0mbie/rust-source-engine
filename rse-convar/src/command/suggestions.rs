use ::core::{
	ffi::c_int,
	hint::assert_unchecked,
};
use ::rse_utl::{
	cppdef::UtlString,
	Vector, CString,
};

use crate::cppdef::COMMAND_COMPLETION_MAX_ITEMS;

#[derive(Debug)]
#[repr(transparent)]
pub struct Suggestions(Vector<UtlString>);
impl Suggestions {
	::rse_cpp::transparent_ref_impls!(Suggestions for Vector<UtlString> as "Vector<UtlString>");

	pub const fn has_capacity(&self) -> bool {
		self.0.len() < COMMAND_COMPLETION_MAX_ITEMS
	}

	pub const fn count(&self) -> SuggestionCount {
		// SAFETY: `has_capacity` ensures that `len` (and therefore `size`) never exceeds `COMMAND_COMPLETION_MAX_ITEMS`.
		unsafe { SuggestionCount::new_unchecked(self.0.as_inner().size) }
	}

	pub fn try_push<S>(&mut self, s: S) -> bool
	where
		S: Into<CString>,
	{
		if self.has_capacity() {
			let c_string: CString = s.into();
			self.0.try_push(c_string.into_inner()).is_ok()
		} else {
			false
		}
	}
}

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

