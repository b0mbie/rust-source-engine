use ::core::ffi::{
	CStr, c_char,
};

use crate::cppdef::{
	RawInterface, ReturnCode,
};

use super::RawInterfaceFactory;

pub trait DllInterfaceFactory: 'static + RawInterfaceFactory {
	const INSTANCE: &Self;
}

/// # Safety
/// `name` must point to a valid C string,
/// and `out_return_code`, if non-null, must point to a readable and writable [`ReturnCode`].
pub unsafe fn handle_create_interface<T: DllInterfaceFactory>(
	name: *const c_char, out_return_code: *mut ReturnCode,
) -> Option<RawInterface> {
	unsafe {
		let name = CStr::from_ptr(name);
		let return_code = out_return_code.as_mut();
		<T as DllInterfaceFactory>::INSTANCE.create_interface_raw(name, return_code)
	}
}

#[macro_export]
macro_rules! dll_interface_factory {
	($ty:ty => $export_name:literal) => {
		const _: () = const {
			#[unsafe(export_name = $export_name)]
			unsafe extern "C-unwind" fn create_interface(
				name: *const ::core::ffi::c_char, out_return_code: *mut $crate::cppdef::ReturnCode,
			) -> Option<$crate::cppdef::RawInterface> {
				unsafe { $crate::handle_create_interface::<$ty>(name, out_return_code) }
			}
		};
	};

	($ty:ty) => {
		$crate::dll_interface_factory!($ty => "CreateInterface");
	};
}
