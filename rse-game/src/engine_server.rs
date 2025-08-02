use ::core::{
	ffi::{
		CStr, c_char, c_int, c_float,
	},
	ptr::NonNull,
	time::Duration,
};
use ::rse_cpp::{
	VtObject, virtual_call,
};

use crate::{
	cppdef::{
		game_dll_interfaces::*,
		SteamId, Vector,
	},
	InterfaceOfFactory, AppSystemFactory,
};

#[derive(Debug)]
#[repr(transparent)]
pub struct VEngineServer {
	ptr: VtObject<VEngineServerVt>,
}

unsafe impl ::rse_interface::Interface for VEngineServer {
	const IDENTIFIER: &CStr = INTERFACEVERSION_VENGINESERVER;
}
impl ::rse_interface::VTableInterface for VEngineServer {
	type VTable = VEngineServerVt;
	unsafe fn from_ptr(ptr: VtObject<Self::VTable>) -> Self {
		unsafe { Self::from_ptr_const(ptr) }
	}
}
impl InterfaceOfFactory for VEngineServer {
	type Factory = AppSystemFactory;
}

impl VEngineServer {
	/// # Safety
	/// `ptr` must be a valid `VEngineServer023` interface.
	pub const unsafe fn from_ptr_const(ptr: VtObject<VEngineServerVt>) -> Self {
		Self {
			ptr,
		}
	}

	pub fn is_map_valid(&self, map_name: &CStr) -> bool {
		(unsafe { virtual_call!(self.ptr, is_map_valid, map_name.as_ptr()) }) != 0
	}

	pub fn is_dedicated_server(&self) -> bool {
		unsafe { virtual_call!(self.ptr, is_dedicated_server) }
	}

	pub fn is_in_edit_mode(&self) -> bool {
		(unsafe { virtual_call!(self.ptr, is_in_edit_mode) }) != 0
	}

	pub fn get_entity_count(&self) -> usize {
		(unsafe { virtual_call!(self.ptr, get_entity_count) }) as _
	}

	pub fn emit_ambient_sound(&mut self, options: EmitSound<'_>) {
		unsafe { virtual_call!(
			self.ptr, emit_ambient_sound,
			options.ent_index, NonNull::from(options.pos),
			options.sample.as_ptr(),
			options.volume,
			options.sound_level,
			options.flags,
			options.pitch,
			options.delay,
		) }
	}

	pub fn server_command(&mut self, command: &CStr) {
		unsafe { virtual_call!(self.ptr, server_command, command.as_ptr()) }
	}

	pub fn time(&self) -> c_float {
		unsafe { virtual_call!(self.ptr, time) }
	}

	pub fn get_game_dir(&mut self, buffer: &mut [c_char]) {
		unsafe { virtual_call!(
			self.ptr, get_game_dir,
			buffer.as_mut_ptr(), buffer.len() as _,
		) }
	}

	pub fn get_client_convar_value<'a>(&'a self, client_index: c_int, name: &CStr) -> &'a CStr {
		let ptr = unsafe { virtual_call!(self.ptr, get_client_convar_value, client_index, name.as_ptr()) };
		unsafe { CStr::from_ptr(ptr) }
	}

	pub fn log_print(&mut self, message: &CStr) {
		unsafe { virtual_call!(self.ptr, log_print, message.as_ptr()) }
	}

	pub fn is_paused(&self) -> bool {
		unsafe { virtual_call!(self.ptr, is_paused) }
	}

	pub fn is_in_commentary_mode(&self) -> bool {
		unsafe { virtual_call!(self.ptr, is_in_commentary_mode) }
	}

	pub fn get_app_id(&self) -> c_int {
		unsafe { virtual_call!(self.ptr, get_app_id) }
	}

	pub fn is_low_violence(&self) -> bool {
		unsafe { virtual_call!(self.ptr, is_low_violence) }
	}

	pub fn insert_server_command(&mut self, command: &CStr) {
		unsafe { virtual_call!(self.ptr, insert_server_command, command.as_ptr()) }
	}

	pub fn get_game_server_steam_id(&self) -> Option<&SteamId> {
		unsafe { virtual_call!(self.ptr, get_game_server_steam_id).as_ref() }
	}

	pub fn get_server_version(&self) -> c_int {
		unsafe { virtual_call!(self.ptr, get_server_version) }
	}

	pub fn get_server_time(&self) -> c_float {
		unsafe { virtual_call!(self.ptr, get_server_time) }
	}

	pub fn set_paused_forced(&mut self, paused: bool) {
		unsafe { virtual_call!(self.ptr, set_paused_forced, paused, -1.0) }
	}

	pub fn set_paused_forced_for(&mut self, paused: bool, duration: Duration) {
		unsafe { virtual_call!(self.ptr, set_paused_forced, paused, duration.as_secs_f64() as _) }
	}
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
