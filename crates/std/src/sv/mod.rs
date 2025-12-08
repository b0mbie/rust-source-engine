//! Source Engine server functionality.

use ::core::ffi::CStr;
use ::rse_server::VEngineServerImpl;

use crate::c_buffer::CBuffer;

pub use ::rse_server::{
	Model, Decal, Generic,
};

pub mod raw;

/// Returns the current system time.
pub fn system_time() -> f32 {
	unsafe { raw::inspect_mt(move |sv| sv.system_time()) }
}

/// Returns `true` if the running server is a dedicated server.
pub fn is_dedicated() -> bool {
	unsafe { raw::inspect_mt(move |sv| sv.is_dedicated_server()) }
}

/// Returns the current server time.
pub fn server_time() -> f32 {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.server_time() as _
		} else {
			0.0
		})
	}
}

/// Returns `true` if the server is paused.
pub fn is_paused() -> bool {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.is_paused()
		} else {
			false
		})
	}
}

/// Returns `true` if the given `map` is a valid map.
pub fn is_map_valid(map: &CStr) -> bool {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.is_map_valid(map)
		} else {
			false
		})
	}
}

/// Inserts `command` at the end of the command buffer.
pub fn execute(command: &CStr) {
	unsafe { raw::inspect_mt(move |sv| sv.push_command_back(command)) }
}

/// Precaches a model.
/// 
/// `preload` indicates whether the file will be precached before level startup.
pub fn precache_model(path: &CStr, preload: bool) -> Option<Model> {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.precache_model(path, preload)
		} else {
			None
		})
	}
}

/// Precaches a sentence file.
/// 
/// `preload` indicates whether the file will be precached before level startup.
pub fn precache_sentence_file(path: &CStr, preload: bool) {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.precache_sentence_file(path, preload)
		})
	}
}

/// Precaches a decal.
/// 
/// `preload` indicates whether the file will be precached before level startup.
pub fn precache_decal(path: &CStr, preload: bool) -> Decal {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.precache_decal(path, preload)
		} else {
			0
		})
	}
}

/// Precaches a generic file.
/// 
/// `preload` indicates whether the file will be precached before level startup.
pub fn precache_generic(path: &CStr, preload: bool) -> Generic {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.precache_generic(path, preload)
		} else {
			0
		})
	}
}

const DEFAULT_PRECACHED: bool = false; // Nah.

/// Returns `true` if the given model is precached.
pub fn is_model_precached(path: &CStr) -> bool {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.is_model_precached(path)
		} else {
			DEFAULT_PRECACHED
		})
	}
}

/// Returns `true` if the given decal is precached.
pub fn is_decal_precached(path: &CStr) -> bool {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.is_decal_precached(path)
		} else {
			DEFAULT_PRECACHED
		})
	}
}

/// Returns `true` if the given generic file is precached.
pub fn is_generic_precached(path: &CStr) -> bool {
	unsafe {
		raw::inspect(move |sv| if let Some(sv) = sv {
			sv.is_generic_precached(path)
		} else {
			DEFAULT_PRECACHED
		})
	}
}

/// Returns a [`GameDir`] buffer that contains the path to the game directory.
pub fn game_dir() -> GameDir {
	unsafe {
		raw::inspect_mt(move |sv| {
			let mut dir = GameDir::new();
			sv.game_dir(dir.buffer.bytes_mut());
			dir
		})
	}
}

/// Writes the path to the game directory into the given [`GameDir`] buffer.
pub fn game_dir_into(dir: &mut GameDir) {
	unsafe { raw::inspect_mt(move |sv| sv.game_dir(dir.buffer.bytes_mut())) }
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
