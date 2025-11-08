use ::core::{
	cell::RefCell,
	ffi::CStr,
};
use ::rse_game_interfaces::{
	VEngineServer, VEngineServerImpl,
};

pub use ::rse_game_interfaces::{
	Model, Decal, Generic,
};

use crate::{
	c_buffer::CBuffer,
	plugin::PluginFactories,
	threads::MainThreadBound,
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
		// FIXME: We might need a better alternative to `RefCell` to be able to handle MT properly.
		// Right now, there is no way to Just Get A Reference to the value wrapped by a `RefCell`
		// without having to use `Result::unwrap_unchecked` and possibly getting garbage.
		// Multi-threaded operations effectively bypass the borrow-checking rules;
		// it might not be possible to represent this behavior with this type.
		let guard = SERVER.get_unchecked().try_borrow().unwrap_unchecked();
		match *guard {
			Some(ref srv) => f(srv),
			None => not_init(),
		}
	}
}

pub fn is_main_thread() -> bool {
	SERVER.can_be_accessed()
}

pub fn system_time() -> f32 {
	unsafe { read_mt(move |srv| srv.system_time()) }
}
pub fn is_dedicated() -> bool {
	unsafe { read_mt(move |srv| srv.is_dedicated_server()) }
}

pub fn server_time() -> f32 {
	read(move |srv| if let Some(srv) = srv {
		srv.server_time() as _
	} else {
		0.0
	})
}

pub fn is_paused() -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_paused()
	} else {
		false
	})
}
pub fn is_map_valid(map_name: &CStr) -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_map_valid(map_name)
	} else {
		false
	})
}

pub fn command(command: &CStr) {
	unsafe { read_mt(move |srv| srv.push_command_back(command)) }
}

pub fn precache_model(path: &CStr, preload: bool) -> Option<Model> {
	write(move |srv| if let Some(srv) = srv {
		srv.precache_model(path, preload)
	} else {
		None
	})
}
pub fn precache_sentence_file(path: &CStr, preload: bool) {
	write(move |srv| if let Some(srv) = srv {
		srv.precache_sentence_file(path, preload)
	})
}
pub fn precache_decal(path: &CStr, preload: bool) -> Decal {
	write(move |srv| if let Some(srv) = srv {
		srv.precache_decal(path, preload)
	} else {
		0
	})
}
pub fn precache_generic(path: &CStr, preload: bool) -> Generic {
	write(move |srv| if let Some(srv) = srv {
		srv.precache_generic(path, preload)
	} else {
		0
	})
}

const DEFAULT_PRECACHED: bool = false; // Nah.
pub fn is_model_precached(path: &CStr) -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_model_precached(path)
	} else {
		DEFAULT_PRECACHED
	})
}
pub fn is_decal_precached(path: &CStr) -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_decal_precached(path)
	} else {
		DEFAULT_PRECACHED
	})
}
pub fn is_generic_precached(path: &CStr) -> bool {
	read(move |srv| if let Some(srv) = srv {
		srv.is_generic_precached(path)
	} else {
		DEFAULT_PRECACHED
	})
}

pub fn game_dir() -> GameDir {
	unsafe {
		read_mt(move |srv| {
			let mut dir = GameDir::new();
			srv.game_dir(dir.buffer.bytes_mut());
			dir
		})
	}
}
pub fn game_dir_into(dir: &mut GameDir) {
	unsafe { read_mt(move |srv| srv.game_dir(dir.buffer.bytes_mut())) }
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct GameDir {
	buffer: CBuffer<{crate::fs_consts::MAX_OSPATH}>,
}

impl GameDir {
	pub const fn new() -> Self {
		Self {
			buffer: CBuffer::new(),
		}
	}

	pub const fn as_c_str(&self) -> &CStr {
		self.buffer.as_c_str()
	}
}
