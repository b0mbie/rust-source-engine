use ::core::ffi::{
	CStr, c_int, c_uint,
};
use ::rse_cpp::virtual_call;
use ::rse_interface::ptr_interface;

use crate::{
	cppdef::{
		IBaseFileSystem, BASEFILESYSTEM_INTERFACE_VERSION,
		ValidFileHandle,
		FileSystemSeek,
	},
	InterfaceOfFactory, AppSystemFactory,
};

ptr_interface! {
	pub struct OBaseFileSystem(BaseFileSystem);
	Pointee = IBaseFileSystem;
}
unsafe impl Send for OBaseFileSystem {}
unsafe impl Sync for OBaseFileSystem {}
impl ::rse_interface::Interface for OBaseFileSystem {
	const IDENTIFIER: &CStr = BASEFILESYSTEM_INTERFACE_VERSION;
}
impl InterfaceOfFactory for OBaseFileSystem {
	type Factory = AppSystemFactory;
}

/// Safe interface to `IBaseFileSystem`.
impl BaseFileSystem {
	pub fn open(&self, path: &CStr, options: &CStr, path_id: &CStr) -> Option<ValidFileHandle> {
		unsafe {
			let raw = virtual_call!(self.as_inner() => open(path.as_ptr(), options.as_ptr(), path_id.as_ptr()));
			ValidFileHandle::new(raw)
		}
	}

	/// # Safety
	/// `raw` must be an open file handle
	/// that came from this interface.
	pub unsafe fn close(&self, raw: ValidFileHandle) {
		unsafe { virtual_call!(self.as_inner() => close(raw.as_ptr())) }
	}

	/// # Safety
	/// `raw` must be an open file handle
	/// that came from this interface.
	pub unsafe fn seek(&self, raw: ValidFileHandle, pos: c_int, method: FileSystemSeek) {
		unsafe { virtual_call!(self.as_inner() => seek(raw.as_ptr(), pos, method)) }
	}

	/// # Safety
	/// `raw` must be an open file handle
	/// that came from this interface.
	pub unsafe fn tell(&self, raw: ValidFileHandle) -> c_uint {
		unsafe { virtual_call!(self.as_inner() => tell(raw.as_ptr())) }
	}

	/// # Safety
	/// `raw` must be an open file handle
	/// that came from this interface.
	pub unsafe fn size(&self, raw: ValidFileHandle) -> c_uint {
		unsafe { virtual_call!(self.as_inner() => size(raw.as_ptr())) }
	}

	pub fn size_at(&self, path: &CStr, path_id: &CStr) -> c_uint {
		unsafe { virtual_call!(self.as_inner() => size_at(path.as_ptr(), path_id.as_ptr())) }
	}

	/// # Safety
	/// `raw` must be an open file handle
	/// that came from this interface.
	pub unsafe fn flush(&self, raw: ValidFileHandle) {
		unsafe { virtual_call!(self.as_inner() => flush(raw.as_ptr())) }
	}

	/// # Safety
	/// `raw` must be an open file handle
	/// that came from this interface.
	pub unsafe fn read(&self, raw: ValidFileHandle, buffer: &mut [u8]) -> c_int {
		unsafe { virtual_call!(
			self.as_inner() => read(buffer.as_mut_ptr().cast(), slice_len_c_int(buffer.len()), raw.as_ptr())
		) }
	}

	/// # Safety
	/// `raw` must be an open file handle
	/// that came from this interface.
	pub unsafe fn write(&self, raw: ValidFileHandle, data: &[u8]) -> c_int {
		unsafe { virtual_call!(
			self.as_inner() => write(data.as_ptr().cast(), slice_len_c_int(data.len()), raw.as_ptr())
		) }
	}

	// TODO: Read/write methods.
	// TODO: File metadata methods.
}

const fn slice_len_c_int(len: usize) -> c_int {
	if len > c_int::MAX as usize {
		c_int::MAX as _
	} else {
		len as _
	}
}