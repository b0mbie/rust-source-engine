//! Source Engine client functionality.

use ::core::{
	cell::RefCell,
	ffi::CStr,
};
use ::rse_client::interfaces::{
	VEngineClient, VEngineClientImpl,
	VEngineClient013Impl,
};

use crate::{
	plugin::PluginFactories,
	thread::MainThreadBound,
};

/// Inserts `command` into the command buffer as if it was typed by the client to their console.
/// 
/// # Restrictions
/// Only commands that are marked
/// [`FCVAR_CLIENTCMD_CAN_EXECUTE`](crate::con::CvarFlags::CLIENTCMD_CAN_EXECUTE)
/// can be executed from this function.
/// Use [`execute_unrestricted`]
/// to be able to execute any command.
pub fn execute(command: &CStr) {
	unsafe { read_mt(move |cl| cl.as_v013().client_cmd(command)) }
}

/// Inserts `command` into the command buffer as if it was typed by the client to their console.
/// 
/// # Restrictions
/// Unlike [`execute`],
/// this function can execute any command.
pub fn execute_unrestricted(command: &CStr) {
	unsafe { read_mt(move |cl| cl.as_v013().client_cmd_unrestricted(command)) }
}

/// Returns the size of the area that the game is being rendered to.
pub fn screen_size() -> (usize, usize) {
	unsafe { read_mt(move |cl| {
		let mut width = 0;
		let mut height = 0;
		cl.as_v013().screen_size(&mut width, &mut height);
		(width.max(0) as _, height.max(0) as _)
	}) }
}

/// Returns `true` if the client is currently in-game.
pub fn in_game() -> bool {
	read(move |cl| if let Some(cl) = cl {
		cl.as_v013().is_in_game()
	} else {
		false
	})
}

/// Returns `true` if the client is currently connected to a server.
pub fn connected() -> bool {
	read(move |cl| if let Some(cl) = cl {
		cl.as_v013().is_connected()
	} else {
		false
	})
}

/// Takes a screenshot of the game,
/// saving the resulting file to the specified `path`
/// in the specified `folder`
/// relative to the game directory.
pub fn take_screenshot(path: &CStr, folder: Option<&CStr>) {
	read(move |cl| if let Some(cl) = cl {
		cl.as_v013().take_screenshot(path, folder)
	})
}

/// Returns the protocol version number.
pub fn protocol_version() -> u64 {
	unsafe { read_mt(move |cl| cl.protocol_version()) as _ }
}

/// Returns `true` if the game is running in windowed mode.
pub fn is_windowed() -> bool {
	read(move |cl| if let Some(cl) = cl {
		cl.is_windowed_mode()
	} else {
		true
	})
}

/// Flashes the game window if the system allows for it.
pub fn flash_window() {
	unsafe { read_mt(move |cl| cl.flash_window()) }
}

/// Returns the client version number.
pub fn client_version() -> i64 {
	unsafe { read_mt(move |cl| cl.client_version()) as _ }
}

/// Returns `true` if the game window is focused.
pub fn is_focused() -> bool {
	read(move |cl| if let Some(cl) = cl {
		cl.is_active_app()
	} else {
		// Sure. Let's say that all eyes are on you.
		true
	})
}

static CLIENT: MainThreadBound<RefCell<Option<VEngineClient>>> = MainThreadBound::new(RefCell::new(None));

/// Tries to initialize the `IVEngineClient` functions in this module.
/// 
/// # Safety
/// This function must be called from the main thread.
pub(crate) unsafe fn attach(factories: PluginFactories) -> bool {
	match factories.create_interface() {
		Ok(iface) => {
			unsafe { *CLIENT.get_unchecked().try_borrow_mut().unwrap_unchecked() = Some(iface); }
			true
		}
		Err(error) => {
			::rse_tier0::con_warn!("{error}");
			false
		}
	}
}

#[cold]
const fn not_init() -> ! {
	panic!("client interface used without being initialized")
}

fn read<F: FnOnce(Option<&VEngineClient>) -> R, R>(f: F) -> R {
	if let Some(guard) = CLIENT.get().and_then(move |cell| cell.try_borrow().ok()) {
		match *guard {
			Some(ref srv) => f(Some(srv)),
			None => not_init(),
		}
	} else {
		f(None)
	}
}

// FIXME: Remove this when `write` is used.
#[allow(dead_code)]
fn write<F: FnOnce(Option<&mut VEngineClient>) -> R, R>(f: F) -> R {
	if let Some(mut guard) = CLIENT.get().and_then(move |cell| cell.try_borrow_mut().ok()) {
		match *guard {
			Some(ref mut srv) => f(Some(srv)),
			None => not_init(),
		}
	} else {
		f(None)
	}
}

/// # Safety
/// The operations performed on the interface *must* support multi-threading.
unsafe fn read_mt<F: FnOnce(&VEngineClient) -> R, R>(f: F) -> R {
	unsafe {
		// FIXME: See `sv::read_mt`.
		let guard = CLIENT.get_unchecked().try_borrow().unwrap_unchecked();
		match *guard {
			Some(ref srv) => f(srv),
			None => not_init(),
		}
	}
}
