use ::core::ffi::CStr;
use ::rse_cpp::{
	transparent_wrapper,
	virtual_call,
};
use ::rse_shared::ServerEdict;
use ::rse_interface::{
	Interface, ptr_interface,
};

use crate::{
	cppdef::{
		IBotManager, IBotController,
		INTERFACEVERSION_PLAYERBOTMANAGER,
	},
	InterfaceOfFactory, GameServerFactory,
};

ptr_interface! {
	pub struct OBotManager(BotManager);
	Pointee = IBotManager;
}
unsafe impl Send for OBotManager {}
unsafe impl Sync for OBotManager {}
impl Interface for OBotManager {
	const IDENTIFIER: &CStr = INTERFACEVERSION_PLAYERBOTMANAGER;
}
impl InterfaceOfFactory for OBotManager {
	type Factory = GameServerFactory;
}

impl BotManager {
	// TODO: Is it possible to make it so that `create_bot` can be used with `bot_controller_for` with `mut`?
	pub fn bot_controller_for<'a>(&self, edict: &'a ServerEdict) -> Option<&'a BotController> {
		unsafe {
			let ptr = virtual_call!(self.as_inner() => get_bot_controller(edict.as_ptr() as _));
			Some(BotController::from_mut(ptr.as_mut()?))
		}
	}
	pub fn create_bot(&self, name: &CStr) -> Option<&ServerEdict> {
		unsafe {
			let ptr = virtual_call!(self.as_inner() => create_bot(name.as_ptr()));
			if !ptr.is_null() {
				Some(ServerEdict::from_c_edict(&*ptr))
			} else {
				None
			}
		}
	}
	pub fn bot(&self, name: &CStr) -> Option<&BotController> {
		let edict = self.create_bot(name)?;
		self.bot_controller_for(edict)
	}
}

transparent_wrapper! {
	pub struct BotController for IBotController as "IBotController";
}
// TODO: `IBotController` methods.
