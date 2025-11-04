use ::core::ffi::{
	CStr, c_char, c_int,
};
use ::rse_cpp::{
	AsObject, owned_vt_object_wrapper,
	virtual_call,
};

use crate::{
	cppdef::{
		FileSystemVt, FILESYSTEM_INTERFACE_VERSION,
	},
	InterfaceOfFactory, AppSystemFactory,
};

pub use crate::cppdef::{
	FileSystemMount, PathTypeFilter, PathTypeQuery,
};

/// Safe interface to `IFileSystem`.
pub trait FileSystemImpl: AsObject<FileSystemVt> {
	/// Returns `true` if the filesystem supports Steam operations.
	fn is_steam(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => file_system.is_steam()) }
	}

	/// Mounts an app specified by its ID in addition to
	/// the one specified in the environment variable `steamappid`.
	/// 
	/// If `extra_app_id < -1`, then it will mount that app ID *only*.
	/// 
	/// This function was needed by the dedicated server because
	/// the `SteamAppId` environment variable only gets passed to `steam.dll`
	/// at load time, so the dedicated server couldn't pass it in that way.
	fn mount_steam_content(&self, extra_app_id: c_int) -> FileSystemMount {
		unsafe { virtual_call!(self.as_object() => file_system.mount_steam_content(extra_app_id)) }
	}

	/// Fills `buffer` with a C string that represents the current working directory,
	/// returning `true` if the operation succeeded.
	fn current_directory(&self, buffer: &mut [u8]) -> bool {
		unsafe { virtual_call!(
			self.as_object() => file_system.get_current_directory(buffer.as_mut_ptr() as _, buffer.len() as _)
		) }
	}

	/// Returns `true` if `path` points to a directory.
	fn is_directory(&self, path_id: &CStr, path: &CStr) -> bool {
		unsafe { virtual_call!(self.as_object() => file_system.is_directory(path.as_ptr(), path_id.as_ptr())) }
	}

	/// Fills `buffer` with a C string that represents
	/// the full path that is equivalent to the given `relative_path`,
	/// returning `buffer` as a C string if the operation succeeded,
	/// or `None` if the path couldn't be resolved.
	fn relative_path_to_full_path<'buf>(
		&self,
		path_id: &CStr, relative_path: &CStr,
		buffer: &'buf mut [u8],
		path_filter: PathTypeFilter,
		path_type_query: Option<&mut PathTypeQuery>,
	) -> Option<&'buf CStr> {
		const unsafe fn c_str_opt<'a>(ptr: *const c_char) -> Option<&'a CStr> {
			if !ptr.is_null() {
				unsafe { Some(CStr::from_ptr(ptr)) }
			} else {
				None
			}
		}
		unsafe {
			let ptr = virtual_call!(
				self.as_object() => file_system.relative_path_to_full_path(
					relative_path.as_ptr(), path_id.as_ptr(),
					buffer.as_mut_ptr() as _, buffer.len() as _,
					path_filter, path_type_query.map(move |p| p as *mut _).unwrap_or_default(),
				)
			);
			c_str_opt(ptr)
		}
	}

	/// Fills `buffer` with a C string that represents
	/// the relative path that is equivalent to the given `full_path`,
	/// returning `true` if the operation succeeded,
	/// or `false` if the path couldn't be resolved.
	fn full_path_to_relative_path(&self, full_path: &CStr, buffer: &mut [u8]) -> bool {
		unsafe { virtual_call!(
			self.as_object() => file_system.full_path_to_relative_path(
				full_path.as_ptr(),
				buffer.as_mut_ptr() as _, buffer.len() as _,
			)
		) }
	}
}
impl<T: ?Sized + AsObject<FileSystemVt>> FileSystemImpl for T {}

owned_vt_object_wrapper! {
	pub struct FileSystem for FileSystemVt;
}
unsafe impl ::rse_interface::Interface for FileSystem {
	const IDENTIFIER: &CStr = FILESYSTEM_INTERFACE_VERSION;
}
impl InterfaceOfFactory for FileSystem {
	type Factory = AppSystemFactory;
}
