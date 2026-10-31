use ::core::{
	ffi::CStr,
	ptr::NonNull,
};
use ::rse_cpp::virtual_call;
use ::rse_game_interfaces::{
	InterfaceOfFactory, AppSystemFactory,
};
use ::rse_interface::{
	Interface, ptr_interface,
};

use crate::{
	cppdef::{
		IGameEventManager2, INTERFACEVERSION_GAMEEVENTSMANAGER2,
		IGameEvent,
	},
};

ptr_interface! {
	pub struct OGameEventManager2(GameEventManager2);
	Pointee = IGameEventManager2;
}
unsafe impl Send for OGameEventManager2 {}
unsafe impl Sync for OGameEventManager2 {}
impl Interface for OGameEventManager2 {
	const IDENTIFIER: &CStr = INTERFACEVERSION_GAMEEVENTSMANAGER2;
}
impl InterfaceOfFactory for OGameEventManager2 {
	type Factory = AppSystemFactory;
}

impl GameEventManager2 {
	/// Creates an event by its identifying name,
	/// returning `None` if the event is not known or no listener is registered for it.
	/// 
	/// If `force` is `true`, don't require any listeners to be registered to create this event.
	pub fn create_raw_event(&mut self, name: &CStr, force: bool) -> Option<RawEvent> {
		let ptr = unsafe { virtual_call!(self.as_mut_inner() => create_event(name.as_ptr(), force)) };
		NonNull::new(ptr)
	}

	/// Duplicates `event`.
	/// 
	/// # Safety
	/// `event` must have been created by this interface.
	pub unsafe fn duplicate_raw_event(&mut self, event: RawEvent) -> RawEvent {
		let ptr = unsafe { virtual_call!(self.as_mut_inner() => duplicate_event(event.as_ptr())) };
		// SAFETY: This should never fail given a non-null `event`.
		unsafe { NonNull::new_unchecked(ptr) }
	}

	/// Frees `event`.
	/// 
	/// # Safety
	/// `event` must have been created by this interface.
	/// `event` must not be used again after this function returns.
	pub unsafe fn free_raw_event(&mut self, event: RawEvent) {
		unsafe { virtual_call!(self.as_mut_inner() => free_event(event.as_ptr())) }
	}

	/// Fires `event` on the server,
	/// broadcasting it to all clients if `dont_broadcast` is `false`.
	/// 
	/// # Safety
	/// `event` must have been created by this interface.
	/// This function frees `event` if it returns `true`.
	pub unsafe fn fire_raw_event(&mut self, event: RawEvent, dont_broadcast: bool) -> bool {
		unsafe { virtual_call!(self.as_mut_inner() => fire_event(event.as_ptr(), dont_broadcast)) }
	}

	/// Fires `event` only for the local client.
	/// 
	/// # Safety
	/// `event` must have been created by this interface.
	/// This function frees `event` if it returns `true`.
	/// 
	/// This function should be used only by client code.
	pub unsafe fn fire_raw_event_clientside(&mut self, event: RawEvent) -> bool {
		unsafe { virtual_call!(self.as_mut_inner() => fire_event_client_side(event.as_ptr())) }
	}
}

pub type RawEvent = NonNull<IGameEvent>;
