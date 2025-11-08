use ::core::ffi::{
	CStr, c_int, c_uint,
};
use ::rse_cpp::{
	owned_vt_object_wrapper,
	convert_vt_ref,
	VtObject, AsObject, virtual_call,
};
use ::rse_game_interfaces::InterfaceOfFactory;
use ::rse_interface::Interface;

use crate::cppdef::engine_client::{
	VEngineClient013Vt, VENGINE_CLIENT_INTERFACE_VERSION_13,
	VEngineClientVt, VENGINE_CLIENT_INTERFACE_VERSION,
};

/// Safe interface to `IVEngineServer`.
/// 
/// # Thread safety
/// Unless otherwise specified,
/// all functions are *not* thread-safe.
pub trait VEngineClientImpl: AsObject<VEngineClientVt> {
	/// # Thread safety
	/// This function is **thread-safe**.
	fn as_v013(&self) -> &VtObject<VEngineClient013Vt> {
		convert_vt_ref(self.as_object())
	}

	/// Returns the protocol version number.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	fn protocol_version(&self) -> c_uint {
		unsafe { virtual_call!(self.as_object() => engine_client.get_protocol_version()) }
	}

	/// Returns `true` if the game is running in windowed mode.
	// FIXME: This may be thread-safe!
	fn is_windowed_mode(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => engine_client.is_windowed_mode()) }
	}

	/// Flashes the game window if the system allows for it.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	// FIXME: This really *does* seem thread-safe (uses Win32 API), but further testing is required!
	fn flash_window(&self) {
		unsafe { virtual_call!(self.as_object() => engine_client.flash_window()) }
	}

	/// Returns the client version number.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	fn client_version(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => engine_client.get_client_version()) }
	}

	/// Returns `true` if the game window is focused.
	// FIXME: This may be thread-safe!
	fn is_active_app(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => engine_client.is_active_app()) }
	}
}
impl<T: ?Sized + AsObject<VEngineClientVt>> VEngineClientImpl for T {}

/// Safe interface to `IVEngineServer013`.
/// 
/// # Thread safety
/// Unless otherwise specified,
/// all functions are *not* thread-safe.
pub trait VEngineClient013Impl: AsObject<VEngineClient013Vt> {
	/// Fills `width` and `height` with the size of the area that the game is being rendered to.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	// FIXME: This really *does* seem thread-safe (uses Win32 API), but further testing is required!
	fn screen_size(&self, width: &mut c_int, height: &mut c_int) {
		unsafe { virtual_call!(self.as_object() => get_screen_size(width.into(), height.into())) }
	}

	fn server_cmd(&self, command: &CStr, reliable: bool) {
		unsafe { virtual_call!(self.as_object() => server_cmd(command.as_ptr(), reliable)) }
	}

	/// Inserts `command` into the command buffer as if it was typed by the client to their console.
	/// 
	/// # Restrictions
	/// Only commands that are marked `FCVAR_CLIENTCMD_CAN_EXECUTE`
	/// can be executed from this function.
	/// Use [`client_cmd_unrestricted`](VEngineClient013Impl::client_cmd_unrestricted)
	/// to be able to execute any command.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	fn client_cmd(&self, command: &CStr) {
		unsafe { virtual_call!(self.as_object() => client_cmd(command.as_ptr())) }
	}

	/// Inserts `command` into the command buffer as if it was typed by the client to their console.
	/// 
	/// # Restrictions
	/// Unlike [`client_cmd`](VEngineClient013Impl::client_cmd),
	/// this function can execute any command.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	fn client_cmd_unrestricted(&self, command: &CStr) {
		unsafe { virtual_call!(self.as_object() => client_cmd_unrestricted(command.as_ptr())) }
	}

	fn is_in_game(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_in_game()) }
	}
	fn is_connected(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_connected()) }
	}

	fn take_screenshot(&self, path: &CStr, folder: Option<&CStr>) {
		unsafe { virtual_call!(
			self.as_object() => take_screenshot(path.as_ptr(), folder.map(move |s| s.as_ptr()).unwrap_or_default())
		) }
	}
}
impl<T: ?Sized + AsObject<VEngineClient013Vt>> VEngineClient013Impl for T {}

owned_vt_object_wrapper! {
	pub struct VEngineClient for VEngineClientVt;
}
unsafe impl Interface for VEngineClient {
	const IDENTIFIER: &CStr = VENGINE_CLIENT_INTERFACE_VERSION;
}
impl InterfaceOfFactory for VEngineClient {
	type Factory = ::rse_game_interfaces::AppSystemFactory;
}

owned_vt_object_wrapper! {
	pub struct VEngineClient013 for VEngineClientVt;
}
unsafe impl Interface for VEngineClient013 {
	const IDENTIFIER: &CStr = VENGINE_CLIENT_INTERFACE_VERSION_13;
}
impl InterfaceOfFactory for VEngineClient013 {
	type Factory = ::rse_game_interfaces::AppSystemFactory;
}
