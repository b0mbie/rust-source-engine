use ::core::{
	ffi::{
		CStr, c_char,
	},
	ptr::NonNull,
};
use ::rse_cpp::virtual_call;
use ::rse_game_interfaces::{
	InterfaceOfFactory, AppSystemFactory,
};
use ::rse_interface::{
	Interface, ptr_interface,
};

use crate::cppdef::{
	IFileSystem, FILESYSTEM_INTERFACE_VERSION,
	FileFindHandle_t,
};

use super::BaseFileSystem;

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

	pub fn is_steam(&self) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_steam()) }
	}

	pub fn is_directory(&self, path: &CStr, path_id: &CStr) -> bool {
		unsafe { virtual_call!(self.as_inner() => is_directory(path.as_ptr(), path_id.as_ptr())) }
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
