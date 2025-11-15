use ::rust_alloc::boxed::Box;
use ::core::{
	cell::UnsafeCell,
	ffi::{
		c_float, c_int,
	},
	pin::Pin,
};
use ::rse_convar::{
	console_base::RegistrableMut,
	variable::low::{
		ConVarObject, StaticConVarObject,
	},
};

use super::{
	Variable,
	ConVarParams,
	GetValue,
};

mod wrapper;
use wrapper::*;

pub use wrapper::StdCStrLock as CStrLock;

#[derive(Debug)]
#[repr(transparent)]
pub struct GenericConVar<'str, T> {
	con_var: UnsafeCell<StaticConVarObject<'str, StdVariable<T>>>,
}

unsafe impl<'str, T: Sync> Sync for GenericConVar<'str, T> {}

impl<'str, T> GenericConVar<'str, T> {
	pub fn value<'a, V: GetValue<'a>>(&'a self) -> V {
		V::get_value(self)
	}

	fn with_object_mut<'a, R, F: FnOnce(Pin<&'a mut ConVarObject<'str, StdVariable<T>>>) -> R>(&'a self, f: F) -> R {
		unsafe {
			let object_mut = Pin::new_unchecked((*self.con_var.get()).as_mut_inner());
			f(object_mut)
		}
	}

	pub fn float(&self) -> c_float {
		self.with_object_mut(StdVariable::float)
	}

	pub fn int(&self) -> c_int {
		self.with_object_mut(StdVariable::int)
	}

	pub fn c_str(&self) -> CStrLock<'_> {
		self.with_object_mut(StdVariable::c_str)
	}

	pub fn register(&'static self) -> bool {
		unsafe { crate::con::cvar::register_raw(self.as_registrable()) }
	}

	fn as_registrable(&'static self) -> RegistrableMut {
		unsafe { (*self.con_var.get()).as_registrable() }
	}
}

impl<'str, T> GenericConVar<'str, T>
where
	T: Variable,
{
	/// # Safety
	/// The returned object must be *pinned* into an area of memory (with e.g. a `static` item).
	pub const unsafe fn new(inner: T, params: ConVarParams<'str>) -> Self {
		Self {
			con_var: UnsafeCell::new(unsafe {
				StaticConVarObject::new(StdVariable::new(inner), params)
			}),
		}
	}

	pub fn boxed(inner: T, params: ConVarParams<'str>) -> Pin<Box<Self>> {
		unsafe { Box::pin(Self::new(inner, params)) }
	}
}
