use ::core::{
	cell::UnsafeCell,
	ffi::CStr,
};
use ::rse_convar::{
	cppdef::{
		ConCommand, ConCommandBits,
		CommandCallback, CompletionCallback,
	},
	console_base::{
		RegistrableMut,
		CvarFlags,
	},
	command::low::ConCommandObject,
};

use super::{
	Invocation, Suggestions,
};

pub enum Callbacks {
	Functions {
		dispatch: DispatchFn,
		complete: Option<CompleteFn>,
	},
}

pub enum DispatchFn {
	Plain(DispatchPlainFn),
	With(DispatchWithFn),
}

pub type DispatchPlainFn = fn();
pub type DispatchWithFn = fn(invocation: &Invocation);
pub type CompleteFn = fn(partial: &CStr, suggestions: &mut Suggestions);

mod wrapper;
use wrapper::*;

#[repr(transparent)]
pub struct GenericConCommand<'str, T>
where
	T: GenericCommand<'str>,
{
	con_command: UnsafeCell<ConCommandObject<'str, StdCommand<T>>>,
}

impl<'str, T> Drop for GenericConCommand<'str, T>
where
	T: GenericCommand<'str>,
{
	fn drop(&mut self) {
		unsafe { T::drop_with_object(self) }
	}
}

unsafe impl<'str, T: Sync> Sync for GenericConCommand<'str, T>
where
	T: GenericCommand<'str>,
{}

impl<'str, T> GenericConCommand<'str, T>
where
	T: GenericCommand<'str>,
{
	pub const fn new(
		inner: T,
		name: &'str CStr, help: Option<&'str CStr>, flags: CvarFlags,
		callbacks: Callbacks,
	) -> Self {
		let mut bits = ConCommandBits::new();
		let (dispatch, complete) = match callbacks {
			Callbacks::Functions { dispatch, complete } => {
				let dispatch = match dispatch {
					DispatchFn::Plain(f) => dispatch_fn(f),
					DispatchFn::With(f) => {
						bits = bits.use_new_command_callback();
						dispatch_with_fn(f)
					}
				};
				let complete = match complete {
					None => CompletionCallback { not_used: () },
					Some(f) => {
						bits = bits.with_completion_callback();
						complete_fn(f)
					}
				};
				(dispatch, complete)
			}
		};
		Self::from_raw(inner, name, help, flags, dispatch, complete, bits)
	}

	const fn from_raw(
		inner: T,
		name: &'str CStr, help: Option<&'str CStr>, flags: CvarFlags,
		dispatch: CommandCallback,
		complete: CompletionCallback,
		bits: ConCommandBits,
	) -> Self {
		let cmd = unsafe { ConCommandObject::new(
			StdCommand::new(inner),
			name, help, flags,
			dispatch, complete, bits,
		) };
		Self {
			con_command: UnsafeCell::new(cmd),
		}
	}

	pub const fn as_inner(&self) -> &ConCommand {
		unsafe { (*self.con_command.get()).as_inner() }
	}

	pub fn register(&'static self) -> bool {
		unsafe { crate::con::cvar::register_raw(self.as_registrable()) }
	}

	pub fn as_registrable(&self) -> RegistrableMut {
		unsafe { (*self.con_command.get()).as_registrable() }
	}
}

pub trait GenericCommand<'str>: Sized {
	/// Allow the implementing type to use the data of [`GenericConCommand`]
	/// to properly destroy itself.
	/// 
	/// # Safety
	/// This function must only be called *once*.
	/// This is already done by [`GenericConCommand`].
	unsafe fn drop_with_object(object: &mut GenericConCommand<'str, Self>) {
		let _ = object;
	}
}
