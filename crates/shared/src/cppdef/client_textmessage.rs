use ::core::ffi::{
	c_char, c_float, c_int,
};

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct client_textmessage_t {
	pub effect: c_int,

	pub r1: u8,
	pub g1: u8,
	pub b1: u8,
	pub a1: u8,

	pub r2: u8,
	pub g2: u8,
	pub b2: u8,
	pub a2: u8,

	pub x: c_float,
	pub y: c_float,
	pub fade_in: c_float,
	pub fade_out: c_float,
	pub hold_time: c_float,
	pub fx_time: c_float,

	pub vgui_scheme_font_name: *const c_char,
	pub name: *const c_char,
	pub message: *const c_char,

	pub rounded_rect_backdrop_box: bool,
	pub box_size: c_float,
	pub box_color: [u8; 4],
	pub clear_message: *const c_char,
}
