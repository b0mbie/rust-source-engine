use ::core::{
	ffi::{
		CStr, c_int, c_float,
	},
	ops::DerefMut,
};

pub use ::rse_convar::{
	variable::{
		low::ConVarObject,
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

pub trait Variable<'str> {
	fn on_changed(&mut self, new: NewValue<'_>, old: OldValue<'_>) {
		let _ = new;
		let _ = old;
	}

	/// Allow the implementing type to use the data of [`ConVarObject`]
	/// to properly destroy itself.
	/// 
	/// # Safety
	/// This function must only be called *once*.
	/// This is already done by [`GenericConVar`].
	unsafe fn drop_with_object<T>(object: &mut ConVarObject<'str, T>)
	where
		T: DerefMut<Target = Self>,
	{
		let _ = object;
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
