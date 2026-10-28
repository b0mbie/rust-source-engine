use ::core::ffi::c_int;

#[allow(non_camel_case_types)]
mod dependent {
	#[cfg(target_pointer_width = "64")]
	mod bitness_dependent {
		pub type uintp = u64;
		pub type intp = i64;
	}

	#[cfg(not(target_pointer_width = "64"))]
	mod bitness_dependent {
		pub type uintp = u32;
		pub type intp = i32;
	}

	pub use bitness_dependent::*;
}

/// Integer that can accomodate a pointer.
pub use dependent::intp;

/// Unsigned integer that can accomodate a pointer.
pub use dependent::uintp;

pub const MAX_PATH: usize = 260;

#[allow(non_camel_case_types)]
pub type qboolean = c_int;

#[repr(C)]
pub struct vrect_t {
	pub x: c_int,
	pub y: c_int,
	pub width: c_int,
	pub height: c_int,
	pub next: *mut Self,
}
