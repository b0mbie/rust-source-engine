//! APIs for interacting with Console Variables, or *ConVars*.

use ::core::ffi::{
	CStr, c_float, c_int,
};

use crate::console_base::CvarDllIdentifier;

pub mod low;

/// # Safety
/// `dll_identifier` must return a valid identifier previously returned by
/// `ICvar::AllocateDLLIdentifier`.
pub unsafe trait Variable: Sized {
	const NAME: &CStr;
	const HELP: Option<&CStr>;
	fn on_changed(new: NewValue<'_, Self>, old: OldValue<'_>) {
		let _ = new;
		let _ = old;
	}

	fn dll_identifier(&mut self) -> CvarDllIdentifier;
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct NewValue<'a, T> {
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
