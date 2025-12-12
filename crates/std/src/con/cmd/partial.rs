use ::core::{
	ffi::CStr,
	slice::from_raw_parts,
	str::Utf8Error,
};

use super::Pieces;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Partial<'a> {
	partial: &'a CStr,
}

impl<'a> Partial<'a> {
	pub(crate) const fn new(partial: &'a CStr) -> Self {
		Self {
			partial,
		}
	}

	/// Returns the whole partial string.
	pub const fn as_c_str(&self) -> &'a CStr {
		self.partial
	}

	/// Returns the whole partial string
	/// as a byte slice.
	pub const fn as_bytes(&self) -> &'a [u8] {
		self.partial.to_bytes()
	}

	/// Returns the whole partial string
	/// as a [`str`]
	/// if it contains valid UTF-8.
	pub const fn to_str(&self) -> Result<&'a str, Utf8Error> {
		self.partial.to_str()
	}

	/// Returns an iterator over all the arguments,
	/// with the first argument, if any, being the command name.
	pub fn pieces(&self) -> PartialPieces<'a> {
		PartialPieces::new(self.partial)
	}

	/// Returns an iterator over all the arguments.
	pub fn args(&self) -> Args<'a> {
		Args::new(self.pieces())
	}

	/// Returns the string that only contains the command arguments.
	pub fn arg_string(&self) -> &'a CStr {
		self.args().rest()
	}

	pub fn split_last(&self) -> (usize, &'a [u8], &'a [u8]) {
		let split = self.args().split_last();
		(split.index, split.value, split.before)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SplitLast<'a> {
	pub index: usize,
	pub value: &'a [u8],
	pub before: &'a [u8],
}

pub struct Args<'a> {
	pieces: PartialPieces<'a>,
	is_consequent: bool,
}

impl<'a> Args<'a> {
	const fn new(pieces: PartialPieces<'a>) -> Self {
		Self {
			pieces,
			is_consequent: false,
		}
	}

	pub const fn before(&self) -> &'a [u8] {
		self.pieces.before()
	}

	/// Returns the rest of the unparsed argument string.
	pub const fn rest(&self) -> &'a CStr {
		self.pieces.rest()
	}

	/// Returns the index of the last argument,
	/// the argument string,
	/// and the string that came before it,
	/// or `None` if there are no arguments.
	pub fn split_last(self) -> SplitLast<'a> {
		let mut pieces = self.pieces;
		if !self.is_consequent {
			pieces.next();
		}
		pieces.split_last()
	}
}

impl<'a> Iterator for Args<'a> {
	type Item = &'a [u8];
	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if !self.is_consequent {
				self.pieces.next();
				self.is_consequent = true;
			} else {
				break self.pieces.next()
			}
		}
	}
}

pub struct PartialPieces<'a> {
	pieces: Pieces<'a>,
	input: *const u8,
	end: *const u8,
}

impl<'a> PartialPieces<'a> {
	fn new(input: &'a CStr) -> Self {
		Self {
			pieces: Pieces::new(input),
			input: input.as_ptr() as _,
			end: input.to_bytes().as_ptr_range().end,
		}
	}

	pub const fn input(&self) -> &'a CStr {
		unsafe { CStr::from_ptr(self.input as _) }
	}

	pub const fn before(&self) -> &'a [u8] {
		let arg_start = self.pieces.rest().as_ptr();
		let len = unsafe { arg_start.offset_from_unsigned(self.input as _) };
		unsafe { from_raw_parts(self.input, len) }
	}

	pub const fn rest(&self) -> &'a CStr {
		self.pieces.rest()
	}

	/// Returns the index of the last argument,
	/// the argument string,
	/// and the string that came before it.
	pub fn split_last(mut self) -> SplitLast<'a> {
		let Some((index, value)) = self.by_ref().enumerate().last() else {
			return SplitLast {
				index: 0,
				before: self.input().to_bytes(),
				value: b"",
			}
		};
		let n_after = unsafe { self.end.offset_from_unsigned(value.as_ptr_range().end) };
		if n_after == 0 {
			let n_before = unsafe { value.as_ptr().offset_from_unsigned(self.input) };
			let before = unsafe { from_raw_parts(self.input, n_before) };
			SplitLast {
				index,
				before,
				value,
			}
		} else {
			SplitLast {
				index: index + 1,
				before: self.input().to_bytes(),
				value: b"",
			}
		}
	}
}

impl<'a> Iterator for PartialPieces<'a> {
	type Item = &'a [u8];
	fn next(&mut self) -> Option<Self::Item> {
		self.pieces.next()
	}
}


#[test]
fn split_last() {
	assert_eq!(
		PartialPieces::new(c"the").split_last(),
		SplitLast {
			index: 0,
			before: b"",
			value: b"the",
		},
	);
	assert_eq!(
		PartialPieces::new(c"the ").split_last(),
		SplitLast {
			index: 1,
			before: b"the ",
			value: b"",
		},
	);
	assert_eq!(
		PartialPieces::new(c" the very coole:").split_last(),
		SplitLast {
			index: 3,
			before: b" the very coole",
			value: b":",
		},
	);
	assert_eq!(
		PartialPieces::new(c"\tthe very \"cool:").split_last(),
		SplitLast {
			index: 2,
			before: b"\tthe very \"",
			value: b"cool:",
		},
	);
	assert_eq!(
		PartialPieces::new(c"").split_last(),
		SplitLast {
			index: 0,
			before: b"",
			value: b"",
		},
	);

	assert_eq!(
		Args::new(PartialPieces::new(c"echo the very \"cool\"")).split_last(),
		SplitLast {
			index: 3,
			before: b"echo the very \"cool\"",
			value: b"",
		},
	);
}
