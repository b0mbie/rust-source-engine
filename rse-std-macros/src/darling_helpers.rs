use ::darling::{
	Result, Error,
	FromMeta,
	ToTokens,
};
use ::proc_macro2::TokenStream;
use ::std::ffi::{
	CString, c_float,
};
use ::syn::Lit;

#[repr(transparent)]
pub struct DarlLimitValue(pub c_float);

impl ToTokens for DarlLimitValue {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens)
	}
}

impl FromMeta for DarlLimitValue {
	fn from_value(value: &Lit) -> Result<Self> {
		(match value {
			Lit::Float(n) => n.base10_parse::<c_float>().map(Self).map_err(Error::from),
			Lit::Int(n) => n.base10_parse::<c_float>().map(Self).map_err(Error::from),
			Lit::Bool(b) => Ok(Self(if b.value { 1.0 } else { 0.0 })),
			_ => Err(Error::unexpected_lit_type(value)),
		})
		.map_err(|e| e.with_span(value))
	}
}

#[repr(transparent)]
pub struct DarlCString(pub CString);

impl ToTokens for DarlCString {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.0.to_tokens(tokens)
	}
}

impl FromMeta for DarlCString {
	fn from_value(value: &Lit) -> Result<Self> {
		(match value {
			Lit::Bool(b) => Self::from_bool(b.value),
			Lit::Str(s) => Self::from_string(&s.value()),
			Lit::CStr(s) => Ok(Self(s.value())),
			_ => Err(Error::unexpected_lit_type(value)),
		})
		.map_err(|e| e.with_span(value))
	}

	fn from_string(value: &str) -> Result<Self> {
		match CString::new(value) {
			Ok(s) => Ok(Self(s)),
			Err(e) => Err(Error::custom(e)),
		}
	}
}