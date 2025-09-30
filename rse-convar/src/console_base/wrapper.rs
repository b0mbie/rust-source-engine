use ::core::ffi::CStr;

use crate::cppdef::CvarFlags;

::rse_cpp::transparent_wrapper! {
	pub struct ConCommandBaseExt for crate::cppdef::ConCommandBaseExt as "ConCommandBase";
}

impl ConCommandBaseExt {
	pub const fn flags(&self) -> CvarFlags {
		self.0.flags
	}

	pub const fn is_flag_set(&self, flag: CvarFlags) -> bool {
		(self.0.flags & flag) != 0
	}

	pub const fn add_flags(&mut self, flags: CvarFlags) {
		self.0.flags |= flags
	}

	pub const fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.name) }
	}

	pub const fn help(&self) -> Option<&CStr> {
		let ptr = self.0.help_string;
		if !ptr.is_null() {
			unsafe { Some(CStr::from_ptr(ptr)) }
		} else {
			None
		}
	}

	pub const fn is_registered(&self) -> bool {
		self.0.registered
	}
}
