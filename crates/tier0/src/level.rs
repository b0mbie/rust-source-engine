use ::core::ffi::c_int;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Level(pub c_int);
impl Level {
	pub const NONE: Self = Self(0);
	pub const DEFAULT: Self = Self(1);
	pub const DEVELOPER: Self = Self(2);
}
impl Default for Level {
	fn default() -> Self {
		Self::NONE
	}
}
impl From<c_int> for Level {
	fn from(value: c_int) -> Self {
		Self(value)
	}
}
impl From<Level> for c_int {
	fn from(value: Level) -> Self {
		value.0
	}
}
