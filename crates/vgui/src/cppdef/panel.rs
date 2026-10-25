use ::core::ffi::CStr;
use ::rse_cpp::class;
use ::rse_interface::cppdef::IBaseInterface;

use super::{
	IClientPanel,
	VPANEL,
};

class! {
	pub IPanel: pub virtual IBaseInterface {
		pub virtual fn init(p: VPANEL, panel: *mut IClientPanel);

		// TODO: `IPanel::SetPos` and others.
	}
}

pub const VGUI_PANEL_INTERFACE_VERSION: &CStr = c"VGUI_Panel009";
