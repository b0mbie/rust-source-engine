use ::core::ffi::{
	c_int, c_float,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct con_nprint_t {
	/// Row number.
	pub index: c_int,
	/// Number of seconds before this notification disappears.
	/// 
	/// A value of `-1` specifies that the notification should be displayed for one frame,
	/// after which it disappears.
	pub time_to_live: c_float,
	/// Normalized RGB color of the text.
	pub color: [c_float; 3],
	/// `true` if the notification text should use a fixed-width font.
	pub fixed_width_font: bool,
}
