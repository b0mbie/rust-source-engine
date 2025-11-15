use ::core::ffi::{
	CStr, c_float, c_int,
};

use crate::console_base::RegistrableMut;

use super::{
	super::{
		ConVarParams, GetValue,
	},
	RawVariable, ConVarObject,
};

#[repr(transparent)]
pub struct StaticConVarObject<'str, T> {
	maybe_unparented: ConVarObject<'str, T>,
}

impl<'str, T> StaticConVarObject<'str, T> {
	pub const unsafe fn from_object(object: ConVarObject<'str, T>) -> Self {
		Self {
			maybe_unparented: object,
		}
	}

	pub fn into_object(self) -> ConVarObject<'str, T> {
		self.maybe_unparented
	}

	pub const fn as_registrable(&mut self) -> RegistrableMut {
		unsafe { self.as_mut_inner().as_mut_raw() as *mut _ as *mut _ }
	}

	pub const unsafe fn as_inner(&self) -> &ConVarObject<'str, T> {
		&self.maybe_unparented
	}

	pub const fn as_mut_inner(&mut self) -> &mut ConVarObject<'str, T> {
		self.maybe_unparented.init_parent();
		&mut self.maybe_unparented
	}

	pub const fn c_str(&self) -> &CStr {
		unsafe { self.as_inner().as_ext().c_str() }
	}

	pub const fn float(&self) -> c_float {
		unsafe { self.as_inner().as_ext().float() }
	}

	pub const fn int(&self) -> c_int {
		unsafe { self.as_inner().as_ext().int() }
	}

	pub fn value<'a, V: GetValue<'a>>(&'a self) -> V {
		unsafe { self.as_inner().as_ext().value() }
	}
}

impl<'str, T> StaticConVarObject<'str, T>
where
	T: RawVariable<'str>,
{
	/// # Safety
	/// The [`StaticConVarObject`] must be *pinned* into an area of memory (with e.g. a `static` item).
	pub const unsafe fn new(inner: T, params: ConVarParams<'str>) -> Self {
		unsafe { Self::from_object(ConVarObject::unparented(inner, params)) }
	}
}
