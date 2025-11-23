use ::rust_alloc::boxed::Box;
use ::core::{
	cell::UnsafeCell,
	ffi::{
		CStr, c_float, c_int,
	},
	pin::Pin,
};
use ::rse_convar::{
	console_base::RegistrableMut,
	variable::low::StaticConVarObject,
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

macro_rules! with_object_mut {
	($generic_con_var:expr; |$object_mut:ident| $body:expr) => {{
		let generic_con_var = $generic_con_var;
		unsafe {
			let $object_mut = ::core::pin::Pin::new_unchecked((*generic_con_var.con_var.get()).as_mut_inner());
			$body
		}
	}};
}

impl<'str, T> GenericConVar<'str, T> {
	pub fn value<'a, V: GetValue<'a>>(&'a self) -> V {
		V::get_value(self)
	}

	pub const fn name(&self) -> &'str CStr {
		with_object_mut!(self; |object| object.into_ref().get_ref().name())
	}

	pub const fn default(&self) -> &'str CStr {
		with_object_mut!(self; |object| object.into_ref().get_ref().default())
	}

	pub const fn help(&self) -> Option<&'str CStr> {
		with_object_mut!(self; |object| object.into_ref().get_ref().help())
	}

	pub fn float(&self) -> c_float {
		with_object_mut!(self; |object| StdVariable::float(object))
	}

	pub fn int(&self) -> c_int {
		with_object_mut!(self; |object| StdVariable::int(object))
	}

	pub fn c_str(&self) -> CStrLock<'_> {
		with_object_mut!(self; |object| StdVariable::c_str(object))
	}

	pub fn register(&'static self) -> bool {
		unsafe { crate::con::cvar::register_raw(self.as_registrable()) }
	}

	pub fn as_registrable(&'static self) -> RegistrableMut {
		unsafe { (*self.con_var.get()).as_registrable() }
	}
}

impl<'str, T> GenericConVar<'str, T>
where
	T: Variable<'str>,
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
