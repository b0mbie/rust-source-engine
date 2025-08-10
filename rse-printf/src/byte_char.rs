use ::core::ffi::{
	CStr, c_int,
};

use crate::Formattable;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ByteChar(pub u8);
impl From<u8> for ByteChar {
	fn from(value: u8) -> Self {
		Self(value)
	}
}
impl From<ByteChar> for u8 {
	fn from(value: ByteChar) -> Self {
		value.0
	}
}

unsafe impl Formattable for ByteChar {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%c"
	}

	type Precision = ();
	fn precision(&self) -> Self::Precision {}

	type CType = c_int;
	fn into_c_type(self) -> Self::CType {
		self.0 as _
	}
}
