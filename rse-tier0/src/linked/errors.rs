use crate::{
	CFormattable, Tier0Errors,
};

use super::{
	cppdef::*,
	LinkedTier0, STR_FORMAT,
};

impl<T: CFormattable> Tier0Errors<T> for LinkedTier0 {
	fn error(&self, t: T) -> ! {
		unsafe { Error(T::FORMAT_STR.as_ptr(), t.into_c_type()) }
	}
}
impl Tier0Errors<&str> for LinkedTier0 {
	fn error(&self, s: &str) -> ! {
		unsafe { Error(STR_FORMAT, s.len(), s.as_ptr()) }
	}
}
