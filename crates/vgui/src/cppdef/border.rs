use ::core::ffi::{
	c_char, c_int,
};
use ::rse_cpp::RefMut;
use ::rse_shared::cppdef::KeyValues;

use super::VPANEL;

::rse_cpp::class! {
	pub IBorder {
		pub virtual fn destructor();
		#[cfg(not(windows))]
		pub virtual fn destructor_2();

		pub virtual fn paint(panel: VPANEL);
		pub virtual fn paint_at(
			x0: c_int, y0: c_int, x1: c_int, y1: c_int,
		);
		pub virtual fn paint_at_with_breaks(
			x0: c_int, y0: c_int, x1: c_int, y1: c_int,
			break_side: c_int, break_start: c_int, break_stop: c_int,
		);
		pub virtual fn set_inset(left: c_int, top: c_int, right: c_int, bottom: c_int);
		pub virtual fn get_inset(left: RefMut<c_int>, top: RefMut<c_int>, right: RefMut<c_int>, bottom: RefMut<c_int>);
		pub virtual fn apply_scheme_settings(scheme: *mut IScheme, resource_data: *mut KeyValues);
		pub virtual fn get_name() -> *const c_char;
		pub virtual fn set_name(name: *const c_char);

		pub virtual fn get_background_type() -> backgroundtype_e;

		pub virtual fn paint_first() -> bool;
		pub virtual fn paint2(
			x0: c_int, y0: c_int, x1: c_int, y1: c_int,
			break_side: c_int, break_start: c_int, break_stop: c_int,
			n_repeats: c_int,
		);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum backgroundtype_e {
	Filled,
	Textured,
	RoundedCorners,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum sides_e {
	Left,
	Top,
	Right,
	Bottom,
}

::rse_cpp::class! {
	// TODO: `IScheme`.
	IScheme {}
}
