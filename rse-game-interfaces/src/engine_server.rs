use ::core::{
	ffi::{
		CStr, c_char, c_int, c_float,
	},
	num::NonZero,
	ptr::NonNull,
};
use ::rse_cpp::{
	AsObject, virtual_call, owned_vt_object_wrapper,
};
use ::rse_game::cppdef::{
	SteamId,
	SoundLevel,
};
use ::rse_math::Vector;

use crate::{
	cppdef::{
		VEngineServerVt, INTERFACEVERSION_VENGINESERVER,
	},
	InterfaceOfFactory, AppSystemFactory,
};

pub trait VEngineServerImpl: AsObject<VEngineServerVt> {
	/// Returns `true` if the given `map_name` is a valid map.
	fn is_map_valid(&self, map_name: &CStr) -> bool {
		(unsafe { virtual_call!(self.as_object() => is_map_valid(map_name.as_ptr())) }) != 0
	}
	/// Returns `true` if the running server is a dedicated server.
	fn is_dedicated_server(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_dedicated_server()) }
	}
	/// Returns `true` if the server is in "edit mode"
	fn is_in_edit_mode(&self) -> bool {
		(unsafe { virtual_call!(self.as_object() => is_in_edit_mode()) }) != 0
	}
	/// Returns `true` if the server is paused.
	fn is_paused(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_paused()) }
	}
	/// Returns `true` if the server is in commentary mode.
	fn is_in_commentary_mode(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_in_commentary_mode()) }
	}

	/// Returns the current system time.
	fn system_time(&self) -> c_float {
		unsafe { virtual_call!(self.as_object() => time()) }
	}
	/// Returns the current server time.
	fn server_time(&self) -> c_float {
		unsafe { virtual_call!(self.as_object() => get_server_time()) }
	}
	/// Returns the game server's Steam ID.
	fn game_server_steam_id(&self) -> Option<&SteamId> {
		unsafe { virtual_call!(self.as_object() => get_game_server_steam_id()).as_ref() }
	}
	/// Returns the server's version.
	fn version(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => get_server_version()) }
	}
	/// Writes the game directory into `buffer`.
	fn game_dir(&mut self, buffer: &mut [c_char]) {
		unsafe { virtual_call!(
			self.as_object() => get_game_dir(
				buffer.as_mut_ptr(), buffer.len() as _,
			)
		) }
	}

	/// Prints `message` to the server log.
	fn log_print(&mut self, message: &CStr) {
		unsafe { virtual_call!(self.as_object() => log_print(message.as_ptr())) }
	}

	/// Returns the Steam app ID of the running server.
	fn app_id(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => get_app_id()) }
	}
	/// Returns `true` if the server is in Low-Violence mode.
	fn is_low_violence(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_low_violence()) }
	}
	/// Inserts `command` at the end of the command buffer.
	fn push_command_back(&mut self, command: &CStr) {
		unsafe { virtual_call!(self.as_object() => server_command(command.as_ptr())) }
	}
	/// Inserts `command` at the beginning of the command buffer.
	fn push_command_front(&mut self, command: &CStr) {
		unsafe { virtual_call!(self.as_object() => insert_server_command(command.as_ptr())) }
	}

	/// Pauses the server indefinitely.
	fn set_paused_forced(&mut self, paused: bool) {
		unsafe { virtual_call!(self.as_object() => set_paused_forced(paused, -1.0)) }
	}
	/// Pauses the server for the specifies `duration` of time, in seconds.
	fn set_paused_forced_for(&mut self, paused: bool, duration: c_float) {
		unsafe { virtual_call!(self.as_object() => set_paused_forced(paused, duration)) }
	}

	/// Returns the number of entities.
	fn entity_count(&self) -> usize {
		(unsafe { virtual_call!(self.as_object() => get_entity_count()) }) as _
	}
	/// Returns the value of the named ConVar of a client.
	fn client_con_var_value<'a>(&'a self, client_index: c_int, name: &CStr) -> &'a CStr {
		let ptr = unsafe { virtual_call!(self.as_object() => get_client_convar_value(client_index, name.as_ptr())) };
		unsafe { CStr::from_ptr(ptr) }
	}
	/// Emits an ambient sound.
	fn emit_ambient_sound(&mut self, options: EmitSound<'_>) {
		unsafe { virtual_call!(
			self.as_object() => emit_ambient_sound(
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
	fn precache_model(&mut self, path: &CStr, preload: bool) -> Option<Model> {
		unsafe {
			Model::new(virtual_call!(self.as_object() => precache_model(path.as_ptr(), preload)))
		}
	}
	/// Precaches a sentence file.
	/// 
	/// `preload` indicates whether the file will be precached before level startup.
	fn precache_sentence_file(&mut self, path: &CStr, preload: bool) {
		unsafe {
			// `PrecacheSentenceFile` doesn't return anything useful, so we ignore the result.
			virtual_call!(self.as_object() => precache_sentence_file(path.as_ptr(), preload));
		}
	}
	/// Precaches a decal.
	/// 
	/// `preload` indicates whether the file will be precached before level startup.
	fn precache_decal(&mut self, path: &CStr, preload: bool) -> Decal {
		unsafe { virtual_call!(self.as_object() => precache_decal(path.as_ptr(), preload)) }
	}
	/// Precaches a generic file.
	/// 
	/// `preload` indicates whether the file will be precached before level startup.
	fn precache_generic(&mut self, path: &CStr, preload: bool) -> Generic {
		unsafe { virtual_call!(self.as_object() => precache_generic(path.as_ptr(), preload)) }
	}

	/// Returns `true` if the given model is precached.
	fn is_model_precached(&self, path: &CStr) -> bool {
		unsafe { virtual_call!(self.as_object() => is_model_precached(path.as_ptr())) }
	}
	/// Returns `true` if the given decal is precached.
	fn is_decal_precached(&self, path: &CStr) -> bool {
		unsafe { virtual_call!(self.as_object() => is_decal_precached(path.as_ptr())) }
	}
	/// Returns `true` if the given generic file is precached.
	fn is_generic_precached(&self, path: &CStr) -> bool {
		unsafe { virtual_call!(self.as_object() => is_generic_precached(path.as_ptr())) }
	}
}
impl<T: AsObject<VEngineServerVt>> VEngineServerImpl for T {}

pub type Model = NonZero<c_int>;
pub type Decal = c_int;
pub type Generic = c_int;

owned_vt_object_wrapper! {
	pub struct VEngineServer for VEngineServerVt;
}
unsafe impl ::rse_interface::Interface for VEngineServer {
	const IDENTIFIER: &CStr = INTERFACEVERSION_VENGINESERVER;
}
impl InterfaceOfFactory for VEngineServer {
	type Factory = AppSystemFactory;
}

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
