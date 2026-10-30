use ::core::ffi::CStr;
use ::rse_cpp::class;
use ::rse_shared::cppdef::WorkshopMapDesc_t;

use super::VPANEL;

class! {
	pub IEngineVGui {
		pub virtual ~destructors();
		pub virtual fn get_panel(ty: VGuiPanel_t) -> VPANEL;
		pub virtual fn is_game_ui_visible() -> bool;
		pub virtual fn get_workshop_map(index: u32, desc: *mut WorkshopMapDesc_t) -> bool;
	}
}

pub const VENGINE_VGUI_VERSION: &CStr = c"VEngineVGui002";
pub const VENGINE_VGUI_VERSION_OLD: &CStr = c"VEngineVGui001";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum VGuiPanel_t {
	Root = 0,
	GameUiDll,
	ClientDll,
	Tools,
	InGameScreens,
	GameDll,
	ClientDllTools,
	Sizing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum PaintMode_t {
	UiPanels = 1 << 0,
	InGamePanels = 1 << 1,
	Cursor = 1 << 2,
}
