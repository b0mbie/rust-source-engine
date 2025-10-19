use ::core::ffi::{
	CStr, c_float, c_int,
};
use ::rse_convar::{
	cppdef::ConVar as CConVar,
	console_base::{
		CvarDllIdentifier,
		RegistrableMut,
	},
	ConCommandBase, ConVar, ConCommand,
};
use ::rse_cpp::{
	AsObject,
	virtual_call,
};

use crate::cppdef::{
	CVAR_INTERFACE_VERSION, CvarVt,
};

use super::{
	InterfaceOfFactory, AppSystemFactory,
};

pub mod registered;
use registered::*;

pub trait CvarImpl: AsObject<CvarVt> {
	/// Returns a unique identifier that must be used for console variables and commands
	/// that are registered with this interface.
	fn allocate_dll_identifier(&mut self) -> CvarDllIdentifier {
		unsafe { virtual_call!(self.as_object() => cvar.allocate_dll_identifier()) }
	}

	/// Returns `true` if ConVars that must be set on the material thread can be set now.
	/// 
	/// Generally, this function will return `true`
	/// if the material thread is the thread setting a ConVar.
	/// If this function returns `false` while a ConVar is being set,
	/// that ConVar should call [`queue_material_thread_set`](CvarImpl::queue_material_thread_set) on itself
	/// to queue the assignment to be done again on the material thread.
	fn is_material_thread_set_allowed(&self) -> bool {
		unsafe { virtual_call!(self.as_object() => cvar.is_material_thread_set_allowed()) }
	}

	/// Queues the given `con_var` to be set to `value` on the material thread.
	/// 
	/// This function should typically be called when
	/// [`is_material_thread_set_allowed`](CvarImpl::is_material_thread_set_allowed) returns `false`,
	/// but doing otherwise should be sound.
	/// It is advised against doing so, however.
	/// 
	/// # Safety
	/// `con_var` must be valid when the material thread sets it.
	unsafe fn queue_material_thread_set<V>(&mut self, con_var: *mut CConVar, value: V)
	where
		V: QueueMaterialThreadValue,
	{
		unsafe { V::queue_material_thread_set(self, con_var, value) }
	}

	/// Register the console variable or command `registrable` with this interface,
	/// linking it into the global linked list.
	/// 
	/// # Safety
	/// `registrable` *must* be registrable with the `ICvar` interface.
	/// 
	/// Registered console commands and variables *must* eventually be unregistered with
	/// [`unregister_raw`](CvarImpl::unregister_raw) or [`unregister_all`](CvarImpl::unregister_all).
	unsafe fn register_raw(&mut self, registrable: RegistrableMut) {
		unsafe { virtual_call!(self.as_object() => cvar.register_con_command(registrable)) }
	}

	/// Unregister the console variable or command `registrable` with this interface,
	/// unlinking it from the global linked list.
	/// 
	/// # Safety
	/// `registrable` must have been registered with this interface.
	unsafe fn unregister_raw(&mut self, registrable: RegistrableMut) {
		unsafe { virtual_call!(self.as_object() => cvar.unregister_con_command(registrable)) }
	}

	/// Unregister all console variables and commands associated with the given `dll_identifier`.
	/// 
	/// # Safety
	/// `dll_identifier` must be a [`CvarDllIdentifier`] that was returned by a call to
	/// [`allocate_dll_identifier`](CvarImpl::allocate_dll_identifier).
	unsafe fn unregister_all(&mut self, dll_identifier: CvarDllIdentifier) {
		unsafe { virtual_call!(self.as_object() => cvar.unregister_con_commands(dll_identifier)) }
	}

	/// Finds a named console variable or command,
	/// returning `None` if one was not found.
	/// 
	/// # Safety
	/// The returned value
	/// may be unregistered at any time on the main thread.
	/// Therefore, this function *must* only be called on the main thread.
	unsafe fn find(&self, name: &CStr) -> Option<&ConCommandBase> {
		unsafe {
			virtual_call!(self.as_object() => cvar.find_command_base_const(name.as_ptr()))
				.as_ref().map(move |ptr| ConCommandBase::from_ref(ptr))
		}
	}

	/// Finds a named console variable or command,
	/// returning `None` if one was not found.
	/// 
	/// # Safety
	/// The returned value
	/// may be unregistered at any time on the main thread.
	/// Therefore, this function *must* only be called on the main thread.
	unsafe fn find_mut(&mut self, name: &CStr) -> Option<&mut ConCommandBase> {
		unsafe {
			virtual_call!(self.as_object() => cvar.find_command_base(name.as_ptr()))
				.as_mut().map(move |ptr| ConCommandBase::from_mut(ptr))
		}
	}

	/// Finds a named console variable,
	/// returning `None` if one was not found.
	/// 
	/// # Safety
	/// The returned value
	/// may be unregistered at any time on the main thread.
	/// Therefore, this function *must* only be called on the main thread.
	unsafe fn find_var(&self, name: &CStr) -> Option<&ConVar> {
		unsafe {
			virtual_call!(self.as_object() => cvar.find_var_const(name.as_ptr()))
				.as_ref().map(move |ptr| ConVar::from_ref(ptr))
		}
	}

	/// Finds a named console variable,
	/// returning `None` if one was not found.
	/// 
	/// # Safety
	/// The returned value
	/// may be unregistered at any time on the main thread.
	/// Therefore, this function *must* only be called on the main thread.
	unsafe fn find_var_mut(&mut self, name: &CStr) -> Option<&mut ConVar> {
		unsafe {
			virtual_call!(self.as_object() => cvar.find_var(name.as_ptr()))
				.as_mut().map(move |ptr| ConVar::from_mut(ptr))
		}
	}

	/// Finds a named console command,
	/// returning `None` if one was not found.
	/// 
	/// # Safety
	/// The returned value
	/// may be unregistered at any time on the main thread.
	/// Therefore, this function *must* only be called on the main thread.
	unsafe fn find_cmd(&self, name: &CStr) -> Option<&ConCommand> {
		unsafe {
			virtual_call!(self.as_object() => cvar.find_command_const(name.as_ptr()))
				.as_ref().map(move |ptr| ConCommand::from_ref(ptr))
		}
	}

	/// Finds a named console command,
	/// returning `None` if one was not found.
	/// 
	/// # Safety
	/// The returned value
	/// may be unregistered at any time on the main thread.
	/// Therefore, this function *must* only be called on the main thread.
	unsafe fn find_cmd_mut(&mut self, name: &CStr) -> Option<&mut ConCommand> {
		unsafe {
			virtual_call!(self.as_object() => cvar.find_command(name.as_ptr()))
				.as_mut().map(move |ptr| ConCommand::from_mut(ptr))
		}
	}

	/// Returns an iterator over all registered console variables and commands.
	/// 
	/// # Safety
	/// Values yielded by the iterator
	/// may be unregistered at any time on the main thread.
	/// Therefore, this function *must* only be called on the main thread.
	#[doc(alias = "GetCommands")]
	unsafe fn registered(&self) -> RegisteredIter<'_> {
		unsafe {
			let ptr = virtual_call!(self.as_object() => cvar.get_commands_const());
			RegisteredIter::from_ptr(ptr)
		}
	}

	/// Returns an iterator over all registered console variables and commands,
	/// allowing mutable access to them.
	/// 
	/// # Safety
	/// Values yielded by the iterator
	/// may be unregistered at any time on the main thread.
	/// Therefore, this function *must* only be called on the main thread.
	#[doc(alias = "GetCommands")]
	unsafe fn registered_mut(&mut self) -> RegisteredIterMut<'_> {
		unsafe {
			let ptr = virtual_call!(self.as_object() => cvar.get_commands());
			RegisteredIterMut::from_ptr(ptr)
		}
	}
}
impl<T: AsObject<CvarVt>> CvarImpl for T {}

pub trait QueueMaterialThreadValue {
	/// # Safety
	/// `con_var` must be valid when the material thread sets it.
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut CConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl;
}

impl QueueMaterialThreadValue for Option<&CStr> {
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut CConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl,
	{
		unsafe {
			virtual_call!(
				cvar.as_object()
				=> cvar.queue_material_thread_set_value_string(
					con_var, value.map(move |s| s.as_ptr()).unwrap_or_default(),
				)
			)
		}
	}
}

impl QueueMaterialThreadValue for &CStr {
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut CConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl,
	{
		unsafe {
			virtual_call!(cvar.as_object() => cvar.queue_material_thread_set_value_string(con_var, value.as_ptr()))
		}
	}
}

impl QueueMaterialThreadValue for c_float {
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut CConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl,
	{
		unsafe {
			virtual_call!(cvar.as_object() => cvar.queue_material_thread_set_value_float(con_var, value))
		}
	}
}

impl QueueMaterialThreadValue for c_int {
	unsafe fn queue_material_thread_set<Cvar>(cvar: &mut Cvar, con_var: *mut CConVar, value: Self)
	where
		Cvar: ?Sized + CvarImpl,
	{
		unsafe {
			virtual_call!(cvar.as_object() => cvar.queue_material_thread_set_value_int(con_var, value))
		}
	}
}

::rse_cpp::owned_vt_object_wrapper! {
	pub struct Cvar for CvarVt;
}
unsafe impl ::rse_interface::Interface for Cvar {
	const IDENTIFIER: &CStr = CVAR_INTERFACE_VERSION;
}
impl InterfaceOfFactory for Cvar {
	type Factory = AppSystemFactory;
}