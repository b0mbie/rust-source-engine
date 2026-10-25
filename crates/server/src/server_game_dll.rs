use ::core::ffi::{
	CStr, c_int,
};
use ::rse_cpp::{
	util::clamp_len_to_c_int,
	RefMut,
	virtual_call,
};
use ::rse_game_interfaces::{
	InterfaceOfFactory, GameServerFactory,
};
use ::rse_shared::{
	ServerClasses, ServerClassesMut, ServerClass,
};

use crate::cppdef::{
	IServerGameDLL, INTERFACEVERSION_SERVERGAMEDLL,
	TickInterval,
};

::rse_interface::ptr_interface! {
	pub struct OServerGameDll(ServerGameDll);
	Pointee = IServerGameDLL;
}
impl ::rse_interface::Interface for OServerGameDll {
	const IDENTIFIER: &CStr = INTERFACEVERSION_SERVERGAMEDLL;
}
impl InterfaceOfFactory for OServerGameDll {
	type Factory = GameServerFactory;
}

impl ServerGameDll {
	pub fn tick_interval(&self) -> TickInterval {
		unsafe { virtual_call!(self.as_inner() => get_tick_interval()) } 
	}
	pub fn server_classes(&self) -> ServerClasses<'_> {
		let head = unsafe { ServerClass::from_ptr(virtual_call!(self.as_inner() => get_all_server_classes())) };
		ServerClasses::new(head)
	}
	pub fn server_classes_mut(&mut self) -> ServerClassesMut<'_> {
		let head = unsafe { ServerClass::from_mut_ptr(virtual_call!(self.as_mut_inner() => get_all_server_classes())) };
		ServerClassesMut::new(head)
	}
	pub fn game_description(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_inner() => get_game_description())) }
	}

	pub fn get_user_message_info(&self, index: c_int, name: &mut [u8]) -> Option<UserMessageInfo> {
		let mut size = 0;
		if self.get_user_message_info_raw(index, name, &mut size) {
			Some(UserMessageInfo {
				size: if size != -1 {
					Some(size as _)
				} else {
					None
				}
			})
		} else {
			None
		}
	}

	pub fn get_user_message_info_raw(&self, index: c_int, name: &mut [u8], size: &mut c_int) -> bool {
		unsafe { virtual_call!(self.as_inner() => get_user_message_info(
			index,
			name.as_mut_ptr().cast(), clamp_len_to_c_int(name.len()),
			RefMut::from_mut(size),
		)) }
	}

	// TODO: More functions from `IServerGameDLL`.
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserMessageInfo {
	/// Size of the user message, or `None` if it is dynamically-sized.
	pub size: Option<usize>,
}
