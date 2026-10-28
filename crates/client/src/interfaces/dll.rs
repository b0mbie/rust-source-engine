use ::core::ffi::CStr;
use ::rse_game_interfaces::InterfaceOfFactory;
use ::rse_interface::{
	Interface,
	ptr_interface,
};

use crate::cppdef::{
	IBaseClientDLL,
	CLIENT_DLL_INTERFACE_VERSION,
};

ptr_interface! {
	pub struct OBaseClientDLL(BaseClientDLL);
	Pointee = IBaseClientDLL;
}
impl Interface for OBaseClientDLL {
	const IDENTIFIER: &CStr = CLIENT_DLL_INTERFACE_VERSION;
}
impl InterfaceOfFactory for OBaseClientDLL {
	type Factory = ::rse_game_interfaces::GameServerFactory;
}
