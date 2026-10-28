use ::core::ffi::{
	c_short, c_ushort,
};

#[repr(C)]
pub struct ScreenFade_t {
	pub duration: c_ushort,
	pub hold_time: c_ushort,
	pub fade_flags: c_short,
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}
