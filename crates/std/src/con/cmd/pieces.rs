use ::core::ffi::CStr;
use ::sourconpars::TokensCStr;

/// Iterator over command argument tokens.
#[repr(transparent)]
pub struct Pieces<'a> {
	tokens: TokensCStr<'a>,
}

impl<'a> Pieces<'a> {
	/// Returns a new iterator that parses the given input string.
	pub fn new(input: &'a CStr) -> Self {
		Self {
			tokens: TokensCStr::new(input),
		}
	}

	/// Returns the rest of the unparsed argument string.
	pub const fn rest(&self) -> &'a CStr {
		self.tokens.rest()
	}
}

impl<'a> Iterator for Pieces<'a> {
	type Item = &'a [u8];
	fn next(&mut self) -> Option<Self::Item> {
		self.tokens.next()
	}
}
