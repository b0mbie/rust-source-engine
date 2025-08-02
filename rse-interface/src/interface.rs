use ::core::ffi::CStr;

use crate::cppdef::{
	CreateInterfaceFn, ReturnCode, RawInterface,
};

/// Trait for interfaces that are identified by a C string.
/// 
/// # Safety
/// `IDENTIFIER` must be valid to be used to query for a value of the implementing type.
pub unsafe trait Interface {
	const IDENTIIFER: &CStr;
}

/// # Safety
/// The [`RawInterface`] that is used to instantiate values of this type is valid for an arbitrary period of time,
/// and the implementation that uses it must be sound.
pub unsafe trait FromRawInterface: Interface {
	type Code: InterfaceCode;
	const INITIAL_CODE: Self::Code;

	/// # Safety
	/// `raw_interface` must be a pointer that was queried with the implementing type's `IDENTIFIER`.
	unsafe fn from_raw_interface(raw_interface: RawInterface, code: Self::Code) -> Self;
	
	type Error;
	fn convert_error(code: Self::Code) -> Self::Error;
}

pub trait ToRawInterface: Interface {
	/// # Safety
	/// The returned [`RawInterface`] is valid only for the duration specified by the implementing interface.
	unsafe fn to_raw_interface(&mut self) -> RawInterface;
}

pub trait InterfaceCode {
	fn as_out_return_code(&mut self) -> Option<&mut ReturnCode>;
}
impl InterfaceCode for () {
	fn as_out_return_code(&mut self) -> Option<&mut ReturnCode> {
		None
	}
}
impl InterfaceCode for ReturnCode {
	fn as_out_return_code(&mut self) -> Option<&mut ReturnCode> {
		Some(self)
	}
}

pub trait RawInterfaceFactory {
	/// # Safety
	/// The returned [`RawInterface`] is valid only for the duration specified by the implementing interface.
	unsafe fn create_interface_raw(
		&self, name: &CStr, return_code: Option<&mut ReturnCode>,
	) -> Option<RawInterface>;
}

impl RawInterfaceFactory for CreateInterfaceFn {
	unsafe fn create_interface_raw(
		&self, name: &CStr, return_code: Option<&mut ReturnCode>,
	) -> Option<RawInterface> {
		unsafe {
			(self)(
				name.as_ptr(),
				return_code.map(move |p| p as *mut ReturnCode).unwrap_or(::core::ptr::null_mut()),
			)
		}
	}
}

pub trait InterfaceFactory: RawInterfaceFactory {
	fn create_interface<I: FromRawInterface>(&self) -> Result<I, I::Error>;
}

impl<T: RawInterfaceFactory> InterfaceFactory for T {
	fn create_interface<I: FromRawInterface>(&self) -> Result<I, I::Error> {
		let mut code = I::INITIAL_CODE;
		unsafe {
			let Some(raw_interface) = self.create_interface_raw(I::IDENTIIFER, code.as_out_return_code()) else {
				return Err(I::convert_error(code))
			};
			Ok(I::from_raw_interface(raw_interface, code))
		}
	}
}
