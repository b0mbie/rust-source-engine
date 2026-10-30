use ::core::ffi::{c_ushort};
use ::rse_cpp::class;

#[allow(non_camel_case_types)]
pub type FileNameHandle_t = *mut ::core::ffi::c_void;
pub const FILENAMEHANDLE_INVALID: FileNameHandle_t = ::core::ptr::null_mut();

class! {
	pub CStringPoolIndex {
		pub pool: c_ushort;
		pub offset: c_ushort;
	}
}
