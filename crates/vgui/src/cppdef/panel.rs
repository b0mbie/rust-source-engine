use ::core::ffi::{
	CStr, c_char, c_int,
};
use ::rse_cpp::{
	class,
	RefMut,
};
use ::rse_interface::cppdef::IBaseInterface;
use ::rse_shared::cppdef::KeyValues;
use ::rse_utl::cppdef::CUtlVector;

use super::{
	controls::Panel,
	HScheme,
	IClientPanel,
	VPANEL,
};

class! {
	pub IPanel: pub virtual IBaseInterface {
		pub virtual fn init(p: VPANEL, panel: *mut IClientPanel);

		pub virtual fn set_pos(p: VPANEL, x: c_int, y: c_int);
		pub virtual fn get_pos(p: VPANEL, x: RefMut<c_int>, y: RefMut<c_int>);
		pub virtual fn set_size(p: VPANEL, wide: c_int, tall: c_int);
		pub virtual fn get_size(p: VPANEL, wide: RefMut<c_int>, tall: RefMut<c_int>);
		pub virtual fn set_minimum_size(p: VPANEL, wide: c_int, tall: c_int);
		pub virtual fn get_minimum_size(p: VPANEL, wide: RefMut<c_int>, tall: RefMut<c_int>);
		pub virtual fn set_z_pos(p: VPANEL, z: c_int);
		pub virtual fn get_z_pos(p: VPANEL) -> c_int;

		pub virtual fn get_abs_pos(p: VPANEL, x: RefMut<c_int>, y: RefMut<c_int>);
		pub virtual fn get_clip_rect(
			p: VPANEL,
			x0: RefMut<c_int>, y0: RefMut<c_int>,
			x1: RefMut<c_int>, y1: RefMut<c_int>,
		);
		pub virtual fn set_inset(
			p: VPANEL,
			x0: c_int, y0: c_int,
			x1: c_int, y1: c_int,
		);
		pub virtual fn get_inset(
			p: VPANEL,
			x0: RefMut<c_int>, y0: RefMut<c_int>,
			x1: RefMut<c_int>, y1: RefMut<c_int>,
		);

		pub virtual fn set_visible(p: VPANEL, state: bool);
		pub virtual fn is_visible(p: VPANEL) -> bool;
		pub virtual fn set_parent(p: VPANEL, parent: VPANEL);
		pub virtual fn get_child_count(p: VPANEL) -> c_int;
		pub virtual fn get_child(p: VPANEL, index: c_int) -> VPANEL;
		pub virtual fn get_children(p: VPANEL) -> RefMut<CUtlVector<VPANEL>>;
		pub virtual fn get_parent(p: VPANEL) -> VPANEL;

		pub virtual fn move_to_front(p: VPANEL);
		pub virtual fn move_to_back(p: VPANEL);
		pub virtual fn has_parent(p: VPANEL, potential_parent: VPANEL) -> bool;
		pub virtual fn is_popup(p: VPANEL) -> bool;
		pub virtual fn set_popup(p: VPANEL, state: bool);
		pub virtual fn is_fully_visible(p: VPANEL) -> bool;

		pub virtual fn get_scheme(p: VPANEL) -> HScheme;
		pub virtual fn is_proportional(p: VPANEL) -> bool;
		pub virtual fn is_auto_delete_set(p: VPANEL) -> bool;
		pub virtual fn delete_panel(p: VPANEL);

		pub virtual fn set_keyboard_input_enabled(p: VPANEL, state: bool);
		pub virtual fn set_mouse_input_enabled(p: VPANEL, state: bool);
		pub virtual fn is_keyboard_input_enabled(p: VPANEL) -> bool;
		pub virtual fn is_mouse_input_enabled(p: VPANEL) -> bool;

		pub virtual fn solve(p: VPANEL);

		pub virtual fn get_name(p: VPANEL) -> *const c_char;
		pub virtual fn get_class_name(p: VPANEL) -> *const c_char;

		pub virtual fn send_message(p: VPANEL, params: *mut KeyValues, from_panel: VPANEL);

		pub virtual fn think(p: VPANEL);
		pub virtual fn perform_apply_scheme_settings(p: VPANEL);
		pub virtual fn paint_traverse(p: VPANEL, force_repaint: bool, allow_force: bool);
		pub virtual fn repaint(p: VPANEL);
		pub virtual fn is_within_traverse(p: VPANEL, x: c_int, y: c_int, traverse_popups: bool) -> VPANEL;
		pub virtual fn on_child_added(p: VPANEL, child: VPANEL) -> VPANEL;
		pub virtual fn on_size_changed(p: VPANEL, new_wide: c_int, new_tall: c_int) -> VPANEL;
	
		pub virtual fn internal_focus_changed(p: VPANEL, lost: bool) -> VPANEL;
		pub virtual fn request_info(p: VPANEL, output_data: *mut KeyValues) -> bool;
		pub virtual fn request_focus(p: VPANEL, direction: c_int);
		pub virtual fn request_focus_prev(p: VPANEL, existing_panel: VPANEL) -> bool;
		pub virtual fn request_focus_next(p: VPANEL, existing_panel: VPANEL) -> bool;
		pub virtual fn get_current_key_focus(p: VPANEL) -> VPANEL;
		pub virtual fn get_tab_position(p: VPANEL) -> c_int;

		pub virtual fn plat(p: VPANEL) -> *mut SurfacePlat;
		pub virtual fn set_plat(p: VPANEL, plat: *mut SurfacePlat);

		pub virtual fn get_panel(p: VPANEL, destination_module: *const c_char) -> *mut Panel;
	
		pub virtual fn is_enabled(p: VPANEL) -> bool;
		pub virtual fn set_enabled(p: VPANEL, state: bool);
	
		pub virtual fn is_topmost_popup(p: VPANEL) -> bool;
		pub virtual fn set_topmost_popup(p: VPANEL, state: bool);

		pub virtual fn set_sibling_pin(
			p: VPANEL, new_sibling: VPANEL,
			my_corner_to_pin: u8, sibling_corner_to_pin_to: u8,
		);
	}
}

class! {
	// TODO: `SurfacePlat`.
	pub SurfacePlat {}
}

pub const VGUI_PANEL_INTERFACE_VERSION: &CStr = c"VGUI_Panel009";
