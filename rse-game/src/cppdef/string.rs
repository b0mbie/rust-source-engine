use ::core::ffi::{
	CStr, c_char,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct string_t {
	value: *const c_char,
}

impl string_t {
	pub const fn new(s: &'static CStr) -> Self {
		Self {
			value: s.as_ptr(),
		}
	}

	/// # Safety
	/// `ptr` *must* either be null, or point to a valid C string.
	pub const unsafe fn from_ptr(ptr: *const c_char) -> Self {
		Self {
			value: ptr,
		}
	}

	pub const fn as_ptr(&self) -> *const c_char {
		self.value
	}

	pub const fn to_c_str(&self) -> &CStr {
		if self.value.is_null() {
			c""
		} else {
			unsafe { CStr::from_ptr(self.value) }
		}
	}
}
