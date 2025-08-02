use ::core::{
	ffi::{
		CStr, c_int, c_float,
	},
	marker::PhantomData,
	ops::Deref,
	ptr::null,
};
use ::rse_cpp::{
	VtObject, virtual_call,
};
use crate::cppdef::GameEventVt;

#[repr(transparent)]
pub struct GameEvent<'a> {
	ptr: VtObject<GameEventVt>,
	_life: PhantomData<fn(&'a mut GameEventVt)>,
}

impl<'a> GameEvent<'a> {
	/// # Safety
	/// `ptr` must be a valid `IGameEvent` object.
	pub const unsafe fn from_ptr(ptr: VtObject<GameEventVt>) -> Self {
		Self {
			ptr,
			_life: PhantomData,
		}
	}

	pub fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.ptr, get_name)) }
	}

	pub fn is_reliable(&self) -> bool {
		unsafe { virtual_call!(self.ptr, is_reliable) }
	}

	pub fn is_local(&self) -> bool {
		unsafe { virtual_call!(self.ptr, is_local) }
	}

	pub fn is_key_empty(&self, key: &CStr) -> bool {
		unsafe { virtual_call!(self.ptr, is_empty, key.as_ptr()) }
	}

	pub fn get_bool(&self, key: &CStr, default: bool) -> bool {
		unsafe { virtual_call!(self.ptr, get_bool, key.as_ptr(), default) }
	}

	pub fn get_int(&self, key: &CStr, default: c_int) -> c_int {
		unsafe { virtual_call!(self.ptr, get_int, key.as_ptr(), default) }
	}

	pub fn get_float(&self, key: &CStr, default: c_float) -> c_float {
		unsafe { virtual_call!(self.ptr, get_float, key.as_ptr(), default) }
	}

	pub fn get_string_or(&self, key: &CStr, default: &'a CStr) -> &'a CStr {
		let ptr = unsafe { virtual_call!(self.ptr, get_string, key.as_ptr(), default.as_ptr()) };
		unsafe { CStr::from_ptr(ptr) }
	}

	pub fn get_string(&self, key: &CStr) -> Option<&'a CStr> {
		let ptr = unsafe { virtual_call!(self.ptr, get_string, key.as_ptr(), null()) };
		if !ptr.is_null() {
			unsafe { Some(CStr::from_ptr(ptr)) }
		} else {
			None
		}
	}

	// TODO: `IGameEvent::Get*`.
}

#[repr(transparent)]
pub struct GameEventMut<'a> {
	event: GameEvent<'a>,
}

impl<'a> Deref for GameEventMut<'a> {
	type Target = GameEvent<'a>;
	fn deref(&self) -> &Self::Target {
		&self.event
	}
}

impl GameEventMut<'_> {
	/// # Safety
	/// `ptr` must be a valid `IGameEvent` object.
	pub const unsafe fn from_ptr(ptr: VtObject<GameEventVt>) -> Self {
		Self {
			event: unsafe { GameEvent::from_ptr(ptr) },
		}
	}

	pub fn set_bool(&mut self, key: &CStr, value: bool) {
		unsafe { virtual_call!(self.event.ptr, set_bool, key.as_ptr(), value) }
	}

	pub fn set_int(&mut self, key: &CStr, value: c_int) {
		unsafe { virtual_call!(self.event.ptr, set_int, key.as_ptr(), value) }
	}

	pub fn set_float(&mut self, key: &CStr, value: c_float) {
		unsafe { virtual_call!(self.event.ptr, set_float, key.as_ptr(), value) }
	}

	pub fn set_string(&mut self, key: &CStr, value: &CStr) {
		unsafe { virtual_call!(self.event.ptr, set_string, key.as_ptr(), value.as_ptr()) }
	}

	// TODO: `IGameEvent::Set*`.
}
