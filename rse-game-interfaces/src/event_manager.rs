use ::core::ffi::CStr;
use ::rse_cpp::VtObject;

use crate::{
	cppdef::{
		GameEventManager2Vt, INTERFACEVERSION_GAMEEVENTSMANAGER2,
	},
	InterfaceOfFactory, AppSystemFactory,
};

#[repr(transparent)]
pub struct EventManager {
	ptr: VtObject<GameEventManager2Vt>,
}
unsafe impl ::rse_interface::Interface for EventManager {
	const IDENTIFIER: &CStr = INTERFACEVERSION_GAMEEVENTSMANAGER2;
}
impl ::rse_interface::VTableInterface for EventManager {
	type VTable = GameEventManager2Vt;
	unsafe fn from_ptr(ptr: VtObject<Self::VTable>) -> Self {
		unsafe { Self::from_ptr_const(ptr) }
	}
}
impl InterfaceOfFactory for EventManager {
	type Factory = AppSystemFactory;
}

macro_rules! gem_call {
	($this:expr, $func:ident $(, $arg:expr)* $(,)?) => {
		::cppdvt::virtual_call_raw!($this, (**$this.as_ref()).game_event_manager2.$func, $($arg),*)
	};
}

impl EventManager {
	/// # Safety
	/// `ptr` must be a valid `GAMEEVENTSMANAGER002` interface.
	pub const unsafe fn from_ptr_const(ptr: VtObject<GameEventManager2Vt>) -> Self {
		Self {
			ptr,
		}
	}

	pub const fn as_cpp_ptr(&mut self) -> VtObject<GameEventManager2Vt> {
		self.ptr
	}
}
