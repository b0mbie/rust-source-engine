use ::core::ffi::{
	c_char, c_short, c_ushort, c_void,
};

use super::VectorAligned;

// `struct model_t` is never implemented, only used behind a pointer.
#[repr(transparent)]
pub struct Model(c_void);

// TODO: This is called `csurface_t` in code. Figure out why, and maybe rename.
#[derive(Debug)]
#[repr(C)]
pub struct Surface {
	pub name: *const c_char,
	pub surface_props: c_short,
	pub flags: c_ushort,
}

#[derive(Debug)]
#[repr(C)]
pub struct Ray {
	pub start: VectorAligned,
	pub delta: VectorAligned,
	pub start_offset: VectorAligned,
	pub extents: VectorAligned,
	pub is_ray: bool,
	pub is_swept: bool,
}
