use ::core::ffi::CStr;
use ::rse_cpp::c_str::opt_c_str_from_ptr;

use super::CvarFlags;

::rse_cpp::transparent_wrapper! {
	pub struct ConCommandBaseExt for crate::cppdef::ConCommandBaseExt as "ConCommandBaseExt";
}

impl ConCommandBaseExt {
	pub const fn flags(&self) -> CvarFlags {
		CvarFlags::from_bits_retain(self.0.flags)
	}

	pub const fn flags_ref(&self) -> &CvarFlags {
		CvarFlags::from_ref(&self.0.flags)
	}

	pub const fn flags_mut(&mut self) -> &mut CvarFlags {
		CvarFlags::from_mut(&mut self.0.flags)
	}

	pub const fn are_flags_set(&self, flags: CvarFlags) -> bool {
		(self.0.flags & flags.bits()) != 0
	}

	pub const fn add_flags(&mut self, flags: CvarFlags) {
		self.0.flags |= flags.bits()
	}

	pub const fn name(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.name) }
	}

	pub const fn help(&self) -> Option<&CStr> {
		unsafe { opt_c_str_from_ptr(self.0.help_string) }
	}

	pub const fn is_registered(&self) -> bool {
		self.0.registered
	}
}
