use ::core::ffi::CStr;
use ::rse_convar::cppdef::CvarDllIdentifier;
use ::rse_cpp::AsObject;

use crate::cppdef::{
	CVAR_INTERFACE_VERSION, CvarVt,
};

use super::{
	InterfaceOfFactory, AppSystemFactory,
};

macro_rules! cvar_call {
	($object:expr => $field:ident($($arg:tt)*)) => {{
		let object = $object;
		(object.vtable().cvar.$field)(object.as_ptr().cast(), $($arg)*)
	}};
}

pub trait CvarImpl: AsObject<CvarVt> {
	fn allocate_dll_identifier(&mut self) -> CvarDllIdentifier {
		unsafe { cvar_call!(self.as_object() => allocate_dll_identifier()) }
	}
}
impl<T: AsObject<CvarVt>> CvarImpl for T {}

::rse_cpp::owned_vt_object_wrapper! {
	pub struct Cvar for CvarVt;
}
unsafe impl ::rse_interface::Interface for Cvar {
	const IDENTIFIER: &CStr = CVAR_INTERFACE_VERSION;
}
impl InterfaceOfFactory for Cvar {
	type Factory = AppSystemFactory;
}