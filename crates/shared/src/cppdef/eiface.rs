use ::core::ffi::c_char;
use ::rse_math::Vector;
use ::rse_tier0::MAX_PATH;

#[repr(C)]
pub struct WorkshopMapDesc_t {
	pub map_name: [c_char; MAX_PATH],
	pub original_map_name: [c_char; MAX_PATH],
	pub timestamp: u32,
	pub downloaded: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct bbox_t {
	pub mins: Vector,
	pub maxs: Vector,
}
