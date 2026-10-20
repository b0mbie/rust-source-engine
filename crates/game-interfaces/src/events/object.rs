use ::core::{
	ffi::{
		CStr, c_int, c_float,
	},
	ptr::null,
};
use ::rse_cpp::{
	transparent_wrapper, virtual_call,
};

use crate::cppdef::IGameEvent;

transparent_wrapper! {
	pub struct GameEvent for IGameEvent as "IGameEvent";
}

impl GameEvent {
	pub fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_inner() => get_name())) }
	}
	pub fn is_reliable(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_reliable()) }
	}
	pub fn is_local(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_local()) }
	}
	pub fn is_key_empty(&self, key: &CStr) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_empty(key.as_ptr())) }
	}
	pub fn get_bool_or(&self, key: &CStr, default: bool) -> bool {
		unsafe { virtual_call!(self.as_inner() => get_bool(key.as_ptr(), default)) }
	}
	pub fn get_int_or(&self, key: &CStr, default: c_int) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_int(key.as_ptr(), default)) }
	}
	pub fn get_float_or(&self, key: &CStr, default: c_float) -> c_float {
		unsafe { virtual_call!(self.as_inner() => get_float(key.as_ptr(), default)) }
	}
	pub fn get_string_or(&self, key: &CStr, default: &CStr) -> &CStr {
		let ptr = unsafe { virtual_call!(self.as_inner() => get_string(key.as_ptr(), default.as_ptr())) };
		unsafe { CStr::from_ptr(ptr) }
	}
	pub fn get_string(&self, key: &CStr) -> Option<&CStr> {
		let ptr = unsafe { virtual_call!(self.as_inner() => get_string(key.as_ptr(), null())) };
		if !ptr.is_null() {
			unsafe { Some(CStr::from_ptr(ptr)) }
		} else {
			None
		}
	}
	// TODO: `IGameEvent::Get*`.

	pub fn set_bool(&mut self, key: &CStr, value: bool) {
		unsafe { virtual_call!(self.as_mut_inner() => set_bool(key.as_ptr(), value)) }
	}
	pub fn set_int(&mut self, key: &CStr, value: c_int) {
		unsafe { virtual_call!(self.as_mut_inner() => set_int(key.as_ptr(), value)) }
	}
	pub fn set_float(&mut self, key: &CStr, value: c_float) {
		unsafe { virtual_call!(self.as_mut_inner() => set_float(key.as_ptr(), value)) }
	}
	pub fn set_string(&mut self, key: &CStr, value: &CStr) {
		unsafe { virtual_call!(self.as_mut_inner() => set_string(key.as_ptr(), value.as_ptr())) }
	}
	// TODO: `IGameEvent::Set*`.
}
