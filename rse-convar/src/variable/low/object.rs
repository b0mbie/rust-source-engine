use ::core::{
	ffi::{
		CStr, c_char, c_float, c_int,
	},
	marker::{
		PhantomData, PhantomPinned,
	},
	ptr::null_mut,
};
use ::rse_cpp::{
	new_vtable_self, vtable_methods, this_to_self,
	VtObjectPtr, RefMut,
	VTablePtr,
};

use crate::{
	cppdef::{
		ConVar, ConVarVt, ConVarVtBase,
		ConVarExt as CConVarExt,
		ConCommandBaseExt as CConCommandBaseExt,
		CvarDllIdentifier,
		FnChangeCallback,
	},
	console_base::ConCommandBaseExt,
};

use super::{
	RawVariable, ConVarExt, ConVarParams,
};

macro_rules! cvar_call {
	($vt_object:expr => $name:ident($($arg:tt)*)) => {{
		let vt_object: &::rse_cpp::VtObject<ConVarVt> = $vt_object;
		(vt_object.vtable().con_var.$name)(vt_object.as_ptr(), $($arg)*)
	}};
}

const fn limit_value(limit: Option<c_float>) -> c_float {
	match limit {
		Some(v) => v,
		None => 0.0,
	}
}

const unsafe fn c_str_from<'a>(ptr: *const c_char) -> Option<&'a CStr> {
	if !ptr.is_null() {
		unsafe { Some(CStr::from_ptr(ptr)) }
	} else {
		None
	}
}

#[repr(transparent)]
pub struct StaticConVarObject<T> {
	maybe_unparented: ConVarObject<'static, T>,
}

impl<T> StaticConVarObject<T>
where
	T: RawVariable<'static>,
{
	pub const fn new(inner: T, params: ConVarParams<'static>) -> Self {
		Self {
			maybe_unparented: ConVarObject::unparented(inner, params),
		}
	}

	pub const fn as_inner(&mut self) -> &mut ConVarObject<'static, T> {
		self.maybe_unparented.init_parent();
		&mut self.maybe_unparented
	}
}

#[repr(C)]
pub struct ConVarObject<'a, T> {
	con_var: ConVar,
	pub inner: T,
	_strings: PhantomData<&'a CStr>,
}

impl<T> ConVarObject<'_, T> {
	pub const fn init_parent(&mut self) {
		if self.con_var.data.parent.is_null() {
			self.con_var.data.parent = &mut self.con_var;
		}
	}

	pub const fn as_base(&self) -> &ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_ref(&self.con_var.data.base) }
	}

	pub const fn as_mut_base(&mut self) -> &mut ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_mut(&mut self.con_var.data.base) }
	}

	pub const fn as_ext(&self) -> &ConVarExt {
		unsafe { ConVarExt::from_ref(&self.con_var.data) }
	}

	pub const fn as_mut_ext(&mut self) -> &mut ConVarExt {
		unsafe { ConVarExt::from_mut(&mut self.con_var.data) }
	}

	pub const fn ext_and_mut_inner(&mut self) -> (&ConVarExt, &mut T) {
		unsafe {
			(ConVarExt::from_ref(&self.con_var.data), &mut self.inner)
		}
	}

	const fn parent_mut(&mut self) -> &mut ConVar {
		unsafe { &mut *self.con_var.data.parent }
	}
}

impl<'a, T> ConVarObject<'a, T>
where
	T: RawVariable<'a>,
{
	pub const fn unparented(inner: T, params: ConVarParams<'a>) -> Self {
		let ConVarParams { name, help, default, min, max, comp_min, comp_max } = params;
		Self {
			con_var: ConVar::new(
				unsafe { VTablePtr::new_unchecked(Self::VTABLE as *const _ as *mut _) },
				CConVarExt {
					base: CConCommandBaseExt {
						next: None,
						registered: false,
						name: name.as_ptr(),
						help_string: crate::util::c_str_ptr(help),
						// TODO: Flags.
						flags: 0,
					},

					parent: null_mut(),
					parent_pin: PhantomPinned,

					default_value: default.c_str.as_ptr(),
					value_string: default.c_str.as_ptr() as _,
					string_length: 0,
					value_float: default.float,
					value_int: default.int,
					has_min: min.is_some(),
					min_value: limit_value(min),
					has_max: max.is_some(),
					max_value: limit_value(max),
					// TODO: Change callback?
					change_callback: None,

					has_comp_min: comp_min.is_some(),
					comp_min_value: limit_value(comp_min),
					has_comp_max: comp_max.is_some(),
					comp_max_value: limit_value(comp_max),
					using_competitive_restrictions: false,
				},
			),
			inner,
			_strings: PhantomData,
		}
	}

	const VTABLE: &'static ConVarVt = &ConVarVt {
		con_var: new_vtable_self!(ConVarVtBase {
			destructor,
			#[cfg(not(windows))]
			destructor_2,
			is_command,
			#[cfg(not(windows))]
			is_flag_set,
			add_flags,
			#[cfg(not(windows))]
			get_name,
			get_help_text,
			is_registered,
			get_dll_identifier,
			create_base,
			init,
			#[cfg(not(windows))]
			set_value_string,
			#[cfg(not(windows))]
			set_value_float,
			#[cfg(not(windows))]
			set_value_int,
			internal_set_value,
			internal_set_float_value,
			internal_set_int_value,
			clamp_value,
			change_string_value,
			create_vtbl,
			internal_set_float_value_2
		}),
	};

	vtable_methods! {
		this: VtObjectPtr<ConVarVt>;
		fn destructor() {
			let _ = this;
			// TODO: Destructor?
		}
		#[cfg(not(windows))]
		fn destructor_2() {
			let _ = this;
			// TODO: Destructor?
		}
		fn is_command() -> bool {
			let _ = this;
			false
		}
		#[cfg(not(windows))]
		fn is_flag_set(flag: c_int) -> bool {
			this_to_self!(ref this).as_base().is_flag_set(flag)
		}
		fn add_flags(flags: c_int) {
			this_to_self!(mut this).as_mut_base().add_flags(flags)
		}
		#[cfg(not(windows))]
		fn get_name() -> *const c_char {
			this_to_self!(ref this).con_var.data.base.name
		}
		fn get_help_text() -> *const c_char {
			let this = this_to_self!(mut this);
			T::help(this);
			this.con_var.data.base.help_string
		}
		fn is_registered() -> bool {
			this_to_self!(ref this).as_base().is_registered()
		}
		fn get_dll_identifier() -> CvarDllIdentifier {
			T::dll_identifier(this_to_self!(mut this))
		}
		fn create_base(name: *const c_char, help_string: *const c_char, flags: c_int) {
			let _ = this;
			let _ = name;
			let _ = help_string;
			let _ = flags;
			// Do nothing here. This method is purely for usage in the `ConCommandBase` constructor.
		}
		fn init() {
			T::init(this_to_self!(mut this))
		}
		#[cfg(not(windows))]
		fn set_value_string(value: *const c_char) {
			let this = this_to_self!(mut this);
			let vt_object = this.parent_mut().as_object();
			unsafe { cvar_call!(vt_object => set_value_string(value)) }
		}
		#[cfg(not(windows))]
		fn set_value_float(value: c_float) {
			let this = this_to_self!(mut this);
			let vt_object = this.parent_mut().as_object();
			unsafe { cvar_call!(vt_object => set_value_float(value)) }
		}
		#[cfg(not(windows))]
		fn set_value_int(value: c_int) {
			let this = this_to_self!(mut this);
			let vt_object = this.parent_mut().as_object();
			unsafe { cvar_call!(vt_object => set_value_int(value)) }
		}

		fn internal_set_value(value: *const c_char) {
			let value = unsafe { c_str_from(value) };
			T::set_c_str(this_to_self!(mut this), value)
		}
		fn internal_set_float_value(value: c_float) {
			T::set_float(this_to_self!(mut this), value)
		}
		fn internal_set_int_value(value: c_int) {
			T::set_int(this_to_self!(mut this), value)
		}
		fn clamp_value(mut value: RefMut<c_float>) -> bool {
			let value = unsafe { value.as_mut() };
			this_to_self!(ref this).as_ext().clamp_value(value)
		}

		fn change_string_value(new_value: *const c_char, old_value: c_float) {
			let new_value = unsafe { c_str_from(new_value) };
			T::change_string_value(this_to_self!(mut this), new_value, old_value);
		}

		fn create_vtbl(
			_name: *const c_char, _default_value: *const c_char,
			_flags: c_int,
			_help_string: *const c_char,
			_has_min: bool, _min_value: c_float,
			_has_max: bool, _max_value: c_float,
			_change_callback: FnChangeCallback,
		) {
			let _ = this;
			// Do nothing. This seems to be pretty much just unused.
		}

		fn internal_set_float_value_2(value: c_float, force: bool) {
			let this = this_to_self!(mut this);
			if force {
				T::set_float_forced(this, value)
			} else {
				T::set_float(this, value)
			}
		}
	}
}
