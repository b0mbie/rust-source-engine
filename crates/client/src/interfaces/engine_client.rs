use ::core::ffi::{
	CStr, c_int, c_uint,
};
use ::rse_cpp::{
	ptr_compat::convert_ref,
	virtual_call,
};
use ::rse_game_interfaces::InterfaceOfFactory;
use ::rse_interface::{
	Interface,
	ptr_interface,
};

use crate::cppdef::{
	IVEngineClient013, VENGINE_CLIENT_INTERFACE_VERSION_13,
	IVEngineClient, VENGINE_CLIENT_INTERFACE_VERSION,
};

ptr_interface! {
	pub struct OVEngineClient(VEngineClient);
	Pointee = IVEngineClient;
}
impl Interface for OVEngineClient {
	const IDENTIFIER: &CStr = VENGINE_CLIENT_INTERFACE_VERSION;
}
impl InterfaceOfFactory for OVEngineClient {
	type Factory = ::rse_game_interfaces::AppSystemFactory;
}

/// Safe interface to `IVEngineClient`.
/// 
/// # Thread safety
/// Unless otherwise specified,
/// all functions are *not* thread-safe.
impl VEngineClient {
	pub const fn as_v013(&self) -> &VEngineClient013 {
		unsafe { VEngineClient013::from_ref(convert_ref(&self.0)) }
	}

	/// Returns the protocol version number.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	pub fn protocol_version(&self) -> c_uint {
		unsafe { virtual_call!(self.as_inner() => get_protocol_version()) }
	}

	/// Returns `true` if the game is running in windowed mode.
	// FIXME: This may be thread-safe!
	pub fn is_windowed_mode(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_windowed_mode()) }
	}

	/// Flashes the game window if the system allows for it.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	// FIXME: This really *does* seem thread-safe (uses Win32 API), but further testing is required!
	pub fn flash_window(&self) {
		unsafe { virtual_call!(self.as_inner() => flash_window()) }
	}

	/// Returns the client version number.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	pub fn client_version(&self) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_client_version()) }
	}

	/// Returns `true` if the game window is focused.
	// FIXME: This may be thread-safe!
	pub fn is_active_app(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_active_app()) }
	}
}

ptr_interface! {
	pub struct OVEngineClient013(VEngineClient013);
	Pointee = IVEngineClient013;
}
impl Interface for OVEngineClient013 {
	const IDENTIFIER: &CStr = VENGINE_CLIENT_INTERFACE_VERSION_13;
}
impl InterfaceOfFactory for OVEngineClient013 {
	type Factory = ::rse_game_interfaces::AppSystemFactory;
}

/// Safe interface to `IVEngineClient013`.
/// 
/// # Thread safety
/// Unless otherwise specified,
/// all functions are *not* thread-safe.
impl VEngineClient013 {
	/// Fills `width` and `height` with the size of the area that the game is being rendered to.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	// FIXME: This really *does* seem thread-safe (uses Win32 API), but further testing is required!
	pub fn screen_size(&self, width: &mut c_int, height: &mut c_int) {
		unsafe { virtual_call!(self.as_inner() => get_screen_size(width.into(), height.into())) }
	}

	pub fn server_cmd(&self, command: &CStr, reliable: bool) {
		unsafe { virtual_call!(self.as_inner() => server_cmd(command.as_ptr(), reliable)) }
	}

	/// Inserts `command` into the command buffer as if it was typed by the client to their console.
	/// 
	/// # Restrictions
	/// Only commands that are marked `FCVAR_CLIENTCMD_CAN_EXECUTE`
	/// can be executed from this function.
	/// Use [`client_cmd_unrestricted`](Self::client_cmd_unrestricted)
	/// to be able to execute any command.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	pub fn client_cmd(&self, command: &CStr) {
		unsafe { virtual_call!(self.as_inner() => client_cmd(command.as_ptr())) }
	}

	/// Inserts `command` into the command buffer as if it was typed by the client to their console.
	/// 
	/// # Restrictions
	/// Unlike [`client_cmd`](Self::client_cmd),
	/// this function can execute any command.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	pub fn client_cmd_unrestricted(&self, command: &CStr) {
		unsafe { virtual_call!(self.as_inner() => client_cmd_unrestricted(command.as_ptr())) }
	}

	pub fn is_in_game(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_in_game()) }
	}
	pub fn is_connected(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_connected()) }
	}

	pub fn take_screenshot(&self, path: &CStr, folder: Option<&CStr>) {
		unsafe { virtual_call!(
			self.as_inner() => take_screenshot(path.as_ptr(), folder.map(move |s| s.as_ptr()).unwrap_or_default())
		) }
	}
}
