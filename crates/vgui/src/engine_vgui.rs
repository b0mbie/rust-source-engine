use ::core::ffi::CStr;
use ::rse_cpp::virtual_call;
use ::rse_game_interfaces::InterfaceOfFactory;
use ::rse_interface::{
	Interface,
	ptr_interface,
};

use crate::cppdef::{
	IEngineVGui, VENGINE_VGUI_VERSION,
	VGuiPanel_t, VPANEL,
};

impl EngineVGui {
	pub fn get_panel(&self, ty: VGuiPanel_t) -> VPANEL {
		unsafe { virtual_call!(self.as_inner() => get_panel(ty)) }
	}

	pub fn is_game_ui_visible(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_game_ui_visible()) }
	}
}

ptr_interface! {
	pub struct OEngineVGui(EngineVGui);
	Pointee = IEngineVGui;
}
impl Interface for OEngineVGui {
	const IDENTIFIER: &CStr = VENGINE_VGUI_VERSION;
}
impl InterfaceOfFactory for OEngineVGui {
	type Factory = ::rse_game_interfaces::AppSystemFactory;
}
