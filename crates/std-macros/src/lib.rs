//! Implementation details of some `rse-std` features.

use ::darling::FromMeta;
use ::proc_macro::TokenStream;
use proc_macro2::Span;
use ::proc_macro2::TokenStream as TokenStream2;
use ::syn::{
	parse_macro_input, parse,
	Result,
	Lit,
	ItemStatic,
	ItemFn,
	Type, Expr,
};
use ::quote::{
	quote, ToTokens,
};

pub(crate) mod infallible_c_string;

mod cvar_value;
use cvar_value::*;
mod darling_helpers;
use darling_helpers::*;
use infallible_c_string::*;

#[proc_macro]
pub fn cvar_value_detail(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Lit);
	match cvar_value_impl(input) {
		Ok(stream) => stream.into(),
		Err(error) => error.into_compile_error().into(),
	}
}

#[proc_macro_attribute]
pub fn con_command(args: TokenStream, item: TokenStream) -> TokenStream {
	match con_command_impl(args, item) {
		Ok(s) => s,
		Err(e) => e.into_compile_error().into(),
	}
}

fn con_command_impl(args: TokenStream, item: TokenStream) -> Result<TokenStream> {
	let item: ItemFn = parse(item)?;

	let args: ConCommand = parse(args)?;
	let name = if let Some(name) = args.name {
		name.0
	} else {
		let mut name = item.sig.ident.to_string();
		if !name.contains(char::is_lowercase) {
			name.make_ascii_lowercase();
		}
		c_string_from_string(name)
	};
	let help = opt_to_stream(args.help);
	let flags = args.flags.unwrap_or_else(default_flags);
	let complete = opt_to_stream(args.complete);

	let vis = item.vis.clone();
	let ident = &item.sig.ident;

	let mut item_name = item.sig.ident.clone();
	item_name.set_span(Span::mixed_site());

	Ok(quote! {
		#[allow(non_upper_case_globals)]
		#vis static #item_name: ::rse_std::con::cmd::ConCommand = ::rse_std::con::cmd::ConCommand::new(
			#name, #help,
			#flags,
			{
				#item
				#ident
			},
			#complete,
		);
	}.into())
}

#[derive(FromMeta)]
#[darling(derive_syn_parse)]
struct ConCommand {
	#[darling(default)]
	pub name: Option<DarlCString>,
	#[darling(default)]
	pub help: Option<DarlCString>,
	#[darling(default)]
	pub flags: Option<Expr>,

	#[darling(default)]
	pub complete: Option<Expr>,
}

#[proc_macro_attribute]
pub fn con_var(args: TokenStream, item: TokenStream) -> TokenStream {
	match con_var_impl(args, item) {
		Ok(s) => s,
		Err(e) => e.into_compile_error().into(),
	}
}

fn con_var_impl(args: TokenStream, item: TokenStream) -> Result<TokenStream> {
	let mut item: ItemStatic = parse(item)?;
	item.ty = Box::new({
		let ty = item.ty;
		Type::Verbatim(quote! { ::rse_std::con::var::TypedConVar<#ty> })
	});

	let args: ConVar = parse(args)?;
	let name = if let Some(name) = args.name {
		name.0
	} else {
		let mut name = item.ident.to_string();
		if !name.contains(char::is_lowercase) {
			name.make_ascii_lowercase();
		}
		c_string_from_string(name)
	};
	let help = opt_to_stream(args.help);
	let flags = args.flags.unwrap_or_else(default_flags);
	let min = opt_to_stream(args.min);
	let max = opt_to_stream(args.max);
	let comp_min = opt_to_stream(args.comp_min);
	let comp_max = opt_to_stream(args.comp_max);

	item.expr = Box::new({
		let default = item.expr;
		Expr::Verbatim(quote! {
			unsafe { ::rse_std::con::var::TypedConVar::new(
				::rse_std::con::var::ConVarParams {
					name: #name,
					default: ::rse_std::cvar_value!(#default),
					help: #help,
					min: #min, max: #max,
					comp_min: #comp_min, comp_max: #comp_max,
					flags: #flags,
				},
			) }
		})
	});

	Ok(item.into_token_stream().into())
}

fn default_flags() -> Expr {
	Expr::Verbatim(quote! { ::rse_std::con::CvarFlags::empty() })
}

fn opt_to_stream<T: ToTokens>(opt: Option<T>) -> TokenStream2 {
	if let Some(t) = opt {
		quote! { ::core::option::Option::Some(#t) }
	} else {
		quote! { ::core::option::Option::None }
	}
}

#[derive(FromMeta)]
#[darling(derive_syn_parse)]
struct ConVar {
	#[darling(default)]
	pub name: Option<DarlCString>,
	#[darling(default)]
	pub help: Option<DarlCString>,
	#[darling(default)]
	pub flags: Option<Expr>,

	#[darling(default)]
	pub min: Option<DarlLimitValue>,
	#[darling(default)]
	pub max: Option<DarlLimitValue>,
	#[darling(default)]
	pub comp_min: Option<DarlLimitValue>,
	#[darling(default)]
	pub comp_max: Option<DarlLimitValue>,
}
