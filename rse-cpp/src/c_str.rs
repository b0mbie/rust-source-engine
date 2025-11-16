use ::core::{
	ffi::{
		CStr, c_char,
	},
	ptr::null,
};

/// Returns a [`CStr`] from a C pointer if it is non-null.
/// 
/// # Safety
/// `ptr`, if non-null, must point to a valid C string.
/// All caveats of [`CStr::from_ptr`] also apply here.
pub const unsafe fn opt_c_str_from_ptr<'a>(ptr: *const c_char) -> Option<&'a CStr> {
	if !ptr.is_null() {
		unsafe { Some(CStr::from_ptr(ptr)) }
	} else {
		None
	}
}

/// Returns a C string pointer from an optional [`CStr`] if it is `Some`,
/// returning null if it is `None`.
pub const fn opt_c_str_as_ptr(s: Option<&CStr>) -> *const c_char {
	match s {
		Some(s) => s.as_ptr(),
		None => null(),
	}
}
