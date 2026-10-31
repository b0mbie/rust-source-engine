use ::core::{
	ffi::CStr,
	hint::unreachable_unchecked,
};

/// Buffer that stores the reason of rejecting a player from the server.
#[repr(transparent)]
pub struct RejectReason {
	bytes_until_nul: [u8],
}

impl RejectReason {
	/// # Safety
	/// `buffer` must contain an existing, valid C string.
	pub const unsafe fn new_unchecked(bytes_until_nul: &mut [u8]) -> &mut Self {
		unsafe { &mut *(bytes_until_nul as *mut [u8] as *mut Self) }
	}

	pub const fn as_c_str(&self) -> &CStr {
		// SAFETY: `bytes_until_nul` always contains a NUL to make a C string.
		unsafe {
			match CStr::from_bytes_until_nul(&self.bytes_until_nul) {
				Ok(s) => s,
				Err(..) => unreachable_unchecked(),
			}
		}
	}

	pub fn write(&mut self, text: &[u8]) {
		let text_len = text.len().min(self.bytes_until_nul.len().saturating_sub(1));
		// SAFETY: `text_len` is in range.
		unsafe { self.bytes_until_nul.get_unchecked_mut(..text_len) }.clone_from_slice(text);
		*unsafe { self.bytes_until_nul.get_unchecked_mut(text_len) } = b'\0';
	}
}
