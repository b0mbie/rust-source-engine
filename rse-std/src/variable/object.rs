use ::rse_convar::{
	cppdef::ConCommandBase,
	console_base::AsRegistrable,
	variable::{
		low::{
			ConVarObject, StaticConVarObject,
		},
		Variable, params_for,
	},
};

use super::StdVariable;

#[repr(transparent)]
pub struct ConVar<T> {
	con_var: StaticConVarObject<StdVariable<T>>,
}

impl<T> ConVar<T> {
	pub const fn as_inner(&mut self) -> &mut ConVarObject<'static, StdVariable<T>> {
		self.con_var.as_inner()
	}
}

impl<T> ConVar<T>
where
	T: Variable,
{
	pub const fn new(inner: T) -> Self {
		Self {
			con_var: StaticConVarObject::new(StdVariable::new(inner), params_for::<T>())
		}
	}
}

unsafe impl<T> AsRegistrable for ConVar<T> {
	fn as_registrable(&mut self) -> *mut ConCommandBase {
		unsafe { self.as_inner().as_mut_con_var() as *mut _ as *mut _ }
	}
}
