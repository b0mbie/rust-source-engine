use ::core::ffi::CStr;

use crate::futex::Futex;

use super::{
	Registered, Variable, Command,
};

pub use ::rse_convar::cvar::registered::{
	RegisteredIter, RegisteredIterMut,
};

mod helpers;
pub(crate) use helpers::*;

pub mod raw;
use raw::Cvar;

pub fn with_cvars<F: FnOnce(Cvars<'_>) -> R, R>(f: F) -> Option<R> {
	static USING_CVARS: Futex = Futex::new();
	if USING_CVARS.try_lock() {
		let f = move |cvar: Option<&mut Cvar>| cvar.map(move |cvar| f(Cvars(cvar)));
		let result = unsafe { raw::inspect(f) };
		unsafe { USING_CVARS.unlock() };
		result
	} else {
		None
	}
}

#[repr(transparent)]
pub struct Cvars<'a>(&'a mut Cvar);

macro_rules! cvar_find_fns {
	{
		result = $result:ty;
		find = $find:ident;
		find_mut = $find_mut:ident;
		$(#[$attr:meta])*
	} => {
		$(#[$attr])*
		pub fn $find(&self, name: &CStr) -> Option<&$result> {
			unsafe { self.0.as_v4().$find(name) }
		}

		$(#[$attr])*
		pub fn $find_mut(&mut self, name: &CStr) -> Option<&mut $result> {
			unsafe { self.0.as_mut_v4().$find_mut(name) }
		}
	};
}

impl<'a> Cvars<'a> {
	pub const fn as_inner(&self) -> &Cvar {
		self.0
	}

	pub const fn as_mut_inner(&mut self) -> &mut Cvar {
		self.0
	}

	cvar_find_fns! {
		result = Registered;
		find = find;
		find_mut = find_mut;
		/// Finds a named console variable or command,
		/// returning `None` if one was not found.
	}

	cvar_find_fns! {
		result = Variable;
		find = find_var;
		find_mut = find_var_mut;
		/// Finds a named console variable,
		/// returning `None` if one was not found.
	}

	cvar_find_fns! {
		result = Command;
		find = find_cmd;
		find_mut = find_cmd_mut;
		/// Finds a named console command,
		/// returning `None` if one was not found.
	}

	/// Returns an iterator over all registered console variables and commands.
	pub fn registered(&self) -> RegisteredIter<'_> {
		unsafe { self.0.as_v4().registered() }
	}

	/// Returns an iterator over all registered console variables and commands,
	/// allowing mutable access to them.
	pub fn registered_mut(&mut self) -> RegisteredIterMut<'_> {
		unsafe { self.0.as_mut_v4().registered_mut() }
	}
}
