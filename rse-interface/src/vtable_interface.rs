use ::core::{
	error::Error,
	fmt,
	marker::PhantomData,
};
use ::rse_cpp::{
	OwnedVtObjectWrapper, VtObjectPtr,
};

use crate::{
	Interface, RawInterface, FromRawInterface,
};

#[diagnostic::on_unimplemented(message = "`{Self}` is not a vtable-based Source Engine interface")]
pub trait VTableInterface: Interface {
	type VTable;
	/// # Safety
	/// `ptr` must be a valid pointer to an interface identified by the implementing type's
	/// [`IDENTIFIER`](Interface::IDENTIFIER).
	unsafe fn from_ptr(ptr: VtObjectPtr<Self::VTable>) -> Self;
}

#[diagnostic::do_not_recommend]
impl<T: Interface + OwnedVtObjectWrapper> VTableInterface for T {
	type VTable = T::VTable;
	unsafe fn from_ptr(ptr: VtObjectPtr<Self::VTable>) -> Self {
		unsafe { T::from_ptr(ptr) }
	}
}

#[diagnostic::do_not_recommend]
unsafe impl<T: VTableInterface> FromRawInterface for T {
	type Code = ();
	const INITIAL_CODE: Self::Code = ();
	unsafe fn from_raw_interface(raw_interface: RawInterface, _: Self::Code) -> Self {
		unsafe { T::from_ptr(raw_interface.cast()) }
	}

	type Error = InterfaceError<Self>;
	fn convert_error(_: Self::Code) -> Self::Error {
		InterfaceError::new()
	}
}

#[repr(transparent)]
pub struct InterfaceError<T>(PhantomData<fn(&T)>);

impl<T: Interface> fmt::Display for InterfaceError<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("failed to create interface ")?;
		fmt::Debug::fmt(T::IDENTIFIER, f)
	}
}
impl<T: Interface> Error for InterfaceError<T> {}

impl<T> InterfaceError<T> {
	pub const fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T> Default for InterfaceError<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> fmt::Debug for InterfaceError<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("InterfaceError")
	}
}
