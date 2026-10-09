use ::core::ffi::CStr;
use ::rse_cpp::virtual_call;
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

	// TODO: More functions from `IServerGameDLL`.
}
