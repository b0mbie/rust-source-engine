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
