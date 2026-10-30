use ::core::ffi::{
	c_char, c_float, c_int,
	c_long,
};
use ::rse_cpp::{
	class,
	RefConst, RefMut,
};
use ::rse_math::Color;
use ::rse_shared::cppdef::KeyValues;
use ::rse_utl::cppdef::CUtlVector;

use super::{
	super::{
		IClientPanel,
		HCursor, HFont,
		VPANEL,
		IBorder,
	},
	PHandle, DHANDLE,
	Menu, BaseTooltip,
};

class! {
	pub Panel: pub virtual IClientPanel {
		pub virtual ~destructors();

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
	
		// TODO: `Panel::SetPaintBorderEnabled` and many, many others.

		pub pass_unhandled_input: bool;
		pub last_nav_direction: NavDirection;

		#[cfg(feature = "drag-and-drop")]
		pub drag_drop: *mut DragDrop_t;
		#[cfg(feature = "drag-and-drop")]
		pub clr_drag_frame: Color;
		#[cfg(feature = "drag-and-drop")]
		pub clr_drop_frame: Color;

		pub tooltips: *mut BaseTooltip;
		pub tooltip_overriden: bool;

		pub skip_child: PHandle;
		pub last_double_press_time: c_long;
		pub info_font: HFont;

		#[cfg(feature = "key-binding-maps")]
		pub key_bindings_context: KeyBindingContextHandle_t;

		pub vpanel: VPANEL;
		pub panel_name: *mut c_char;
		pub border: *mut IBorder;

		// TODO: `CUtlFlags< unsigned short > _flags` and others.
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum NavDirection {
	Up,
	Down,
	Left,
	Right,
	Back,
	None,
}

#[cfg(feature = "drag-and-drop")]
#[repr(C)]
pub struct DragDrop_t {
	pub drag_enabled: bool,
	pub show_drag_helper: bool,
	pub dragging: bool,
	pub drag_started: bool,
	pub drag_start_tolerance: c_int,
	pub start_pos: [c_int; 2],
	pub last_pos: [c_int; 2],
	pub drag_data: CUtlVector<*mut KeyValues>,
	pub drag_panels: CUtlVector<PHandle>,
	pub drop_enabled: bool,
	pub hover_context_time: c_float,
	pub current_drop: PHandle,
	pub drop_hover_time: c_long,
	pub drop_menu_shown: bool,
	pub drop_context_menu: DHANDLE<Menu>,
	pub prevent_chaining: bool,
}

// TODO: Is this correct for an `enum` defined like this?
#[cfg(feature = "key-binding-maps")]
#[allow(non_camel_case_types)]
pub type KeyBindingContextHandle_t = c_int;
pub const INVALID_KEYBINDINGCONTEXT_HANDLE: KeyBindingContextHandle_t = (0xffffffff as ::core::ffi::c_uint) as _;
