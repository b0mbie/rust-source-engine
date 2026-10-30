use ::core::ffi::CStr;
use ::rse_interface::{
	Interface,
	ptr_interface,
};

use crate::cppdef::{
	ISoundSystem, SOUNDSYSTEM_INTERFACE_VERSION,
};

ptr_interface! {
	pub struct OSoundSystem(SoundSystem);
	Pointee = ISoundSystem;
}
impl Interface for OSoundSystem {
	const IDENTIFIER: &CStr = SOUNDSYSTEM_INTERFACE_VERSION;
}
