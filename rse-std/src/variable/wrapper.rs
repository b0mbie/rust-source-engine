use ::core::ffi::{
	CStr, c_float, c_int,
};
use ::libc::atof;
use ::rse_convar::{
	cppdef::{
		fcvar, COMMAND_MAX_LENGTH,
	},
	console_base::{
		RawConsoleBase, CvarDllIdentifier, CvarFlags,
	},
	variable::{
		low::{
			RawVariable, ConVarObject,
		},
		Variable, NewValue, OldValue,
	},
};
use ::rse_game_interfaces::cvar::{
	CvarImpl, QueueMaterialThreadValue,
};

use crate::{
	c_buffer::CBuffer,
	cvar::CvarWrite,
};

type ValueBuffer = CBuffer<{COMMAND_MAX_LENGTH - 2}>;

#[derive(Debug, Clone, Copy)]
pub struct StdVariable<T> {
	inner: T,
	value_buffer: ValueBuffer,
}

impl<T> StdVariable<T> {
	pub const fn new(inner: T) -> Self {
		Self {
			inner,
			value_buffer: ValueBuffer::new(),
		}
	}
}

fn set_preamble<T, V>(object: &mut ConVarObject<'_, T>, value: V) -> bool
where
	V: QueueMaterialThreadValue,
{
	// If we're supposed to only be set on the material thread...
	if object.as_base().is_flag_set(fcvar::MATERIAL_THREAD_MASK)
		&& let Some(mut cvar) = CvarWrite::acquire()
		&& !cvar.is_material_thread_set_allowed()
	{
		unsafe { cvar.queue_material_thread_set(object.as_mut_con_var(), value) }
		false
	} else {
		object.as_ext().is_root()
	}
}

impl<'a, T> RawVariable<'a> for StdVariable<T>
where
	T: Variable,
{
	fn set_c_str(object: &mut ConVarObject<'a, Self>, value: Option<&CStr>) {
		if !set_preamble(object, value) {
			return
		}

		let old_value = object.as_ext().float();
		let mut new_float_value = if let Some(value) = value {
			unsafe { atof(value.as_ptr()) as c_float }
		} else {
			0.0
		};

		let mut clamped_float_string = CBuffer::<32>::new();
		let value = if object.as_ext().clamp_value(&mut new_float_value) {
			clamped_float_string.print_float(new_float_value);
			Some(clamped_float_string.as_c_str())
		} else {
			value
		};

		unsafe {
			let con_var = &mut object.as_mut_con_var().data;
			con_var.value_float = new_float_value;
			con_var.value_int = new_float_value as c_int;
		}

		if !object.as_base().is_flag_set(fcvar::NEVER_AS_STRING) {
			Self::change_string_value(object, value, old_value);
		}
	}
	fn set_float_forced(object: &mut ConVarObject<'a, Self>, mut value: c_float) {
		if !set_preamble(object, value) {
			return
		}

		object.as_ext().clamp_value(&mut value);

		let old_value = object.as_ext().float();
		unsafe {
			let con_var = &mut object.as_mut_con_var().data;
			con_var.value_float = value;
			con_var.value_int = value as _;
		}

		if !object.as_base().is_flag_set(fcvar::NEVER_AS_STRING) {
			let old_value_string = old_value_string(object);
			object.inner.value_buffer.print_float(value);
			change_string_value_impl(object, old_value_string.as_c_str(), old_value);
		}
	}
	fn set_int(object: &mut ConVarObject<'a, Self>, mut value: c_int) {
		if !set_preamble(object, value) {
			return
		}

		let mut float_value = value as _;
		if object.as_ext().clamp_value(&mut float_value) {
			value = float_value as _;
		}

		let old_value = object.as_ext().float();
		unsafe {
			let con_var = &mut object.as_mut_con_var().data;
			con_var.value_float = float_value;
			con_var.value_int = value;
		}

		if !object.as_base().is_flag_set(fcvar::NEVER_AS_STRING) {
			let old_value_string = old_value_string(object);
			object.inner.value_buffer.print_int(value);
			change_string_value_impl(object, old_value_string.as_c_str(), old_value);
		}
	}
	fn set_float(object: &mut ConVarObject<'a, Self>, value: c_float) {
		if object.as_ext().float() != value {
			Self::set_float_forced(object, value)
		}
	}
	fn change_string_value(object: &mut ConVarObject<'a, Self>, new_value: Option<&CStr>, old_value: c_float) {
		let old_value_string = old_value_string(object);
		object.inner.value_buffer.set(new_value.unwrap_or(c""));
		change_string_value_impl(object, old_value_string.as_c_str(), old_value)
	}
}

const fn old_value_string<T>(object: &mut ConVarObject<'_, StdVariable<T>>) -> ValueBuffer {
	object.inner.value_buffer
}

fn change_string_value_impl<T>(
	object: &mut ConVarObject<'_, StdVariable<T>>, old_c_str: &CStr, old_value: c_float,
)
where
	T: Variable,
{
	unsafe {
		object.as_mut_ext().as_mut_inner().value_string = object.inner.value_buffer.as_c_str().as_ptr() as _
	}

	if old_c_str != object.as_ext().c_str() {
		// TODO: Figure out how to call change callback.
		// object.as_ext().as_inner().change_callback

		let old = OldValue {
			c_str: old_c_str,
			float: old_value,
		};

		let (ext, inner) = object.ext_and_mut_inner();
		T::on_changed(
			NewValue {
				inner: &mut inner.inner,
				c_str: ext.c_str(),
				float: ext.float(),
				int: ext.int(),
			},
			old,
		);

		// TODO: g_pCVar->CallGlobalChangeCallbacks(this, old_value_string.as_c_str(), old_value);
	}
}

unsafe impl<'a, T> RawConsoleBase<ConVarObject<'a, Self>> for StdVariable<T>
where
	T: Variable,
{
	fn help(object: &mut ConVarObject<'a, Self>) {
		unsafe {
			object.as_mut_base().as_mut_inner().help_string = T::HELP.map(move |s| s.as_ptr()).unwrap_or_default()
		}
	}
	fn add_flags(object: &mut ConVarObject<'a, Self>, flags: CvarFlags) {
		object.as_mut_base().add_flags(flags)
	}
	fn is_registered(object: &mut ConVarObject<'a, Self>) -> bool {
		object.as_base().is_registered()
	}
	fn dll_identifier(object: &mut ConVarObject<'a, Self>) -> CvarDllIdentifier {
		let _ = object;
		crate::cvar::dll_identifier()
	}
}
