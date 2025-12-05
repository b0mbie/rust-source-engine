//! Source Engine server functionality.

use ::core::{
	cell::RefCell,
	ffi::CStr,
};
use ::rse_server::{
	VEngineServer, VEngineServerImpl,
};

pub use ::rse_server::{
	Model, Decal, Generic,
};

use crate::{
	c_buffer::CBuffer,
	plugin::PluginFactories,
	thread::MainThreadBound,
};

static SERVER: MainThreadBound<RefCell<Option<VEngineServer>>> =
	MainThreadBound::new(RefCell::new(None));

/// Tries to initialize the `IVEngineServer` functions in this module.
/// 
/// # Safety
/// This function must be called from the main thread.
pub(crate) unsafe fn attach(factories: PluginFactories) -> bool {
	match factories.create_interface() {
		Ok(iface) => {
			unsafe { *SERVER.get_unchecked().try_borrow_mut().unwrap_unchecked() = Some(iface); }
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
	panic!("server interface used without being initialized")
}

fn read<F: FnOnce(Option<&VEngineServer>) -> R, R>(f: F) -> R {
	if let Some(guard) = SERVER.get().and_then(move |cell| cell.try_borrow().ok()) {
		match *guard {
			Some(ref srv) => f(Some(srv)),
			None => not_init(),
		}
	} else {
		f(None)
	}
}

fn write<F: FnOnce(Option<&mut VEngineServer>) -> R, R>(f: F) -> R {
	if let Some(mut guard) = SERVER.get().and_then(move |cell| cell.try_borrow_mut().ok()) {
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
unsafe fn read_mt<F: FnOnce(&VEngineServer) -> R, R>(f: F) -> R {
	unsafe {
		let srv = &*SERVER.get_unchecked().as_ptr();
		match srv {
			Some(srv) => f(srv),
			None => not_init(),
		}
	}
}

/// Returns the current system time.
pub fn system_time() -> f32 {
	unsafe { read_mt(move |srv| srv.system_time()) }
}

/// Returns `true` if the running server is a dedicated server.
pub fn is_dedicated() -> bool {
	unsafe { read_mt(move |srv| srv.is_dedicated_server()) }
}

/// Returns the current server time.
pub fn server_time() -> f32 {
	read(move |srv| if let Some(srv) = srv {
		srv.server_time() as _
	} else {
		0.0
	})
}

/// Returns `true` if the server is paused.
pub fn is_paused() -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_paused()
	} else {
		false
	})
}

/// Returns `true` if the given `map` is a valid map.
pub fn is_map_valid(map: &CStr) -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_map_valid(map)
	} else {
		false
	})
}

/// Inserts `command` at the end of the command buffer.
pub fn execute(command: &CStr) {
	unsafe { read_mt(move |srv| srv.push_command_back(command)) }
}

/// Precaches a model.
/// 
/// `preload` indicates whether the file will be precached before level startup.
pub fn precache_model(path: &CStr, preload: bool) -> Option<Model> {
	write(move |srv| if let Some(srv) = srv {
		srv.precache_model(path, preload)
	} else {
		None
	})
}

/// Precaches a sentence file.
/// 
/// `preload` indicates whether the file will be precached before level startup.
pub fn precache_sentence_file(path: &CStr, preload: bool) {
	write(move |srv| if let Some(srv) = srv {
		srv.precache_sentence_file(path, preload)
	})
}

/// Precaches a decal.
/// 
/// `preload` indicates whether the file will be precached before level startup.
pub fn precache_decal(path: &CStr, preload: bool) -> Decal {
	write(move |srv| if let Some(srv) = srv {
		srv.precache_decal(path, preload)
	} else {
		0
	})
}

/// Precaches a generic file.
/// 
/// `preload` indicates whether the file will be precached before level startup.
pub fn precache_generic(path: &CStr, preload: bool) -> Generic {
	write(move |srv| if let Some(srv) = srv {
		srv.precache_generic(path, preload)
	} else {
		0
	})
}

const DEFAULT_PRECACHED: bool = false; // Nah.

/// Returns `true` if the given model is precached.
pub fn is_model_precached(path: &CStr) -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_model_precached(path)
	} else {
		DEFAULT_PRECACHED
	})
}

/// Returns `true` if the given decal is precached.
pub fn is_decal_precached(path: &CStr) -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_decal_precached(path)
	} else {
		DEFAULT_PRECACHED
	})
}

/// Returns `true` if the given generic file is precached.
pub fn is_generic_precached(path: &CStr) -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_generic_precached(path)
	} else {
		DEFAULT_PRECACHED
	})
}

/// Returns a [`GameDir`] buffer that contains the path to the game directory.
pub fn game_dir() -> GameDir {
	unsafe {
		read_mt(move |srv| {
			let mut dir = GameDir::new();
			srv.game_dir(dir.buffer.bytes_mut());
			dir
		})
	}
}

/// Writes the path to the game directory into the given [`GameDir`] buffer.
pub fn game_dir_into(dir: &mut GameDir) {
	unsafe { read_mt(move |srv| srv.game_dir(dir.buffer.bytes_mut())) }
}

/// Buffer that holds the path to the game directory as a C string.
#[derive(Default, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct GameDir {
	buffer: CBuffer<{crate::fs_consts::MAX_OSPATH}>,
}

impl GameDir {
	/// Returns a new, empty buffer.
	pub const fn new() -> Self {
		Self {
			buffer: CBuffer::new(),
		}
	}

	/// Returns the [`CStr`] that represents the path to the game directory.
	pub const fn as_c_str(&self) -> &CStr {
		self.buffer.as_c_str()
	}
}
