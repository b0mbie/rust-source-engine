use ::core::ffi::{
	CStr, c_int, c_uint,
};
use ::rse_cpp::{
	AsObject, owned_vt_object_wrapper,
	virtual_call,
};

use crate::{
	cppdef::{
		BaseFileSystemVt, BASEFILESYSTEM_INTERFACE_VERSION,
		ValidFileHandle,
		FileSystemSeek,
	},
	InterfaceOfFactory, AppSystemFactory,
};

/// Safe interface to `IBaseFileSystem`.
pub trait BaseFileSystemImpl: AsObject<BaseFileSystemVt> {
	fn open(&self, path: &CStr, options: &CStr, path_id: &CStr) -> Option<ValidFileHandle> {
		unsafe {
			let raw = virtual_call!(self.as_object() => open(path.as_ptr(), options.as_ptr(), path_id.as_ptr()));
			ValidFileHandle::new(raw)
		}
	}

	unsafe fn close(&self, raw: ValidFileHandle) {
		unsafe { virtual_call!(self.as_object() => close(raw.as_ptr())) }
	}

	unsafe fn seek(&self, raw: ValidFileHandle, pos: c_int, method: FileSystemSeek) {
		unsafe { virtual_call!(self.as_object() => seek(raw.as_ptr(), pos, method)) }
	}

	unsafe fn tell(&self, raw: ValidFileHandle) -> c_uint {
		unsafe { virtual_call!(self.as_object() => tell(raw.as_ptr())) }
	}

	unsafe fn size(&self, raw: ValidFileHandle) -> c_uint {
		unsafe { virtual_call!(self.as_object() => size(raw.as_ptr())) }
	}

	fn size_at(&self, path: &CStr, path_id: &CStr) -> c_uint {
		unsafe { virtual_call!(self.as_object() => size_at(path.as_ptr(), path_id.as_ptr())) }
	}

	unsafe fn flush(&self, raw: ValidFileHandle) {
		unsafe { virtual_call!(self.as_object() => flush(raw.as_ptr())) }
	}

	unsafe fn read(&self, raw: ValidFileHandle, buffer: &mut [u8]) -> c_int {
		unsafe { virtual_call!(
			self.as_object() => read(buffer.as_mut_ptr().cast(), slice_len_c_int(buffer.len()), raw.as_ptr())
		) }
	}

	unsafe fn write(&self, raw: ValidFileHandle, data: &[u8]) -> c_int {
		unsafe { virtual_call!(
			self.as_object() => write(data.as_ptr().cast(), slice_len_c_int(data.len()), raw.as_ptr())
		) }
	}

	// TODO: Read/write methods.
	// TODO: File metadata methods.
}
impl<T: ?Sized + AsObject<BaseFileSystemVt>> BaseFileSystemImpl for T {}

const fn slice_len_c_int(len: usize) -> c_int {
	if len > c_int::MAX as usize {
		c_int::MAX as _
	} else {
		len as _
	}
}

owned_vt_object_wrapper! {
	pub struct BaseFileSystem for BaseFileSystemVt;
}
unsafe impl ::rse_interface::Interface for BaseFileSystem {
	const IDENTIFIER: &CStr = BASEFILESYSTEM_INTERFACE_VERSION;
}
impl InterfaceOfFactory for BaseFileSystem {
	type Factory = AppSystemFactory;
}
