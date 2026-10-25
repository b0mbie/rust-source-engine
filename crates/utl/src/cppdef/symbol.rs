use ::core::ffi::{c_ushort};
use ::rse_cpp::class;

pub type FileNameHandle = *mut ::core::ffi::c_void;
pub const FILENAMEHANDLE_INVALID: FileNameHandle = ::core::ptr::null_mut();

class! {
	pub CStringPoolIndex {
		pub pool: c_ushort;
		pub offset: c_ushort;
	}
}
