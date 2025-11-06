use ::core::ffi::{
	c_ulong, c_void,
};

use super::{
	Vector, VPlane,
};

pub const MAX_SURFINFO_VERTS: usize = 16;

#[repr(C)]
pub struct SurfInfo {
	pub verts: [Vector; MAX_SURFINFO_VERTS],
	pub n_verts: c_ulong,
	pub plane: VPlane,
	pub engine_data: *mut c_void,
}
