use ::rse_cpp::class;

use super::super::HPanel;

class! {
	pub DHANDLE<PanelType>: pub PHandle {
		pub use PanelType as _t;
	}
}

class! {
	pub PHandle {
		pub panel_id: HPanel;
	}
}
