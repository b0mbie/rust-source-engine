use ::core::ffi::CStr;
use ::rse_cpp::{
	AsObject, virtual_call, owned_vt_object_wrapper,
};
use ::rse_game::{
	ServerClasses, ServerClassesMut, ServerClass,
};

use crate::{
	cppdef::{
		ServerGameDllVt, INTERFACEVERSION_SERVERGAMEDLL,
		TickInterval,
	},
	InterfaceOfFactory, GameServerFactory,
};

pub trait ServerGameDllImpl: AsObject<ServerGameDllVt> {
	fn tick_interval(&self) -> TickInterval {
		unsafe { virtual_call!(self.as_object() => get_tick_interval()) } 
	}
	fn server_classes(&self) -> ServerClasses<'_> {
		let head = unsafe { ServerClass::from_ptr(virtual_call!(self.as_object() => get_all_server_classes())) };
		ServerClasses::new(head)
	}
	fn server_classes_mut(&mut self) -> ServerClassesMut<'_> {
		let head = unsafe { ServerClass::from_mut_ptr(virtual_call!(self.as_object() => get_all_server_classes())) };
		ServerClassesMut::new(head)
	}
	fn game_description(&self) -> &CStr {
		unsafe { CStr::from_ptr(virtual_call!(self.as_object() => get_game_description())) }
	}

	// TODO: More functions from `ServerGameDllVt`.
}
impl<T: AsObject<ServerGameDllVt>> ServerGameDllImpl for T {}

owned_vt_object_wrapper! {
	pub struct ServerGameDll for ServerGameDllVt;
}
unsafe impl ::rse_interface::Interface for ServerGameDll {
	const IDENTIFIER: &CStr = INTERFACEVERSION_SERVERGAMEDLL;
}
impl InterfaceOfFactory for ServerGameDll {
	type Factory = GameServerFactory;
}
