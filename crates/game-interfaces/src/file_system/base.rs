use ::core::{
	ffi::{
		CStr, c_char, c_int, c_uint,
	},
	ptr::NonNull,
};
use ::rse_cpp::virtual_call;
use ::rse_interface::{
	Interface, ptr_interface,
};

use crate::{
	cppdef::{
		IBaseFileSystem, BASEFILESYSTEM_INTERFACE_VERSION,
		IFileSystem, FILESYSTEM_INTERFACE_VERSION,
		ValidFileHandle,
		FileSystemSeek,
		FileFindHandle_t,
	},
	InterfaceOfFactory, AppSystemFactory,
};

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

ptr_interface! {
	pub struct OFileSystem(FileSystem);
	Pointee = IFileSystem;
}
unsafe impl Send for OFileSystem {}
unsafe impl Sync for OFileSystem {}
impl Interface for OFileSystem {
	const IDENTIFIER: &CStr = FILESYSTEM_INTERFACE_VERSION;
}
impl InterfaceOfFactory for OFileSystem {
	type Factory = AppSystemFactory;
}

impl FileSystem {
	pub const fn as_base(&self) -> &BaseFileSystem {
		unsafe { BaseFileSystem::from_ref(&self.as_inner().data.inherited_i_base_file_system) }
	}

	pub unsafe fn find_first_ex(&self, wildcard: &CStr, path_id: &CStr) -> Option<(FileFindHandle_t, NonNull<c_char>)> {
		let mut handle = 0;
		unsafe {
			let filename = virtual_call!(
				self.as_inner() => find_first_ex(wildcard.as_ptr(), path_id.as_ptr(), &mut handle)
			);
			NonNull::new(filename.cast_mut()).map(move |p| (handle, p))
		}
	}

	pub unsafe fn find_first(&self, wildcard: &CStr) -> Option<(FileFindHandle_t, NonNull<c_char>)> {
		let mut handle = 0;
		unsafe {
			let filename = virtual_call!(
				self.as_inner() => find_first(wildcard.as_ptr(), &mut handle)
			);
			NonNull::new(filename.cast_mut()).map(move |p| (handle, p))
		}
	}

	pub unsafe fn find_next(&self, handle: FileFindHandle_t) -> Option<NonNull<c_char>> {
		NonNull::new(unsafe { virtual_call!(self.as_inner() => find_next(handle)) }.cast_mut())
	}

	pub unsafe fn find_is_directory(&self, handle: FileFindHandle_t) -> bool {
		unsafe { virtual_call!(self.as_inner() => find_is_directory(handle)) }
	}

	pub unsafe fn find_close(&self, handle: FileFindHandle_t) {
		unsafe { virtual_call!(self.as_inner() => find_close(handle)) }
	}
}

const fn slice_len_c_int(len: usize) -> c_int {
	if len > c_int::MAX as usize {
		c_int::MAX as _
	} else {
		len as _
	}
}