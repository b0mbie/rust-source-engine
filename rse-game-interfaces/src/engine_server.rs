use ::core::{
	ffi::{
		CStr, c_char, c_int, c_float,
	},
	ptr::NonNull,
	time::Duration,
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
	fn is_map_valid(&self, map_name: &CStr) -> bool {
		(unsafe { virtual_call!(self.as_object() => is_map_valid(map_name.as_ptr())) }) != 0
	}
	fn is_dedicated_server(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_dedicated_server()) }
	}
	fn is_in_edit_mode(&self) -> bool {
		(unsafe { virtual_call!(self.as_object() => is_in_edit_mode()) }) != 0
	}
	fn get_entity_count(&self) -> usize {
		(unsafe { virtual_call!(self.as_object() => get_entity_count()) }) as _
	}
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
	fn server_command(&mut self, command: &CStr) {
		unsafe { virtual_call!(self.as_object() => server_command(command.as_ptr())) }
	}
	fn time(&self) -> c_float {
		unsafe { virtual_call!(self.as_object() => time()) }
	}
	fn get_game_dir(&mut self, buffer: &mut [c_char]) {
		unsafe { virtual_call!(
			self.as_object() => get_game_dir(
				buffer.as_mut_ptr(), buffer.len() as _,
			)
		) }
	}
	fn get_client_convar_value<'a>(&'a self, client_index: c_int, name: &CStr) -> &'a CStr {
		let ptr = unsafe { virtual_call!(self.as_object() => get_client_convar_value(client_index, name.as_ptr())) };
		unsafe { CStr::from_ptr(ptr) }
	}
	fn log_print(&mut self, message: &CStr) {
		unsafe { virtual_call!(self.as_object() => log_print(message.as_ptr())) }
	}
	fn is_paused(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_paused()) }
	}
	fn is_in_commentary_mode(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_in_commentary_mode()) }
	}
	fn get_app_id(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => get_app_id()) }
	}
	fn is_low_violence(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_low_violence()) }
	}
	fn insert_server_command(&mut self, command: &CStr) {
		unsafe { virtual_call!(self.as_object() => insert_server_command(command.as_ptr())) }
	}
	fn get_game_server_steam_id(&self) -> Option<&SteamId> {
		unsafe { virtual_call!(self.as_object() => get_game_server_steam_id()).as_ref() }
	}
	fn get_server_version(&self) -> c_int {
		unsafe { virtual_call!(self.as_object() => get_server_version()) }
	}
	fn get_server_time(&self) -> c_float {
		unsafe { virtual_call!(self.as_object() => get_server_time()) }
	}
	fn set_paused_forced(&mut self, paused: bool) {
		unsafe { virtual_call!(self.as_object() => set_paused_forced(paused, -1.0)) }
	}
	fn set_paused_forced_for(&mut self, paused: bool, duration: Duration) {
		unsafe { virtual_call!(self.as_object() => set_paused_forced(paused, duration.as_secs_f64() as _)) }
	}
}
impl<T: AsObject<VEngineServerVt>> VEngineServerImpl for T {}

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
