use ::core::ffi::{
	CStr, c_float, c_int,
};

use crate::console_base::CvarFlags;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConVarValue<'a> {
	pub c_str: &'a CStr,
	pub float: c_float,
	pub int: c_int,
}

impl<'a> ConVarValue<'a> {
	/// Parse a [`CStr`] to properly initialize the [`c_float`] and [`c_int`] values.
	pub fn parse(c_str: &'a CStr) -> Self {
		let ptr = c_str.as_ptr();
		let float = unsafe { ::libc::atof(ptr) as c_float };
		let int = unsafe { ::libc::atoi(ptr) };
		Self {
			c_str,
			float,
			int,
		}
	}
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ConVarParams<'a> {
	pub name: &'a CStr,
	pub default: ConVarValue<'a>,
	pub help: Option<&'a CStr>,
	pub min: Option<c_float>,
	pub max: Option<c_float>,
	pub comp_min: Option<c_float>,
	pub comp_max: Option<c_float>,
	pub flags: CvarFlags,
}

impl<'a> ConVarParams<'a> {
	pub const EMPTY: Self = {
		let empty = c"";
		Self {
			name: empty,
			default: ConVarValue {
				c_str: empty, float: 0.0, int: 0,
			},
			help: None,
			min: None, max: None,
			comp_min: None, comp_max: None,
			flags: CvarFlags::empty(),
		}
	};

	pub const fn simple(name: &'a CStr, default: ConVarValue<'a>) -> Self {
		Self {
			name, default,
			..Self::EMPTY
		}
	}
}
