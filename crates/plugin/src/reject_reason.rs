use ::core::ffi::CStr;

/// Buffer that stores the reason of rejecting a player from the server.
#[repr(transparent)]
pub struct RejectReason<'a> {
	buffer: &'a mut [u8],
}

impl<'a> RejectReason<'a> {
	/// # Safety
	/// `buffer` must contain an existing, valid C string.
	pub const unsafe fn new_unchecked(buffer: &'a mut [u8]) -> Self {
		Self {
			buffer,
		}
	}

	pub fn as_c_str(&self) -> &CStr {
		// SAFETY: `buffer` always contains a NUL to make a C string.
		unsafe { CStr::from_bytes_until_nul(self.buffer).unwrap_unchecked() }
	}

	pub fn write(&mut self, text: &[u8]) {
		let text_len = text.len().min(self.buffer.len().saturating_sub(1));
		// SAFETY: `text_len` is in range.
		unsafe { self.buffer.get_unchecked_mut(..text_len) }.clone_from_slice(text);
		*unsafe { self.buffer.get_unchecked_mut(text_len) } = b'\0';
	}
}
