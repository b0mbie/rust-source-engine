use ::core::ffi::{
	CStr, c_int, c_uint,
};
use ::rse_cpp::{
	util::clamp_len_to_c_int,
	virtual_call,
};
use ::rse_interface::{
	Interface, ptr_interface,
};

use crate::{
	cppdef::{
		IBaseFileSystem, BASEFILESYSTEM_INTERFACE_VERSION,
		ValidFileHandle,
		FileSystemSeek,
	},
	InterfaceOfFactory, AppSystemFactory,
};

use super::FileTime;

ptr_interface! {
	pub struct OBaseFileSystem(BaseFileSystem);
	Pointee = IBaseFileSystem;
}
unsafe impl Send for OBaseFileSystem {}
unsafe impl Sync for OBaseFileSystem {}
impl Interface for OBaseFileSystem {
	const IDENTIFIER: &CStr = BASEFILESYSTEM_INTERFACE_VERSION;
}
impl InterfaceOfFactory for OBaseFileSystem {
	type Factory = AppSystemFactory;
}

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
			self.as_inner() => read(buffer.as_mut_ptr().cast(), clamp_len_to_c_int(buffer.len()), raw.as_ptr())
		) }
	}

	/// # Safety
	/// `raw` must be an open file handle
	/// that came from this interface.
	pub unsafe fn write(&self, raw: ValidFileHandle, data: &[u8]) -> c_int {
		unsafe { virtual_call!(
			self.as_inner() => write(data.as_ptr().cast(), clamp_len_to_c_int(data.len()), raw.as_ptr())
		) }
	}

	pub fn precache(&mut self, filename: &CStr, path_id: &CStr) -> bool {
		unsafe { virtual_call!(self.as_mut_inner() => precache(filename.as_ptr(), path_id.as_ptr())) }
	}

	pub fn file_exists(&self, filename: &CStr, path_id: &CStr) -> bool {
		unsafe { virtual_call!(self.as_inner() => file_exists(filename.as_ptr(), path_id.as_ptr())) }
	}

	pub fn is_file_writable(&self, filename: &CStr, path_id: &CStr) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_file_writable(filename.as_ptr(), path_id.as_ptr())) }
	}

	pub fn set_file_writable(&mut self, filename: &CStr, path_id: &CStr, writable: bool) -> bool {
		unsafe { virtual_call!(self.as_inner() => set_file_writable(filename.as_ptr(), writable, path_id.as_ptr())) }
	}

	pub fn get_file_time(&self, filename: &CStr, path_id: &CStr) -> FileTime {
		let raw = unsafe { virtual_call!(self.as_inner() => get_file_time(filename.as_ptr(), path_id.as_ptr())) };
		FileTime::from_raw(raw)
	}

	// TODO: File metadata methods.
}
