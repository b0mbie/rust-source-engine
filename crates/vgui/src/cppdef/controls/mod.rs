use ::core::ffi::{
	c_char, c_float, c_int,
};
use ::rse_cpp::{
	class,
	RefConst, RefMut,
};
use ::rse_math::Color;
use ::rse_shared::cppdef::KeyValues;
use ::rse_utl::cppdef::CUtlVector;

use super::{
	IClientPanel,
	HCursor,
	VPANEL,
	IBorder,
};

class! {
	pub Panel: pub virtual IClientPanel {
		pub virtual fn destructor();
		#[cfg(not(windows))]
		pub virtual fn destructor_2();

		pub virtual fn get_vpanel() -> VPANEL;
		pub virtual fn init(x: c_int, y: c_int, wide: c_int, tall: c_int);

		pub virtual fn set_visible(state: bool);
		pub virtual fn is_visible() -> bool;

		pub virtual fn is_within_traverse(x: c_int, y: c_int, traverse_popups: bool) -> VPANEL;
		pub virtual fn post_message_v(target: VPANEL, message: *mut KeyValues, delay_seconds: c_float);

		pub virtual fn on_move();

		pub virtual fn get_parent() -> *mut Panel;
		pub virtual fn get_v_parent() -> VPANEL;
		pub virtual fn set_parent(new_parent: *mut Panel);
		pub virtual fn set_v_parent(new_parent: VPANEL);
		pub virtual fn has_parent(potential_parent: VPANEL) -> bool;

		pub virtual fn get_children() -> RefConst<CUtlVector<VPANEL>>;

		pub virtual fn set_auto_delete(state: bool);
		pub virtual fn is_auto_delete_set() -> bool;
		pub virtual fn delete_panel();

		pub virtual fn add_action_signal_target(message_target: *mut Panel);
		pub virtual fn add_action_signal_target_v(message_target: VPANEL);
		pub virtual fn remove_action_signal_target(old_target: *mut Panel);
		pub virtual fn post_action_signal(message: *mut KeyValues);
		pub virtual fn request_info_from_child(child_name: *const c_char, output_data: *mut KeyValues) -> bool;
		pub virtual fn post_message_to_child(child_name: *const c_char, message: *mut KeyValues);
		pub virtual fn post_message(target: *mut Panel, message: *mut KeyValues, delay_seconds: c_float);
		pub virtual fn request_info(output_data: *mut KeyValues) -> bool;
		pub virtual fn set_info(input_data: *mut KeyValues) -> bool;
		pub virtual fn set_silent_mode(silent: bool);

		pub virtual fn install_mouse_handler(handler: *mut Panel, this_handles_as_well: bool, movement_events: bool);

		pub virtual fn set_enabled(state: bool);
		pub virtual fn is_enabled() -> bool;
		pub virtual fn is_popup() -> bool;
		pub virtual fn get_clip_rect(x0: RefMut<c_int>, y0: RefMut<c_int>, x1: RefMut<c_int>, y1: RefMut<c_int>);
		pub virtual fn move_to_front();

		pub virtual fn set_bg_color(color: Color);
		pub virtual fn set_fg_color(color: Color);
		pub virtual fn get_bg_color() -> Color;
		pub virtual fn get_fg_color() -> Color;

		pub virtual fn set_cursor(cursor: HCursor);
		pub virtual fn get_cursor() -> HCursor;
		pub virtual fn set_cursor_always_visible(visible: bool);
		pub virtual fn request_focus(direction: c_int);
		pub virtual fn has_focus() -> bool;
		pub virtual fn invalidate_layout(layout_now: bool, reload_scheme: bool);
		pub virtual fn request_focus_prev(panel: VPANEL) -> bool;
		pub virtual fn request_focus_next(panel: VPANEL) -> bool;
		pub virtual fn set_tab_position(position: c_int);
		pub virtual fn get_tab_position() -> c_int;
		pub virtual fn set_border(border: *mut IBorder);
		pub virtual fn get_border() -> *mut IBorder;
		
		// TODO: `Panel::SetBorder` and many, many others.
	}
}
