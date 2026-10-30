use ::core::ffi::{
	c_char, c_int, c_void,
};
use ::rse_cpp::RefMut;
use ::rse_shared::cppdef::KeyValues;

use super::{
	controls::Panel,
	VPANEL, HScheme,
};

::rse_cpp::class! {
	pub IClientPanel {
		pub virtual fn get_vpanel() -> VPANEL;
		pub virtual fn void();
		pub virtual fn perform_apply_scheme_settings();
		pub virtual fn paint_traverse(force_repaint: bool, allow_force: bool);
		pub virtual fn repaint();
		pub virtual fn is_within_traverse(x: c_int, y: c_int, traverse_popups: bool) -> VPANEL;
		pub virtual fn get_inset(
			top: RefMut<c_int>, left: RefMut<c_int>, right: RefMut<c_int>, bottom: RefMut<c_int>,
		);
		pub virtual fn get_clip_rect(x0: RefMut<c_int>, y0: RefMut<c_int>, x1: RefMut<c_int>, y1: RefMut<c_int>);
		pub virtual fn on_child_added(child: VPANEL);
		pub virtual fn on_size_changed(new_wide: c_int, new_tall: c_int);

		pub virtual fn internal_focus_changed(lost: bool);
		pub virtual fn request_info(output_data: *mut KeyValues) -> bool;
		pub virtual fn request_focus(direction: c_int);
		pub virtual fn request_focus_prev(existing_panel: VPANEL) -> bool;
		pub virtual fn request_focus_next(existing_panel: VPANEL) -> bool;
		pub virtual fn on_message(params: *const KeyValues, from_panel: VPANEL);
		pub virtual fn get_current_key_focus() -> VPANEL;
		pub virtual fn get_tab_position() -> c_int;

		pub virtual fn get_name() -> *const c_char;
		pub virtual fn get_class_name() -> *const c_char;
	
		pub virtual fn get_scheme() -> HScheme;
		pub virtual fn is_proportional() -> bool;
		pub virtual fn is_auto_delete_set() -> bool;
		pub virtual fn delete_panel();

		pub virtual fn query_interface(id: EInterfaceID) -> *mut c_void;

		pub virtual fn get_panel() -> *mut Panel;

		pub virtual fn get_module_name() -> *const c_char;

		pub virtual fn on_tick();
	}
}

// TODO: Is this correct for an `enum` defined like this?
pub type EInterfaceID = c_int;
pub const ICLIENTPANEL_STANDARD_INTERFACE: EInterfaceID = 0;
