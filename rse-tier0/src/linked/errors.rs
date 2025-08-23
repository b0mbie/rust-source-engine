use ::printf::IntoFormattable;

use crate::Tier0Errors;

use super::{
	cppdef::*,
	LinkedTier0,
	call_printf,
};

impl<T: IntoFormattable> Tier0Errors<T> for LinkedTier0 {
	fn error(&self, t: T) -> ! {
		unsafe { call_printf!(t => Error) }
	}
}
