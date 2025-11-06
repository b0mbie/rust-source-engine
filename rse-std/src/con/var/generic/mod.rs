use ::core::{
	cell::UnsafeCell,
	ffi::{
		c_float, c_int,
	},
};
use ::rse_convar::{
	console_base::RegistrableMut,
	variable::low::StaticConVarObject,
};

use super::{
	ChangeVariable,
	ConVarParams,
	GetValue,
};

mod wrapper;
use wrapper::*;

pub use wrapper::StdCStrLock as CStrLock;

#[derive(Debug)]
#[repr(transparent)]
pub struct GenericConVar<T> {
	con_var: UnsafeCell<StaticConVarObject<StdVariable<T>>>,
}

unsafe impl<T: Sync> Sync for GenericConVar<T> {}

impl<T> GenericConVar<T> {
	pub fn value<'a, V: GetValue<'a>>(&'a self) -> V {
		V::get_value(self)
	}

	pub fn float(&self) -> c_float {
		unsafe { StdVariable::float((*self.con_var.get()).as_mut_inner()) }
	}

	pub fn int(&self) -> c_int {
		unsafe { StdVariable::int((*self.con_var.get()).as_mut_inner()) }
	}

	pub fn c_str(&self) -> CStrLock<'_> {
		unsafe { StdVariable::c_str((*self.con_var.get()).as_mut_inner()) }
	}

	pub fn register(&self) -> bool {
		unsafe { crate::con::cvar::register_raw(self.as_registrable()) }
	}

	fn as_registrable(&self) -> RegistrableMut {
		unsafe { (*self.con_var.get()).as_registrable() }
	}
}

impl<T> GenericConVar<T>
where
	T: ChangeVariable,
{
	/// # Safety
	/// The returned object must be *pinned* into an area of memory (with e.g. a `static` item).
	pub const unsafe fn new(inner: T, params: ConVarParams<'static>) -> Self {
		Self {
			con_var: UnsafeCell::new(unsafe {
				StaticConVarObject::new(StdVariable::new(inner), params)
			}),
		}
	}
}
