pub use ::rse_game_interfaces::{
	Model, Decal, Generic,
};

use ::rse_game_interfaces::{
	InterfaceFactories,
	VEngineServer, VEngineServerImpl,
};
use ::std::{
	cell::RefCell,
	ffi::CStr,
	mem::MaybeUninit,
};

use crate::{
	c_buffer::CBuffer,
	threads::MainThreadBound,
};

static SERVER: MainThreadBound<RefCell<MaybeUninit<VEngineServer>>> =
	MainThreadBound::new(RefCell::new(MaybeUninit::uninit()));

/// Tries to initialize the `IVEngineServer` functions in this module.
/// 
/// # Safety
/// This function must be called from the main thread.
/// 
/// If this function returns `false`, then the functions in this module must not be used.
pub(crate) unsafe fn attach(factories: InterfaceFactories<'_>) -> bool {
	match factories.create_interface() {
		Ok(iface) => {
			unsafe { SERVER.get_unchecked().try_borrow_mut().unwrap_unchecked().write(iface); }
			true
		}
		Err(error) => {
			::rse_tier0::con_warn!("{error}");
			false
		}
	}
}

fn read<F: FnOnce(Option<&VEngineServer>) -> R, R>(f: F) -> R {
	if let Some(guard) = SERVER.get().and_then(move |cell| cell.try_borrow().ok()) {
		unsafe { f(Some(guard.assume_init_ref())) }
	} else {
		f(None)
	}
}

fn write<F: FnOnce(Option<&mut VEngineServer>) -> R, R>(f: F) -> R {
	if let Some(mut guard) = SERVER.get().and_then(move |cell| cell.try_borrow_mut().ok()) {
		unsafe { f(Some(guard.assume_init_mut())) }
	} else {
		f(None)
	}
}

/// # Safety
/// The operations performed on the interface *must* support multi-threading.
unsafe fn read_mt<F: FnOnce(&VEngineServer) -> R, R>(f: F) -> R {
	unsafe { f(SERVER.get_unchecked().try_borrow().unwrap_unchecked().assume_init_ref()) }
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
	buffer: CBuffer<{crate::fs::MAX_OSPATH}>,
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
