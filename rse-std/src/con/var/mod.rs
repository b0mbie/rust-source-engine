use ::core::ffi::{
	CStr, c_int, c_float,
};

pub use ::rse_convar::{
	variable::{
		ConVarParams, ConVarValue,
	},
	cvar_value,
};

mod dynamic;
pub use dynamic::*;
mod generic;
pub use generic::*;
mod get_value;
pub use get_value::*;
mod typed;
pub use typed::*;

pub trait Variable {
	fn on_changed(&mut self, new: NewValue<'_>, old: OldValue<'_>) {
		let _ = new;
		let _ = old;
	}
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct NewValue<'a> {
	pub c_str: &'a CStr,
	pub float: c_float,
	pub int: c_int,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OldValue<'a> {
	pub c_str: &'a CStr,
	pub float: c_float,
}
