use ::core::{
	error::Error,
	fmt,
	marker::PhantomData,
};

use crate::Interface;

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