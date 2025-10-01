use ::rse_utl::{
	cppdef::UtlString,
	Vector, CString,
};

use crate::cppdef::COMMAND_COMPLETION_MAX_ITEMS;

use super::SuggestionCount;

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
