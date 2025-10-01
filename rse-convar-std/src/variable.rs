use ::core::ffi::{
	CStr, c_float, c_int,
};
use ::libc::atof;
use ::rse_convar::{
	cppdef::{
		fcvar, COMMAND_MAX_LENGTH,
	},
	console_base::{
		RawConsoleBase, CvarDllIdentifier,
	},
	variable::{
		low::{
			RawVariable, ConVarObject,
		},
		Variable, NewValue, OldValue,
	},
};
use ::rse_game_interfaces::{
	Cvar, CvarImpl,
};

use crate::CBuffer;

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

// TODO: Mimic standard ConVar behavior.
impl<'a, T> RawVariable<'a> for StdVariable<T>
where
	T: Variable,
{
	fn set_c_str(object: &mut ConVarObject<'a, Self>, value: Option<&CStr>) {
		// TODO: Material thread check and queue.
		if !object.as_ext().is_root() {
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

		*object.as_mut_ext().float_mut() = new_float_value;
		*object.as_mut_ext().int_mut() = new_float_value as c_int;

		if !object.as_base().is_flag_set(fcvar::NEVER_AS_STRING) {
			Self::change_string_value(object, value, old_value);
		}
	}
	fn set_float_forced(object: &mut ConVarObject<'a, Self>, mut value: c_float) {
		// TODO: Material thread check and queue.
		if !object.as_ext().is_root() {
			return
		}

		object.as_ext().clamp_value(&mut value);

		let old_value = object.as_ext().float();
		*object.as_mut_ext().float_mut() = value;
		*object.as_mut_ext().int_mut() = value as _;

		if !object.as_base().is_flag_set(fcvar::NEVER_AS_STRING) {
			let old_value_string = old_value_string(object);
			object.inner.value_buffer.print_float(value);
			change_string_value_impl(object, old_value_string.as_c_str(), old_value);
		}
	}
	fn set_int(object: &mut ConVarObject<'a, Self>, mut value: c_int) {
		// TODO: Material thread check and queue.
		if !object.as_ext().is_root() {
			return
		}

		let mut float_value = value as _;
		if object.as_ext().clamp_value(&mut float_value) {
			value = float_value as _;
		}

		let old_value = object.as_ext().float();
		*object.as_mut_ext().float_mut() = float_value;
		*object.as_mut_ext().int_mut() = value;

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
	fn init(object: &mut ConVarObject<'a, Self>) {
		let _ = object;
	}
	fn help(object: &mut ConVarObject<'a, Self>) {
		unsafe {
			object.as_mut_base().as_mut_inner().help_string = T::HELP.map(move |s| s.as_ptr()).unwrap_or_default()
		}
	}
	fn dll_identifier(object: &mut ConVarObject<'a, Self>) -> CvarDllIdentifier {
		object.inner.inner.dll_identifier()
	}
}
