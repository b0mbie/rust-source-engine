use ::core::ffi::CStr;
use ::rse_cpp::{
	AsObject, owned_vt_object_wrapper,
};

use crate::{
	cppdef::{
		GameEventManager2Vt, INTERFACEVERSION_GAMEEVENTSMANAGER2,
	},
	InterfaceOfFactory, AppSystemFactory,
};

pub trait GameEventManager2Impl: AsObject<GameEventManager2Vt> {}
impl<T: ?Sized + AsObject<GameEventManager2Vt>> GameEventManager2Impl for T {}

owned_vt_object_wrapper! {
	pub struct GameEventManager2 for GameEventManager2Vt;
}
unsafe impl ::rse_interface::Interface for GameEventManager2 {
	const IDENTIFIER: &CStr = INTERFACEVERSION_GAMEEVENTSMANAGER2;
}
impl InterfaceOfFactory for GameEventManager2 {
	type Factory = AppSystemFactory;
}
