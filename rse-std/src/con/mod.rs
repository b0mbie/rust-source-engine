use ::rse_game_interfaces::cvar::{
	registered::{
		RegisteredIter, RegisteredIterMut,
	},
	Cvar, CvarImpl,
};
use ::std::ffi::CStr;

pub use ::rse_convar::{
	console_base::{
		RegistrableMut, CvarFlags,
	},
	ConCommandBase as Registered, ConVar as Variable, ConCommand as Command,
};

pub(crate) mod cvar;

pub mod cmd;
pub mod var;

pub fn with_cvars<F: FnOnce(Cvars<'_>) -> R, R>(f: F) -> Option<R> {
	cvar::with_cvar_mut(move |cvar| f(Cvars(cvar)))
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
			unsafe { self.0.$find(name) }
		}

		$(#[$attr])*
		pub fn $find_mut(&mut self, name: &CStr) -> Option<&mut $result> {
			unsafe { self.0.$find_mut(name) }
		}
	};
}

impl<'a> Cvars<'a> {
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
		unsafe { self.0.registered() }
	}

	/// Returns an iterator over all registered console variables and commands,
	/// allowing mutable access to them.
	pub fn registered_mut(&mut self) -> RegisteredIterMut<'_> {
		unsafe { self.0.registered_mut() }
	}
}
