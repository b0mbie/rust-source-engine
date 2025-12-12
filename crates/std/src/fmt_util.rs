use ::core::{
	fmt::{
		self, Write,
	},
	mem::take,
};

pub fn formatted_len(args: fmt::Arguments<'_>) -> usize {
	#[repr(transparent)]
	struct FmtLen(pub usize);
	impl Write for FmtLen {
		fn write_str(&mut self, s: &str) -> fmt::Result {
			self.0 += s.len();
			Ok(())
		}
	}
	let mut len = FmtLen(0);
	let _ = len.write_fmt(args);
	len.0
}

#[repr(transparent)]
pub struct SliceWrite<'a>(pub &'a mut [u8]);
impl<'a> SliceWrite<'a> {
	pub fn write(&mut self, data: &[u8]) -> usize {
		let amt = data.len().min(self.0.len());
        let (dest, rest) = take(&mut self.0).split_at_mut(amt);
		dest.copy_from_slice(&data[..amt]);
		self.0 = rest;
		amt
	}
}

impl Write for SliceWrite<'_> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write(s.as_bytes());
		Ok(())
	}
}

#[repr(transparent)]
pub struct CStrWrite<'a> {
	bytes: &'a mut [u8],
}
impl<'a> CStrWrite<'a> {
	pub const fn new(bytes: &'a mut [u8]) -> Option<Self> {
		if !bytes.is_empty() {
			unsafe { Some(Self::new_unchecked(bytes)) }
		} else {
			None
		}
	}

	pub const unsafe fn new_unchecked(bytes: &'a mut [u8]) -> Self {
		Self {
			bytes,
		}
	}

	const fn len_without_nul(&self) -> usize {
		unsafe { self.bytes.len().unchecked_sub(1) }
	}

	pub fn write(&mut self, data: &[u8]) -> usize {
		let amt = data.len().min(self.len_without_nul());
		let (dest, rest) = take(&mut self.bytes).split_at_mut(amt);
		dest.copy_from_slice(&data[..amt]);
		self.bytes = rest;
		amt
	}

	pub const fn finish(self) {
		self.bytes[0] = 0
	}
}

impl Write for CStrWrite<'_> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write(s.as_bytes());
		Ok(())
	}
}
