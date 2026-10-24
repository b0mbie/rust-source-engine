use ::core::ffi::c_long;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FileTime {
	raw: c_long,
}

impl FileTime {
	pub const fn from_raw(raw: c_long) -> Self {
		Self {
			raw,
		}
	}

	pub const fn to_raw(self) -> c_long {
		self.raw
	}
}
