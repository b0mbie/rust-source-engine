use ::core::ffi::{
	CStr, c_char, c_int, c_float, c_void,
};
use ::rse_cpp::{
	wide_c_str::wchar_t,
	class,
};
use ::rse_shared::cppdef::{
	KeyValues,
	bf_read, bf_write,
};
use ::rse_interface::cppdef::IBaseInterface;

/// Maximum game event name length.
pub const MAX_EVENT_NAME_LENGTH: usize = 32;
/// Maximum number of bits needed for an event index.
pub const MAX_EVENT_BITS: u32 = 9;
/// Maximum number of events allowed.
pub const MAX_EVENT_NUMBER: usize = 1 << MAX_EVENT_BITS;
/// Maximum size, in bytes, for a serialized event.
pub const MAX_EVENT_BYTES: usize = 1024;

pub const INTERFACEVERSION_GAMEEVENTSMANAGER2: &CStr = c"GAMEEVENTSMANAGER002";

class! {
	pub IGameEventManager2: pub virtual IBaseInterface {
		pub virtual fn load_events_from_file(filename: *const c_char) -> c_int;
		pub virtual fn reset();
		pub virtual fn add_listener(listener: *mut IGameEventListener2, name: *const c_char, server_side: bool) -> bool;
		pub virtual fn find_listener(listener: *mut IGameEventListener2, name: *const c_char) -> bool;
		pub virtual fn remove_listener(listener: *mut IGameEventListener2);
		pub virtual fn create_event(name: *const c_char, force: bool) -> *mut IGameEvent;
		pub virtual fn fire_event(event: *mut IGameEvent, dont_broadcast: bool) -> bool;
		pub virtual fn fire_event_client_side(event: *mut IGameEvent) -> bool;
		pub virtual fn duplicate_event(event: *mut IGameEvent) -> *mut IGameEvent;
		pub virtual fn free_event(event: *mut IGameEvent);
		pub virtual fn serialize_event(event: *mut IGameEvent, buf: *mut bf_write) -> bool;
		pub virtual fn unserialize_event(buf: *mut bf_read) -> *mut IGameEvent;
	}
}

class! {
	pub IGameEventListener2 {
		pub virtual fn destructor();
		#[cfg(not(windows))]
		pub virtual fn destructor_2();
		pub virtual fn fire_game_event(event: *mut IGameEvent);
	}
}

class! {
	pub IGameEvent {
		pub virtual fn destructor();
		#[cfg(not(windows))]
		pub virtual fn destructor_2();
		pub virtual fn get_name() -> *const c_char;
		pub virtual fn is_reliable() -> bool;
		pub virtual fn is_local() -> bool;
		pub virtual fn is_empty(key_name: *const c_char) -> bool;
		pub virtual fn get_bool(key_name: *const c_char, default_value: bool) -> bool;
		pub virtual fn get_int(key_name: *const c_char, default_value: c_int) -> c_int;
		pub virtual fn get_float(key_name: *const c_char, default_value: c_float) -> c_float;
		pub virtual fn get_string(key_name: *const c_char, default_value: *const c_char) -> *const c_char;
		pub virtual fn set_bool(key_name: *const c_char, value: bool);
		pub virtual fn set_int(key_name: *const c_char, value: c_int);
		pub virtual fn set_float(key_name: *const c_char, value: c_float);
		pub virtual fn set_string(key_name: *const c_char, value: *const c_char);
		pub virtual fn get_uint64(key_name: *const c_char, default_value: u64) -> u64;
		pub virtual fn get_w_string(key_name: *const c_char, default_value: *const wchar_t) -> *const wchar_t;
		pub virtual fn get_ptr(key_name: *const c_char) -> *const c_void;
		pub virtual fn set_uint64(key_name: *const c_char, value: u64);
		pub virtual fn set_w_string(key_name: *const c_char, value: *const wchar_t);
		pub virtual fn set_ptr(key_name: *const c_char, value: *const c_void);
		pub virtual fn get_data_keys() -> *mut KeyValues;
		pub virtual fn for_event_data(visitor: *mut IGameEventVisitor2) -> bool;
	}
}

class! {
	pub IGameEventVisitor2 {
		pub virtual fn visit_local(name: *const c_char, value: *const c_void) -> bool;
		pub virtual fn visit_string(name: *const c_char, value: *const c_char) -> bool;
		pub virtual fn visit_float(name: *const c_char, value: c_float) -> bool;
		pub virtual fn visit_int(name: *const c_char, value: c_int) -> bool;
		pub virtual fn visit_uint64(name: *const c_char, value: u64) -> bool;
		pub virtual fn visit_w_string(name: *const c_char, value: *const wchar_t) -> bool;
		pub virtual fn visit_bool(name: *const c_char, value: bool) -> bool;
	}
}
