use ::core::ffi::{
	CStr, c_float,
};
use ::rse_convar::{
	cvar::{
		CvarImpl, QueueMaterialThreadValue,
	},
	cppdef::ConVar,
	console_base::RegistrableMut,
};

use super::raw::{
	self, Cvar,
};

pub unsafe fn call_global_change_callbacks(registered: *mut ConVar, old_string: &CStr, old_float: c_float) {
	let f = move |cvar: Option<&mut Cvar>| {
		if let Some(cvar) = cvar {
			unsafe { cvar.call_global_change_callbacks(registered, old_string, old_float) }
		}
	};
	unsafe { raw::inspect(f) }
}

pub fn is_material_thread_set_allowed() -> bool {
	let f = move |cvar: Option<&Cvar>| {
		cvar.map(move |cvar| cvar.is_material_thread_set_allowed()).unwrap_or(false)
	};
	unsafe { raw::inspect_mt(f) }
}

pub unsafe fn queue_material_thread_set<V: QueueMaterialThreadValue>(con_var: *mut ConVar, value: V) {
	let f = move |cvar: Option<&mut Cvar>| {
		if let Some(cvar) = cvar {
			unsafe { cvar.queue_material_thread_set(con_var, value) }
		}
	};
	unsafe { raw::inspect_unchecked(f) }
}

pub unsafe fn register_raw(registrable: RegistrableMut) -> bool {
	let f = move |cvar: Option<&mut Cvar>| {
		if let Some(cvar) = cvar {
			unsafe { cvar.register_raw(registrable) };
			true
		} else {
			false
		}
	};
	unsafe { raw::inspect(f) }
}
