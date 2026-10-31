use ::core::{
	ffi::{
		CStr, c_char,
	},
	ptr::NonNull,
};
use ::rse_shared::cppdef::FileFindHandle_t;

use super::with_fs;

pub struct Find {
	raw: FileFindHandle_t,
	current_path: Option<NonNull<c_char>>,
}

impl Drop for Find {
	fn drop(&mut self) {
		with_fs(move |fs| unsafe { fs.find_close(self.raw) })
	}
}

impl Find {
	pub fn open(wildcard: &CStr) -> Option<Self> {
		with_fs(move |fs| unsafe { fs.find_first(wildcard) })
			.map(move |(raw, current_path)| {
				Self {
					raw,
					current_path: Some(current_path),
				}
			})
	}

	pub fn open_in(wildcard: &CStr, path_id: &CStr) -> Option<Self> {
		with_fs(move |fs| unsafe { fs.find_first_ex(wildcard, path_id) })
			.map(move |(raw, current_path)| {
				Self {
					raw,
					current_path: Some(current_path),
				}
			})
	}

	pub fn get(&self) -> Option<(&CStr, bool)> {
		if let Some(p) = self.current_path {
			let path = unsafe { CStr::from_ptr(p.as_ptr()) };
			Some((path, self.is_directory()))
		} else {
			None
		}
	}

	pub fn path(&self) -> Option<&CStr> {
		if let Some(p) = self.current_path {
			unsafe { Some(CStr::from_ptr(p.as_ptr())) }
		} else {
			None
		}
	}

	pub fn is_directory(&self) -> bool {
		with_fs(move |fs| unsafe { fs.find_is_directory(self.raw) })
	}

	pub fn advance(&mut self) {
		let raw = self.raw;
		self.current_path = with_fs(move |fs| unsafe { fs.find_next(raw) })
	}
}
