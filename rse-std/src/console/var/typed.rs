use ::core::{
	fmt,
	marker::PhantomData,
};

use super::{
	ConVar, ConVarParams,
	GetValue,
};

#[repr(transparent)]
pub struct TypedConVar<T> {
	inner: ConVar,
	_value_ty: PhantomData<fn() -> T>,
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

	pub fn get(&self) -> T {
		self.inner.value()
	}

	pub fn register(&self) {
		self.inner.register()
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
