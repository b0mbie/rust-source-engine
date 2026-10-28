use ::core::ffi::{
	CStr, c_int,
};
use ::rse_cpp::{
	c_str::opt_c_str_from_ptr,
	virtual_call,
};
use ::rse_game_interfaces::InterfaceOfFactory;
use ::rse_interface::{
	Interface,
	ptr_interface,
};
use ::rse_utl::Vector;

use crate::cppdef::{
	controls::Panel,
	IPanel, VGUI_PANEL_INTERFACE_VERSION,
	VPANEL,
};

impl Panels {
	pub unsafe fn get_child_count(&self, raw: VPANEL) -> c_int {
		unsafe { virtual_call!(self.as_inner() => get_child_count(raw)) }
	}
	pub unsafe fn get_child(&self, raw: VPANEL, which: c_int) -> VPANEL {
		unsafe { virtual_call!(self.as_inner() => get_child(raw, which)) }
	}
	pub unsafe fn get_children(&self, raw: VPANEL) -> &Vector<VPANEL> {
		unsafe { virtual_call!(self.as_inner() => get_children(raw)).cast().as_ref() }
	}
	pub unsafe fn get_name(&self, raw: VPANEL) -> Option<&CStr> {
		unsafe { opt_c_str_from_ptr(virtual_call!(self.as_inner() => get_name(raw))) }
	}
	pub unsafe fn get_class_name(&self, raw: VPANEL) -> Option<&CStr> {
		unsafe { opt_c_str_from_ptr(virtual_call!(self.as_inner() => get_class_name(raw))) }
	}
	pub unsafe fn get_parent(&self, raw: VPANEL) -> VPANEL {
		unsafe { virtual_call!(self.as_inner() => get_parent(raw)) }
	}
	pub unsafe fn get_panel(&self, raw: VPANEL, destination_module: &CStr) -> *mut Panel {
		unsafe { virtual_call!(self.as_inner() => get_panel(raw, destination_module.as_ptr())) }
	}
}

ptr_interface! {
	// N.B. This is named `Panels` and not `Panel` because
	// it could get confused with the `Panel` type from `vgui_controls`,
	// and this name is more fitting.
	pub struct OPanels(Panels);
	Pointee = IPanel;
}
impl Interface for OPanels {
	const IDENTIFIER: &CStr = VGUI_PANEL_INTERFACE_VERSION;
}
impl InterfaceOfFactory for OPanels {
	type Factory = ::rse_game_interfaces::AppSystemFactory;
}
