use ::proc_macro::TokenStream;
use ::proc_macro2::{
	TokenStream as TokenStream2, TokenTree,
	Span, Literal,
};
use ::std::{
	borrow::Cow,
	ffi::{
		c_str::NulError,
		CString, CStr, c_float, c_int,
	},
};
use ::syn::{
	Error, Result,
	Lit,
};
use ::quote::quote_spanned;

use crate::infallible_c_string::*;

pub fn cvar_value_impl(input: Lit) -> Result<TokenStream2> {
	let span = input.span();
	let string = cvar_value_string(&input)?;
	let (float, int): (TokenStream2, TokenStream2) = match input {
		Lit::Int(lit) => {
			(quote_spanned! { span => (#lit) as _ }, lit2stream(lit.token()).into())
		}
		Lit::Float(lit) => {
			(lit2stream(lit.token()).into(), quote_spanned! { span => (#lit) as _ })
		}
		Lit::CStr(..) | Lit::Str(..) | Lit::ByteStr(..) => {
			(quote_spanned! { span => 0.0 }, quote_spanned! { span => 0 })
		}
		Lit::Bool(lit) => {
			let (float, int): (c_float, c_int) = if lit.value {
				(1.0, 1)
			} else {
				(0.0, 0)
			};
			(quote_spanned! { span => #float }, quote_spanned! { span => #int })
		}
		input => return Err(type_error(&input)),
	};
	Ok(quote_spanned! {
		span => ConVarValue {
			c_str: #string,
			float: #float,
			int: #int,
		}
	})
}

pub fn cvar_value_string(input: &Lit) -> Result<Cow<'static, CStr>> {
	match input {
		Lit::Int(value) => {
			let span = value.span();
			let value_string = CString::new(value.base10_digits())
				.map_err(move |e| nul_error(span, e))?;
			Ok(Cow::Owned(value_string))
		}
		Lit::Float(value) => {
			let span = value.span();
			let value_string = CString::new(value.base10_digits())
				.map_err(move |e| nul_error(span, e))?;
			Ok(Cow::Owned(value_string))
		}
		Lit::CStr(s) => {
			Ok(Cow::Owned(s.value()))
		}
		Lit::Str(s) => {
			let value_string = c_string_from_string(s.value());
			Ok(Cow::Owned(value_string))
		}
		Lit::ByteStr(s) => {
			let value_string = c_string_from_bytes(s.value());
			Ok(Cow::Owned(value_string))
		}
		Lit::Bool(b) => {
			let value = if b.value { c"1" } else { c"0" };
			Ok(Cow::Borrowed(value))
		}
		input => {
			Err(type_error(input))
		}
	}
}

fn type_error(input: &Lit) -> Error {
	Error::new_spanned(input, "ConVar value may only be a number or string")
}

fn nul_error(span: Span, error: NulError) -> Error {
	Error::new(span, error)
}

fn lit2stream(lit: Literal) -> TokenStream {
	TokenStream2::from(TokenTree::from(lit)).into()
}
