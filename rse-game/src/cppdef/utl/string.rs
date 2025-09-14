use ::core::ffi::c_char;

#[derive(Debug)]
#[repr(C)]
pub struct UtlString {
	pub string: *mut c_char,
}
