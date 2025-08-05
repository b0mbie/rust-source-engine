use ::core::ffi::*;

pub unsafe trait CFormattable {
	const FORMAT_STR: &'static CStr;
	type CType;
	fn into_c_type(self) -> Self::CType;
}
unsafe impl CFormattable for &CStr {
	const FORMAT_STR: &'static CStr = c"%s";
	type CType = *const c_char;
	fn into_c_type(self) -> Self::CType {
		self.as_ptr()
	}
}

unsafe impl CFormattable for f32 {
	const FORMAT_STR: &'static CStr = c"%g";
	type CType = c_double;
	fn into_c_type(self) -> Self::CType {
		self as _
	}
}
unsafe impl CFormattable for f64 {
	const FORMAT_STR: &'static CStr = c"%g";
	type CType = c_double;
	fn into_c_type(self) -> Self::CType {
		self as _
	}
}

unsafe impl CFormattable for c_schar {
	const FORMAT_STR: &'static CStr = c"%hhd";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl CFormattable for c_uchar {
	const FORMAT_STR: &'static CStr = c"%hhu";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}

unsafe impl CFormattable for c_short {
	const FORMAT_STR: &'static CStr = c"%hd";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl CFormattable for c_ushort {
	const FORMAT_STR: &'static CStr = c"%hu";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl CFormattable for c_int {
	const FORMAT_STR: &'static CStr = c"%d";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl CFormattable for c_uint {
	const FORMAT_STR: &'static CStr = c"%u";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl CFormattable for c_longlong {
	const FORMAT_STR: &'static CStr = c"%lld";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl CFormattable for c_ulonglong {
	const FORMAT_STR: &'static CStr = c"%llu";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl CFormattable for isize {
	const FORMAT_STR: &'static CStr = c"%td";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
unsafe impl CFormattable for usize {
	const FORMAT_STR: &'static CStr = c"%zu";
	type CType = Self;
	fn into_c_type(self) -> Self::CType {
		self
	}
}
