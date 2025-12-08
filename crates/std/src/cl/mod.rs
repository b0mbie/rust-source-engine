//! Source Engine client functionality.

use ::core::ffi::CStr;
use ::rse_client::interfaces::{
	VEngineClientImpl, VEngineClient013Impl,
};

pub mod raw;

/// Inserts `command` into the command buffer as if it was typed by the client to their console.
/// 
/// # Restrictions
/// Only commands that are marked
/// [`FCVAR_CLIENTCMD_CAN_EXECUTE`](crate::con::CvarFlags::CLIENTCMD_CAN_EXECUTE)
/// can be executed from this function.
/// Use [`execute_unrestricted`]
/// to be able to execute any command.
pub fn execute(command: &CStr) {
	unsafe { raw::get_mt(move |cl| cl.client_cmd(command)) }
}

/// Inserts `command` into the command buffer as if it was typed by the client to their console.
/// 
/// # Restrictions
/// Unlike [`execute`],
/// this function can execute any command.
pub fn execute_unrestricted(command: &CStr) {
	unsafe { raw::get_mt(move |cl| cl.client_cmd_unrestricted(command)) }
}

/// Returns the size of the area that the game is being rendered to.
pub fn screen_size() -> (usize, usize) {
	unsafe {
		raw::get_mt(move |cl| {
			let mut width = 0;
			let mut height = 0;
			cl.screen_size(&mut width, &mut height);
			(width.max(0) as _, height.max(0) as _)
		})
	}
}

/// Returns `true` if the client is currently in-game.
pub fn in_game() -> bool {
	unsafe {
		raw::get(move |cl| {
			cl.map(move |cl| cl.is_in_game()).unwrap_or(false)
		})
	}
}

/// Returns `true` if the client is currently connected to a server.
pub fn connected() -> bool {
	unsafe {
		raw::get(move |cl| {
			cl.map(move |cl| cl.is_connected()).unwrap_or(false)
		})
	}
}

/// Takes a screenshot of the game,
/// saving the resulting file to the specified `path`
/// in the specified `folder`
/// relative to the game directory.
pub fn take_screenshot(path: &CStr, folder: Option<&CStr>) {
	unsafe {
		raw::get(move |cl| if let Some(cl) = cl {
			cl.take_screenshot(path, folder)
		})
	}
}

/// Returns the protocol version number,
/// or `None` if this information is unavailable.
pub fn protocol_version() -> Option<u64> {
	unsafe {
		raw::get_mt(move |cl| {
			cl.to_v14().map(move |cl| cl.protocol_version() as _)
		})
	}
}

/// Returns `true` if the game is running in windowed mode,
/// or `None` if this information is unavailable.
pub fn is_windowed() -> Option<bool> {
	unsafe {
		raw::get(move |cl| {
			cl.and_then(move |cl| cl.to_v14())
				.map(move |cl| cl.is_windowed_mode())
		})
	}
}

/// Flashes the game window if the system allows for it.
pub fn flash_window() {
	unsafe {
		raw::get_mt(move |cl| if let Some(cl) = cl.to_v14() {
			cl.flash_window()
		})
	}
}

/// Returns the client version number,
/// or `None` if this information is unavailable.
pub fn client_version() -> Option<i64> {
	unsafe {
		raw::get_mt(move |cl| {
			cl.to_v14().map(move |cl| cl.client_version() as _)
		})
	}
}

/// Returns `true` if the game window is focused,
/// or `None` if this information is unavailable.
pub fn is_focused() -> Option<bool> {
	unsafe {
		raw::get(move |cl| {
			cl.and_then(move |cl| cl.to_v14())
				.map(move |cl| cl.is_windowed_mode())
		})
	}
}