use ::core::ffi::{
	CStr, c_char, c_int,
};

use crate::{
	IntoFormattable, Formattable,
};

impl IntoFormattable for char {
	type Formattable = Utf8EncodedChar;
	fn into_formattable(self) -> Self::Formattable {
		let mut buffer = [0; 4];
		let length = self.encode_utf8(&mut buffer).len();
		Utf8EncodedChar {
			buffer,
			length: length as _,
		}
	}
}

#[derive(Clone, Copy)]
pub struct Utf8EncodedChar {
	buffer: [u8; 4],
	length: u8,
}

unsafe impl Formattable for Utf8EncodedChar {
	type FormatString = &'static CStr;
	fn format_string(&self) -> Self::FormatString {
		c"%.*s"
	}

	type Precision = c_int;
	fn precision(&self) -> Self::Precision {
		self.length as _
	}

	type CType = *const c_char;
	fn into_c_type(self) -> Self::CType {
		self.buffer.as_ptr() as _
	}
}
