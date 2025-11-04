use ::core::ffi::{
	CStr, c_float, c_int,
};

use crate::console_base::CvarFlags;

use super::{
	ConVarParams, ConVarValue,
};

pub const fn params_for<T>() -> ConVarParams<'static>
where
	T: Variable,
{
	ConVarParams {
		name: T::NAME,
		default: T::DEFAULT,
		help: T::HELP,
		min: T::MIN, max: T::MAX,
		comp_min: T::MIN, comp_max: T::MAX,
		flags: T::FLAGS,
	}
}

pub trait Variable: ChangeVariable {
	const NAME: &CStr;
	const HELP: Option<&CStr> = None;
	const FLAGS: CvarFlags = CvarFlags::empty();

	const DEFAULT: ConVarValue<'static>;

	const MIN: Option<c_float> = None;
	const MAX: Option<c_float> = None;
	const COMP_MIN: Option<c_float> = None;
	const COMP_MAX: Option<c_float> = None;
}

pub trait ChangeVariable {
	fn on_changed(new: NewValue<'_, Self>, old: OldValue<'_>) {
		let _ = new;
		let _ = old;
	}
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct NewValue<'a, T: ?Sized> {
	pub inner: &'a mut T,
	pub c_str: &'a CStr,
	pub float: c_float,
	pub int: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OldValue<'a> {
	pub c_str: &'a CStr,
	pub float: c_float,
}
