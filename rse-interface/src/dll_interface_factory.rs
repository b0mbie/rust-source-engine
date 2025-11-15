use ::core::ffi::{
	CStr, c_char,
};

use crate::cppdef::{
	RawInterface, ReturnCode,
};

use super::RawInterfaceFactory;

/// # Safety
/// `name` must point to a valid C string,
/// and `out_return_code`, if non-null, must point to a readable and writable [`ReturnCode`].
pub unsafe fn handle_create_interface<T: ?Sized + RawInterfaceFactory>(
	factory: &T,
	name: *const c_char, out_return_code: *mut ReturnCode,
) -> Option<RawInterface> {
	unsafe {
		let name = CStr::from_ptr(name);
		let return_code = out_return_code.as_mut();
		factory.create_interface_raw(name, return_code)
	}
}

#[macro_export]
macro_rules! export_interface_factory {
	($ty:ty = $init:expr) => {
		const _: () = {
			$crate::export_interface_factory_as! {
				static EXPORTED_INTERFACE_FACTORY: $ty = $init;
			}
		};
	};
}

#[macro_export]
macro_rules! export_interface_factory_as {
	{
		$(#[$attr:meta])*
		$vis:vis static $name:ident: $ty:ty = $init:expr;
	} => {
		$(#[$attr])*
		$vis static $name: $ty = $init;
		$crate::export_interface_factory_fn!(&$name);
	};
}

#[macro_export]
macro_rules! export_interface_factory_fn {
	($factory_ref:expr => $export_name:literal) => {
		const _: () = const {
			#[unsafe(export_name = $export_name)]
			unsafe extern "C" fn create_interface(
				name: *const ::core::ffi::c_char, out_return_code: *mut $crate::cppdef::ReturnCode,
			) -> Option<$crate::cppdef::RawInterface> {
				unsafe { $crate::handle_create_interface($factory_ref, name, out_return_code) }
			}
			const _ASSERT_CREATE_INTERFACE_SIGNATURE: $crate::cppdef::CreateInterfaceFn = create_interface;
		};
	};

	($factory_ref:expr) => {
		$crate::export_interface_factory_fn!($factory_ref => "CreateInterface");
	};
}
