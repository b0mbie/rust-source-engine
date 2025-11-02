use ::core::{
	ffi::{
		CStr, c_int, c_float,
	},
	ptr::null,
};
use ::rse_cpp::{
	AsObject, virtual_call, vt_object_wrapper,
};

use crate::cppdef::GameEventVt;

pub trait GameEventImpl: AsObject<GameEventVt> {
	fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_object() => get_name())) }
	}
	fn is_reliable(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_reliable()) }
	}
	fn is_local(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => is_local()) }
	}
	fn is_key_empty(&self, key: &CStr) -> bool {
		unsafe { virtual_call!(self.as_object() => is_empty(key.as_ptr())) }
	}
	fn get_bool_or(&self, key: &CStr, default: bool) -> bool {
		unsafe { virtual_call!(self.as_object() => get_bool(key.as_ptr(), default)) }
	}
	fn get_int_or(&self, key: &CStr, default: c_int) -> c_int {
		unsafe { virtual_call!(self.as_object() => get_int(key.as_ptr(), default)) }
	}
	fn get_float_or(&self, key: &CStr, default: c_float) -> c_float {
		unsafe { virtual_call!(self.as_object() => get_float(key.as_ptr(), default)) }
	}
	fn get_string_or(&self, key: &CStr, default: &CStr) -> &CStr {
		let ptr = unsafe { virtual_call!(self.as_object() => get_string(key.as_ptr(), default.as_ptr())) };
		unsafe { CStr::from_ptr(ptr) }
	}
	fn get_string(&self, key: &CStr) -> Option<&CStr> {
		let ptr = unsafe { virtual_call!(self.as_object() => get_string(key.as_ptr(), null())) };
		if !ptr.is_null() {
			unsafe { Some(CStr::from_ptr(ptr)) }
		} else {
			None
		}
	}
	// TODO: `IGameEvent::Get*`.

	fn set_bool(&mut self, key: &CStr, value: bool) {
		unsafe { virtual_call!(self.as_object() => set_bool(key.as_ptr(), value)) }
	}
	fn set_int(&mut self, key: &CStr, value: c_int) {
		unsafe { virtual_call!(self.as_object() => set_int(key.as_ptr(), value)) }
	}
	fn set_float(&mut self, key: &CStr, value: c_float) {
		unsafe { virtual_call!(self.as_object() => set_float(key.as_ptr(), value)) }
	}
	fn set_string(&mut self, key: &CStr, value: &CStr) {
		unsafe { virtual_call!(self.as_object() => set_string(key.as_ptr(), value.as_ptr())) }
	}
	// TODO: `IGameEvent::Set*`.
}
impl<T: ?Sized + AsObject<GameEventVt>> GameEventImpl for T {}

vt_object_wrapper! {
	pub struct GameEvent for GameEventVt;
}
