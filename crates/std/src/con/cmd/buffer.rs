use ::core::ffi::{
	CStr, c_char,
};
use ::rse_utl::CString;

use crate::ffi::{
	c_buffer::CBufferWrite,
	CBuffer,
};

use super::Invocation;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CmdBuffer {
	buffer: CBuffer<{Self::LENGTH}>,
}

impl CmdBuffer {
	pub const LENGTH: usize = Invocation::MAX_COMMAND_LENGTH;

	pub const fn new() -> Self {
		Self {
			buffer: CBuffer::new(),
		}
	}

	pub const fn as_c_str(&self) -> &CStr {
		self.buffer.as_c_str()
	}

	pub const fn capacity(&self) -> usize {
		self.buffer.capacity()
	}

	pub const fn as_ptr(&self) -> *const c_char {
		self.buffer.as_ptr()
	}

	pub const fn as_mut_ptr(&mut self) -> *mut c_char {
		self.buffer.as_mut_ptr()
	}

	pub const fn bytes(&self) -> &[u8] {
		self.buffer.bytes()
	}

	pub fn write<F>(&mut self, f: F)
	where
		F: FnOnce(&mut CBufferWrite<'_>),
	{
		self.buffer.write(f)
	}
}

impl From<CmdBuffer> for CString {
	fn from(value: CmdBuffer) -> Self {
		Self::from(value.buffer)
	}
}
