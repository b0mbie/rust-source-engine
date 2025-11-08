use ::core::ffi::CStr;
use ::rse_cpp::{
	convert_vt_ref,
	VtObject, AsObject, virtual_call,
};

use crate::cppdef::engine_client::{
	VEngineClient013Vt,
	VEngineClientVt, VENGINE_CLIENT_INTERFACE_VERSION,
};

/// Safe interface to `IVEngineServer`.
/// 
/// # Thread safety
/// Unless otherwise specified,
/// all functions are *not* thread-safe.
pub trait VEngineClientImpl: AsObject<VEngineClientVt> {
	fn as_v013(&self) -> &VtObject<VEngineClient013Vt> {
		convert_vt_ref(self.as_object())
	}
	fn is_in_game(&self) -> bool {
		unsafe { virtual_call!(self.as_v013() => is_in_game()) }
	}
	fn is_connected(&self) -> bool {
		unsafe { virtual_call!(self.as_v013() => is_connected()) }
	}
}
impl<T: ?Sized + AsObject<VEngineClientVt>> VEngineClientImpl for T {}

::rse_cpp::owned_vt_object_wrapper! {
	pub struct VEngineClient for VEngineClientVt;
}
unsafe impl ::rse_interface::Interface for VEngineClient {
	const IDENTIFIER: &CStr = VENGINE_CLIENT_INTERFACE_VERSION;
}
impl ::rse_game_interfaces::InterfaceOfFactory for VEngineClient {
	type Factory = ::rse_game_interfaces::AppSystemFactory;
}
