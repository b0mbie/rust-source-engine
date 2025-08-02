use ::core::ffi::{
	CStr, c_char, c_int, c_float, c_void,
};
use ::rse_cpp::{
	VtObject, vtable,
	VtObjectMut,
};
use ::rse_interface::cppdef::BaseInterfaceVt;

use super::{
	KeyValues,
	BfRead, BfWrite,
	wchar_t,
};

/// Maximum game event name length.
pub const MAX_EVENT_NAME_LENGTH: usize = 32;
/// Maximum number of bits needed for an event index.
pub const MAX_EVENT_BITS: u32 = 9;
/// Maximum number of events allowed.
pub const MAX_EVENT_NUMBER: usize = 1 << MAX_EVENT_BITS;
/// Maximum size, in bytes, for a serialized event.
pub const MAX_EVENT_BYTES: usize = 1024;

pub const INTERFACEVERSION_GAMEEVENTSMANAGER2: &CStr = c"GAMEEVENTSMANAGER002";

#[repr(C)]
pub struct GameEventManager2Vt {
	pub base: BaseInterfaceVt,
	pub game_event_manager2: GameEventManager2VtBase,
}

vtable! {
	pub GameEventManager2VtBase for VtObject<GameEventManager2Vt> {
		pub fn load_events_from_file(filename: *const c_char) -> c_int;
		pub fn reset();
		pub fn add_listener(listener: VtObjectMut<GameEventListener2Vt>, name: *const c_char, server_side: bool) -> bool;
		pub fn find_listener(listener: VtObjectMut<GameEventListener2Vt>, name: *const c_char) -> bool;
		pub fn remove_listener(listener: VtObjectMut<GameEventListener2Vt>);
		pub fn create_event(name: *const c_char, force: bool) -> Option<VtObjectMut<GameEventVt>>;
		pub fn fire_event(event: VtObjectMut<GameEventVt>, dont_broadcast: bool) -> bool;
		pub fn fire_event_client_side(event: VtObjectMut<GameEventVt>) -> bool;
		pub fn duplicate_event(event: VtObjectMut<GameEventVt>) -> VtObjectMut<GameEventVt>;
		pub fn free_event(event: VtObjectMut<GameEventVt>);
		pub fn serialize_event(event: VtObjectMut<GameEventVt>, buf: BfWrite) -> bool;
		pub fn unserialize_event(buf: *mut BfRead) -> VtObject<GameEventVt>;
	}
}

vtable! {
	pub GameEventListener2Vt {
		pub fn destructor();
		#[cfg(not(target_os = "windows"))]
		pub fn destructor_2();
		pub fn fire_game_event(event: VtObjectMut<GameEventVt>);
	}
}

vtable! {
	pub GameEventVt {
		pub fn destructor();
		#[cfg(not(target_os = "windows"))]
		pub fn destructor_2();
		pub fn get_name() -> *const c_char;
		pub fn is_reliable() -> bool;
		pub fn is_local() -> bool;
		pub fn is_empty(key_name: *const c_char) -> bool;
		pub fn get_bool(key_name: *const c_char, default_value: bool) -> bool;
		pub fn get_int(key_name: *const c_char, default_value: c_int) -> c_int;
		pub fn get_float(key_name: *const c_char, default_value: c_float) -> c_float;
		pub fn get_string(key_name: *const c_char, default_value: *const c_char) -> *const c_char;
		pub fn set_bool(key_name: *const c_char, value: bool);
		pub fn set_int(key_name: *const c_char, value: c_int);
		pub fn set_float(key_name: *const c_char, value: c_float);
		pub fn set_string(key_name: *const c_char, value: *const c_char);
		pub fn get_uint64(key_name: *const c_char, default_value: u64) -> u64;
		pub fn get_w_string(key_name: *const c_char, default_value: *const wchar_t) -> *const wchar_t;
		pub fn get_ptr(key_name: *const c_char) -> *const c_void;
		pub fn set_uint64(key_name: *const c_char, value: u64);
		pub fn set_w_string(key_name: *const c_char, value: *const wchar_t);
		pub fn set_ptr(key_name: *const c_char, value: *const c_void);
		pub fn get_data_keys() -> *mut KeyValues;
		pub fn for_event_data(visitor: VtObjectMut<GameEventVisitor2Vt>);
	}
}

vtable! {
	pub GameEventVisitor2Vt {
		pub fn visit_local(name: *const c_char, value: *const c_void) -> bool;
		pub fn visit_string(name: *const c_char, value: *const c_char) -> bool;
		pub fn visit_float(name: *const c_char, value: c_float) -> bool;
		pub fn visit_int(name: *const c_char, value: c_int) -> bool;
		pub fn visit_uint64(name: *const c_char, value: u64) -> bool;
		pub fn visit_w_string(name: *const c_char, value: *const wchar_t) -> bool;
		pub fn visit_bool(name: *const c_char, value: bool) -> bool;
	}
}
