use ::core::{
	ffi::{
		CStr, c_float, c_int,
	},
	fmt,
	ops::Deref,
	pin::Pin,
};
use ::libc::atof;
use ::rse_convar::{
	console_base::{
		RawConsoleBase,
		CvarDllIdentifier, CvarFlags,
	},
	variable::{
		low::{
			RawVariable, ConVarObject,
		},
		ConVarExt,
	},
};
use ::rse_game_interfaces::cvar::QueueMaterialThreadValue;
use ::rse_utl::{
	cppdef::UtlString,
	CString,
};

use crate::{
	c_buffer::CBuffer,
	c_strings,
	con::cvar::{
		is_material_thread_set_allowed,
		queue_material_thread_set,
		call_global_change_callbacks,
	},
	futex::Futex,
};

use super::super::{
	Variable, OldValue, NewValue,
};

pub struct StdCStrLock<'a> {
	c_str: &'a CStr,
	lock: &'a Futex,
}

impl Drop for StdCStrLock<'_> {
	fn drop(&mut self) {
		unsafe { self.lock.unlock() }
	}
}

impl<'a> StdCStrLock<'a> {
	pub const fn get(&self) -> &'a CStr {
		self.c_str
	}
}

impl Deref for StdCStrLock<'_> {
	type Target = CStr;
	fn deref(&self) -> &Self::Target {
		self.get()
	}
}

impl fmt::Debug for StdCStrLock<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.get().fmt(f)
	}
}

#[derive(Debug)]
pub struct StdVariable<T> {
	inner: T,
	value_lock: Futex,
}

impl<T> StdVariable<T> {
	pub const fn new(inner: T) -> Self {
		Self {
			inner,
			value_lock: Futex::new(),
		}
	}

	pub fn set_number(object: Pin<&mut ConVarObject<'_, Self>>, float: c_float, int: c_int) {
		Self::locked(
			object,
			move |object| {
				let data = unsafe { &mut object.get_unchecked_mut().as_mut_raw().data };
				data.value_float = float;
				data.value_int = int;
			}
		)
	}

	pub fn float(object: Pin<&mut ConVarObject<'_, Self>>) -> c_float {
		unsafe { Self::locked(object, move |object| object.as_ext().float()) }
	}

	pub fn int(object: Pin<&mut ConVarObject<'_, Self>>) -> c_int {
		unsafe { Self::locked(object, move |object| object.as_ext().int()) }
	}

	pub fn c_str<'a>(object: Pin<&'a mut ConVarObject<'_, Self>>) -> StdCStrLock<'a> {
		unsafe {
			object.inner.lock_value();
			let object = Pin::into_inner_unchecked(object);
			StdCStrLock {
				c_str: object.as_ext().c_str(),
				lock: &object.inner.value_lock,
			}
		}
	}

	fn locked<R, F: FnOnce(Pin<&mut ConVarObject<'_, Self>>) -> R>(
		mut object: Pin<&mut ConVarObject<'_, Self>>, f: F,
	) -> R {
		unsafe {
			object.inner.lock_value();
			let result = f(object.as_mut());
			object.inner.unlock_value();
			result
		}
	}

	pub fn lock_value(&self) {
		self.value_lock.lock()
	}

	pub unsafe fn unlock_value(&self) {
		unsafe { self.value_lock.unlock() }
	}
}

impl<'a, T> RawVariable<'a> for StdVariable<T>
where
	T: Variable,
{
	fn set_c_str(object: Pin<&mut ConVarObject<'a, Self>>, value: Option<&CStr>) {
		let mut ctx = StdCtx::new(object);
		if !ctx.set_preamble(value) {
			return
		}

		let old_value = ctx.float();
		let mut new_float_value = if let Some(value) = value {
			unsafe { atof(value.as_ptr()) as c_float }
		} else {
			0.0
		};

		let mut clamped_float_string = CBuffer::<32>::new();
		let clamped = unsafe { ctx.ext().clamp_value(&mut new_float_value) };
		let value = if clamped {
			clamped_float_string.print_float(new_float_value);
			Some(clamped_float_string.as_c_str())
		} else {
			value
		};

		ctx.set_number(new_float_value, new_float_value as _);

		if !ctx.object.as_base().are_flags_set(CvarFlags::NEVER_AS_STRING) {
			Self::change_string_value(ctx.object, value, old_value);
		}
	}
	fn set_float_forced(object: Pin<&mut ConVarObject<'a, Self>>, mut value: c_float) {
		let mut ctx = StdCtx::new(object);
		if !ctx.set_preamble(value) {
			return
		}

		unsafe { ctx.ext().clamp_value(&mut value) };

		let old_value = ctx.float();
		ctx.set_number(value, value as _);

		if !ctx.object.as_base().are_flags_set(CvarFlags::NEVER_AS_STRING) {
			let old_value_string = ctx.old_value_string();
			ctx.with_value_string_mut(move |s| c_strings::print_float(s, value));
			ctx.change_string_value_impl(old_value_string.as_c_str(), old_value);
		}
	}
	fn set_int(object: Pin<&mut ConVarObject<'a, Self>>, mut value: c_int) {
		let mut ctx = StdCtx::new(object);
		if !ctx.set_preamble(value) {
			return
		}

		let mut float_value = value as _;
		let clamped = unsafe { ctx.ext().clamp_value(&mut float_value) };
		if clamped {
			value = float_value as _;
		}

		let old_value = ctx.float();
		ctx.set_number(float_value, value);

		if !ctx.object.as_base().are_flags_set(CvarFlags::NEVER_AS_STRING) {
			let old_value_string = ctx.old_value_string();
			ctx.with_value_string_mut(move |s| c_strings::print_int(s, value));
			ctx.change_string_value_impl(old_value_string.as_c_str(), old_value);
		}
	}
	fn set_float(object: Pin<&mut ConVarObject<'a, Self>>, value: c_float) {
		let mut ctx = StdCtx::new(object);
		if ctx.float() != value {
			Self::set_float_forced(ctx.object, value)
		}
	}
	fn change_string_value(object: Pin<&mut ConVarObject<'a, Self>>, new_value: Option<&CStr>, old_value: c_float) {
		let mut ctx = StdCtx::new(object);
		let old_value_string = ctx.old_value_string();
		ctx.with_value_string_mut(
			move |s| if let Some(value) = new_value {
				s.set(value);
			} else {
				s.clear();
			}
		);
		ctx.change_string_value_impl(old_value_string.as_c_str(), old_value)
	}
}

unsafe impl<'a, T> RawConsoleBase<ConVarObject<'a, Self>> for StdVariable<T> {
	fn help(object: Pin<&mut ConVarObject<'a, Self>>) {
		let _ = object;
		// `T::HELP` is already stored inside of the object.
		// unsafe { object.as_mut_base().as_mut_inner().help_string = T::HELP.map(move |s| s.as_ptr()).unwrap_or_default() }
	}
	fn add_flags(object: Pin<&mut ConVarObject<'a, Self>>, flags: CvarFlags) {
		unsafe { object.get_unchecked_mut().as_mut_base().add_flags(flags) }
	}
	fn is_registered(object: Pin<&mut ConVarObject<'a, Self>>) -> bool {
		object.as_base().is_registered()
	}
	fn dll_identifier(object: Pin<&mut ConVarObject<'a, Self>>) -> CvarDllIdentifier {
		let _ = object;
		crate::con::cvar::dll_identifier()
	}
}

#[repr(transparent)]
struct StdCtx<'a, 's, T> {
	pub object: Pin<&'a mut ConVarObject<'s, StdVariable<T>>>,
}

impl<'a, 's, T> StdCtx<'a, 's, T> {
	pub const fn new(object: Pin<&'a mut ConVarObject<'s, StdVariable<T>>>) -> Self {
		Self {
			object,
		}
	}

	pub fn set_number(&mut self, float: c_float, int: c_int) {
		StdVariable::set_number(self.object.as_mut(), float, int)
	}

	pub fn float(&mut self) -> c_float {
		StdVariable::float(self.object.as_mut())
	}

	/// # Safety
	/// The current value of the [`ConVarExt`] must not be accessed.
	/// Use [`Self::set_float`]/[`Self::float`] or [`Self::set_int`]/[`Self::int`] for atomic access instead.
	pub unsafe fn ext(&'a self) -> &'a ConVarExt {
		unsafe { self.object.as_ref().get_ref().as_ext() }
	}

	pub fn set_preamble<V>(&mut self, value: V) -> bool
	where
		V: QueueMaterialThreadValue,
	{
		// If we're supposed to only be set on the material thread...
		if self.object.as_base().flags().is_for_material_thread() {
			unsafe {
				if !is_material_thread_set_allowed() {
					queue_material_thread_set(self.object.as_mut().get_unchecked_mut().as_mut_raw(), value);
					false
				} else {
					true
				}
			}
		} else {
			unsafe { self.ext().is_root() }
		}
	}

	// HACK: `::rse_utl::CString` is used for storing the value.
	pub fn with_value_string_mut<R, F: FnOnce(&mut CString) -> R>(&mut self, f: F) -> R {
		const EMPTY: &CStr = c"";

		self.object.inner.lock_value();

		let mut inner = {
			let ext = unsafe { self.object.as_mut().get_unchecked_mut().as_mut_ext() };
			let ptr = ext.as_inner().value_string;
			let default_ptr = ext.as_inner().default_value;
			if EMPTY.as_ptr() == ptr {
				// The special `EMPTY` value is used to not confuse `CString` into `realloc`-ing a static string.
				CString::new().into_inner()
			} else if default_ptr == ptr {
				// The string value pointer, by default, is assigned to be the default value pointer.
				CString::from(ext.default()).into_inner()
			} else {
				UtlString {
					string: ptr,
				}
			}
		};

		let c_string = unsafe { CString::from_mut(&mut inner) };
		let result = f(c_string);

		// The string may have changed, so we need to fix it.
		unsafe {
			let ptr = inner.string;
			self.object.as_mut().get_unchecked_mut().as_mut_raw().data.value_string = if !ptr.is_null() {
				ptr
			} else {
				EMPTY.as_ptr() as _
			};
		}

		unsafe { self.object.inner.unlock_value() }

		result
	}

	pub fn old_value_string(&self) -> CString {
		unsafe {
			self.object.inner.lock_value();
			let string = CString::from(self.ext().c_str());
			self.object.inner.unlock_value();
			string
		}
	}

	pub fn change_string_value_impl(&mut self, old_c_str: &CStr, old_value: c_float)
	where
		T: Variable,
	{
		self.object.inner.lock_value();
		if old_c_str != unsafe { self.ext().c_str() } {
			// TODO: Figure out how to call change callback.
			// object.as_ext().as_inner().change_callback

			let old = OldValue {
				c_str: old_c_str,
				float: old_value,
			};

			unsafe {
				let (ext, inner) = self.object.as_mut().get_unchecked_mut().ext_and_mut_inner();
				inner.inner.on_changed(
					NewValue {
						c_str: ext.c_str(),
						float: ext.float(),
						int: ext.int(),
					},
					old,
				);
			}

			unsafe {
				call_global_change_callbacks(self.object.as_mut().get_unchecked_mut().as_mut_raw(), old_c_str, old_value);
			}
		}
		unsafe { self.object.inner.unlock_value() }
	}
}
