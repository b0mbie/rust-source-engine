use ::rust_alloc::boxed::Box;
use ::core::{
	fmt,
	marker::PhantomData,
	pin::Pin,
};
use ::rse_convar::console_base::RegistrableMut;

use super::{
	ConVar, ConVarParams,
	GetValue,
};

#[repr(transparent)]
pub struct TypedConVar<T> {
	inner: ConVar,
	_value_ty: PhantomData<fn() -> T>,
}

impl<T> TypedConVar<T> {
	pub fn register(&'static self) -> bool {
		self.inner.register()
	}

	pub const fn as_registrable(&'static self) -> RegistrableMut {
		self.inner.as_registrable()
	}
}

impl<T> TypedConVar<T>
where
	T: for<'a> GetValue<'a>,
{
	/// # Safety
	/// The [`ConVar`] must be *pinned* into an area of memory (with e.g. a `static` item).
	pub const unsafe fn new(params: ConVarParams<'static>) -> Self {
		Self {
			inner: unsafe { ConVar::new(params) },
			_value_ty: PhantomData,
		}
	}

	pub fn boxed(params: ConVarParams<'static>) -> Pin<Box<Self>> {
		unsafe { Box::pin(Self::new(params)) }
	}

	pub fn get(&self) -> T {
		self.inner.value()
	}
}

impl<T> fmt::Debug for TypedConVar<T>
where
	T: for<'a> GetValue<'a>,
	T: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.get().fmt(f)
	}
}


impl<T> fmt::Display for TypedConVar<T>
where
	T: for<'a> GetValue<'a>,
	T: fmt::Display,
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.get().fmt(f)
	}
}
