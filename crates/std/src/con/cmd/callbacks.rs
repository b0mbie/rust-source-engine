use ::core::ffi::CStr;

use super::{
	Invocation, Suggestions,
};

pub enum Callbacks {
	Functions {
		dispatch: DispatchFn,
		complete: Option<CompleteFn>,
	},
}

#[macro_export]
macro_rules! dispatch_fn {
	(($($args:tt)*) $body:block) => {
		($crate::dispatch_fn! {@inspect_args $($args)*})(move |$($args)*| $body)
	};

	{@inspect_args} => {
		$crate::con::cmd::DispatchFn::Plain
	};
	{@inspect_args $($args:tt)+} => {
		$crate::dispatch_fn! {
			@inspect_args_consequent $($args)+
		}
	};
	{@inspect_args_consequent $(,)?} => {
		$crate::con::cmd::DispatchFn::With
	};
	{@inspect_args_consequent , $($rest:tt)+} => {
		::core::compile_error! {
			"console command dispatch functions may only take 0 or 1 arguments"
		}
	};
	{@inspect_args_consequent $arg:tt $($rest:tt)*} => {
		$crate::dispatch_fn! {
			@inspect_args_consequent $($rest)*
		}
	};
}

#[derive(Clone, Copy)]
pub enum DispatchFn {
	Plain(DispatchPlainFn),
	With(DispatchWithFn),
}

pub type DispatchPlainFn = fn();
pub type DispatchWithFn = fn(invocation: &Invocation);
pub type CompleteFn = fn(partial: &CStr, suggestions: &mut Suggestions);
