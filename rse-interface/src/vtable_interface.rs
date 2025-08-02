use ::core::{
	error::Error,
	fmt,
};
use ::rse_cpp::VtObject;

use crate::{
	Interface, RawInterface, FromRawInterface,
};

pub trait VTableInterface: Interface {
	type VTable;
	/// # Safety
	/// `ptr` must be a valid pointer to an interface identified by the implementing type's
	/// [`IDENTIFIER`](Interface::IDENTIFIER).
	unsafe fn from_ptr(ptr: VtObject<Self::VTable>) -> Self;
}
unsafe impl<T: VTableInterface> FromRawInterface for T {
	type Code = ();
	const INITIAL_CODE: Self::Code = ();
	unsafe fn from_raw_interface(raw_interface: RawInterface, _: Self::Code) -> Self {
		unsafe { T::from_ptr(raw_interface.cast()) }
	}

	type Error = InterfaceError;
	fn convert_error(_: Self::Code) -> Self::Error {
		InterfaceError
	}
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InterfaceError;
impl fmt::Display for InterfaceError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("failed to create interface")
	}
}
impl Error for InterfaceError {}
