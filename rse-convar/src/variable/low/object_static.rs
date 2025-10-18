use ::core::ffi::{
	CStr, c_float, c_int,
};

use crate::console_base::{
	AsRegistrable, Registrable,
};

use super::{
	super::{
		ConVarParams, GetValue,
	},
	RawVariable, ConVarObject,
};

#[repr(transparent)]
pub struct StaticConVarObject<T> {
	maybe_unparented: ConVarObject<'static, T>,
}

impl<T> StaticConVarObject<T> {
	pub const unsafe fn as_inner(&self) -> &ConVarObject<'static, T> {
		&self.maybe_unparented
	}

	pub const fn as_mut_inner(&mut self) -> &mut ConVarObject<'static, T> {
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

impl<T> StaticConVarObject<T>
where
	T: RawVariable<'static>,
{
	/// # Safety
	/// The [`StaticConVarObject`] must be *pinned* into an area of memory (with e.g. a `static` item).
	pub const unsafe fn new(inner: T, params: ConVarParams<'static>) -> Self {
		Self {
			maybe_unparented: ConVarObject::unparented(inner, params),
		}
	}
}

unsafe impl<T> AsRegistrable for StaticConVarObject<T> {
	fn as_registrable(&mut self) -> Registrable {
		unsafe { self.as_mut_inner().as_mut_con_var() as *mut _ as *mut _ }
	}
}
