use crate::{
	cppdef::ConCommandBase,
	console_base::AsRegistrable,
};

use super::{
	super::ConVarParams,
	RawVariable, ConVarObject,
};

#[repr(transparent)]
pub struct StaticConVarObject<T> {
	maybe_unparented: ConVarObject<'static, T>,
}

impl<T> StaticConVarObject<T> {
	pub const fn as_inner(&mut self) -> &mut ConVarObject<'static, T> {
		self.maybe_unparented.init_parent();
		&mut self.maybe_unparented
	}
}

impl<T> StaticConVarObject<T>
where
	T: RawVariable<'static>,
{
	pub const fn new(inner: T, params: ConVarParams<'static>) -> Self {
		Self {
			maybe_unparented: ConVarObject::unparented(inner, params),
		}
	}
}

unsafe impl<T> AsRegistrable for StaticConVarObject<T> {
	fn as_registrable(&mut self) -> *mut ConCommandBase {
		unsafe { self.as_inner().as_mut_con_var() as *mut _ as *mut _ }
	}
}
