use ::core::{
	ffi::{
		CStr, c_int, c_float,
	},
	num::NonZero,
	ptr::NonNull,
};
use ::rse_cpp::virtual_call;
use ::rse_game_interfaces::{
	InterfaceOfFactory, AppSystemFactory,
};
use ::rse_shared::cppdef::{
	CSteamID,
	SoundLevel,
};
use ::rse_math::Vector;

use crate::cppdef::{
	IVEngineServer, INTERFACEVERSION_VENGINESERVER,
};

::rse_interface::ptr_interface! {
	pub struct OVEngineServer(VEngineServer);
	Pointee = IVEngineServer;
}
impl ::rse_interface::Interface for OVEngineServer {
	const IDENTIFIER: &CStr = INTERFACEVERSION_VENGINESERVER;
}
impl InterfaceOfFactory for OVEngineServer {
	type Factory = AppSystemFactory;
}

/// Safe interface to `IVEngineServer`.
/// 
/// # Thread safety
/// Unless otherwise specified,
/// all functions are *not* thread-safe.
impl VEngineServer {
	/// Returns `true` if the given `map_name` is a valid map.
	pub fn is_map_valid(&self, map_name: &CStr) -> bool {
		(unsafe { virtual_call!(self.as_inner() => is_map_valid(map_name.as_ptr())) }) != 0
	}
	/// Returns `true` if the running server is a dedicated server.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	pub fn is_dedicated_server(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_dedicated_server()) }
	}
	/// Returns `true` if the server is in "edit mode"
	pub fn is_in_edit_mode(&self) -> bool {
		(unsafe { virtual_call!(self.as_inner() => is_in_edit_mode()) }) != 0
	}
	/// Returns `true` if the server is paused.
	pub fn is_paused(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_paused()) }
	}
	/// Returns `true` if the server is in commentary mode.
	pub fn is_in_commentary_mode(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_in_commentary_mode()) }
	}

	/// Returns the current system time.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	pub fn system_time(&self) -> c_float {
		unsafe { virtual_call!(self.as_inner() => time()) }
	}
	/// Returns the current server time.
	pub fn server_time(&self) -> c_float {
		unsafe { virtual_call!(self.as_inner() => get_server_time()) }
	}
	/// Returns the game server's Steam ID.
	pub fn game_server_steam_id(&self) -> Option<&CSteamID> {
		unsafe { virtual_call!(self.as_inner() => get_game_server_steam_id()).as_ref() }
	}
	/// Returns the server's version.
	pub fn version(&self) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_server_version()) }
	}
	/// Writes the game directory into `buffer`.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	pub fn game_dir(&self, buffer: &mut [u8]) {
		unsafe { virtual_call!(
			self.as_inner() => get_game_dir(
				buffer.as_mut_ptr() as _, buffer.len() as _,
			)
		) }
	}

	/// Prints `message` to the server log.
	pub fn log_print(&mut self, message: &CStr) {
		unsafe { virtual_call!(self.as_mut_inner() => log_print(message.as_ptr())) }
	}

	/// Returns the Steam app ID of the running server.
	pub fn app_id(&self) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_app_id()) }
	}
	/// Returns `true` if the server is in Low-Violence mode.
	pub fn is_low_violence(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_low_violence()) }
	}
	/// Inserts `command` at the end of the command buffer.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	pub fn push_command_back(&self, command: &CStr) {
		unsafe { virtual_call!(self.as_inner() => server_command(command.as_ptr())) }
	}
	/// Inserts `command` at the beginning of the command buffer.
	/// 
	/// # Thread safety
	/// This function is **thread-safe**.
	pub fn push_command_front(&self, command: &CStr) {
		unsafe { virtual_call!(self.as_inner() => insert_server_command(command.as_ptr())) }
	}

	/// Pauses the server indefinitely.
	pub fn set_paused_forced(&mut self, paused: bool) {
		unsafe { virtual_call!(self.as_mut_inner() => set_paused_forced(paused, -1.0)) }
	}
	/// Pauses the server for the specifies `duration` of time, in seconds.
	pub fn set_paused_forced_for(&mut self, paused: bool, duration: c_float) {
		unsafe { virtual_call!(self.as_mut_inner() => set_paused_forced(paused, duration)) }
	}

	/// Returns the number of entities.
	pub fn entity_count(&self) -> usize {
		(unsafe { virtual_call!(self.as_inner() => get_entity_count()) }) as _
	}
	/// Returns the value of the named ConVar of a client.
	pub fn client_con_var_value<'a>(&'a self, client_index: c_int, name: &CStr) -> &'a CStr {
		let ptr = unsafe { virtual_call!(self.as_inner() => get_client_convar_value(client_index, name.as_ptr())) };
		unsafe { CStr::from_ptr(ptr) }
	}
	/// Emits an ambient sound.
	pub fn emit_ambient_sound(&mut self, options: EmitSound<'_>) {
		unsafe { virtual_call!(
			self.as_mut_inner() => emit_ambient_sound(
				options.ent_index, NonNull::from(options.pos),
				options.sample.as_ptr(),
				options.volume,
				options.sound_level,
				options.flags,
				options.pitch,
				options.delay,
			)
		) }
	}

	/// Precaches a model.
	/// 
	/// `preload` indicates whether the file will be precached before level startup.
	pub fn precache_model(&mut self, path: &CStr, preload: bool) -> Option<Model> {
		unsafe {
			Model::new(virtual_call!(self.as_mut_inner() => precache_model(path.as_ptr(), preload)))
		}
	}
	/// Precaches a sentence file.
	/// 
	/// `preload` indicates whether the file will be precached before level startup.
	pub fn precache_sentence_file(&mut self, path: &CStr, preload: bool) {
		unsafe {
			// `PrecacheSentenceFile` doesn't return anything useful, so we ignore the result.
			virtual_call!(self.as_mut_inner() => precache_sentence_file(path.as_ptr(), preload));
		}
	}
	/// Precaches a decal.
	/// 
	/// `preload` indicates whether the file will be precached before level startup.
	pub fn precache_decal(&mut self, path: &CStr, preload: bool) -> Decal {
		unsafe { virtual_call!(self.as_mut_inner() => precache_decal(path.as_ptr(), preload)) }
	}
	/// Precaches a generic file.
	/// 
	/// `preload` indicates whether the file will be precached before level startup.
	pub fn precache_generic(&mut self, path: &CStr, preload: bool) -> Generic {
		unsafe { virtual_call!(self.as_mut_inner() => precache_generic(path.as_ptr(), preload)) }
	}

	/// Returns `true` if the given model is precached.
	pub fn is_model_precached(&self, path: &CStr) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_model_precached(path.as_ptr())) }
	}
	/// Returns `true` if the given decal is precached.
	pub fn is_decal_precached(&self, path: &CStr) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_decal_precached(path.as_ptr())) }
	}
	/// Returns `true` if the given generic file is precached.
	pub fn is_generic_precached(&self, path: &CStr) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_generic_precached(path.as_ptr())) }
	}
}

/// Index to a precached model.
pub type Model = NonZero<c_int>;
/// Index to a precached decal.
pub type Decal = c_int;
/// Index to a precached generic file.
pub type Generic = c_int;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct EmitSound<'a> {
	pub ent_index: c_int,
	pub pos: &'a Vector,
	pub sample: &'a CStr,
	pub volume: c_float,
	pub sound_level: SoundLevel,
	pub flags: c_int,
	pub pitch: c_int,
	pub delay: c_float,
}
