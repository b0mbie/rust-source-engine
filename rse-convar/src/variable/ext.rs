use ::core::{
	ffi::{
		CStr, c_float, c_int,
	},
	ptr::addr_eq,
};

use crate::console_base::ConCommandBaseExt;

use super::{
	GetValue, ValueView,
};

::rse_cpp::transparent_wrapper! {
	pub struct ConVarExt for crate::cppdef::ConVarExt as "ConVar";
}

macro_rules! limit_funcs {
	{
		lim_value = $lim_value:ident;

		$(#[$has_lim_attr:meta])*
		has_lim = $has_lim:ident;
		$(#[$lim_unwrap_attr:meta])*
		lim_unwrap = $lim_unwrap:ident;
		$(#[$lim_attr:meta])*
		lim = $lim:ident;
	} => {
		$(#[$has_lim_attr])*
		pub const fn $has_lim(&self) -> bool {
			self.0.$has_lim
		}

		$(#[$lim_unwrap_attr])*
		pub const fn $lim_unwrap(&self) -> c_float {
			self.0.$lim_value
		}

		$(#[$lim_attr])*
		pub const fn $lim(&self) -> Option<c_float> {
			if self.0.$has_lim {
				Some(self.0.$lim_value)
			} else {
				None
			}
		}
	};
}

impl ConVarExt {
	/// Returns an immutable reference to the inner [`ConCommandBaseExt`].
	pub const fn as_base(&self) -> &ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_ref(&self.0.base) }
	}

	/// Returns a mutable reference to the inner [`ConCommandBaseExt`].
	pub const fn as_mut_base(&mut self) -> &mut ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_mut(&mut self.0.base) }
	}

	/// Returns `true` if this ConVar is the root ConVar (its parent is `self`).
	pub fn is_root(&self) -> bool {
		addr_eq(self.parent(), self)
	}

	/// Returns an immutable reference to the parent of this ConVar, which can be `self`.
	pub const fn parent(&self) -> &Self {
		unsafe { Self::from_ref(&(&*self.0.parent).data) }
	}

	/// Returns a mutable reference to the parent of this ConVar, which can be `self`.
	pub const fn parent_mut(&mut self) -> &mut Self {
		unsafe { Self::from_mut(&mut (&mut *self.0.parent).data) }
	}

	/// Returns the default string value of this ConVar.
	pub const fn default(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.default_value) }
	}

	/// Returns the `T` value of this ConVar.
	pub fn value<'a, T: GetValue<'a>>(&'a self) -> T {
		T::get_value(ValueView::new(self))
	}

	/// Returns the [`CStr`] value of this ConVar.
	pub const fn c_str(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.value_string) }
	}

	/// Returns the [`c_float`] value of this ConVar.
	pub const fn float(&self) -> c_float {
		self.0.value_float
	}

	/// Returns the [`c_int`] value of this ConVar.
	pub const fn int(&self) -> c_int {
		self.0.value_int
	}

	limit_funcs! {
		lim_value = min_value;
		/// Returns `true` if this ConVar has a minimum value.
		has_lim = has_min;
		/// Returns the minimum value of this ConVar,
		/// or `0` if it doesn't have one.
		lim_unwrap = min_unwrap;
		/// Returns the minimum value of this ConVar,
		/// or `None` if it doesn't have one.
		lim = min;
	}

	limit_funcs! {
		lim_value = max_value;
		/// Returns `true` if this ConVar has a maximum value.
		has_lim = has_max;
		/// Returns the maximum value of this ConVar,
		/// or `0` if it doesn't have one.
		lim_unwrap = max_unwrap;
		/// Returns the maximum value of this ConVar,
		/// or `None` if it doesn't have one.
		lim = max;
	}

	limit_funcs! {
		lim_value = comp_min_value;
		/// Returns `true` if this ConVar has a minimum value
		/// for competitive play.
		has_lim = has_comp_min;
		/// Returns the minimum value of this ConVar for competitive play,
		/// or `0` if it doesn't have one.
		lim_unwrap = comp_min_unwrap;
		/// Returns the minimum value of this ConVar for competitive play,
		/// or `None` if it doesn't have one.
		lim = comp_min;
	}

	limit_funcs! {
		lim_value = comp_max_value;
		/// Returns `true` if this ConVar has a maximum value
		/// for competitive play.
		has_lim = has_comp_max;
		/// Returns the maximum value of this ConVar for competitive play,
		/// or `0` if it doesn't have one.
		lim_unwrap = comp_max_unwrap;
		/// Returns the maximum value of this ConVar for competitive play,
		/// or `None` if it doesn't have one.
		lim = comp_max;
	}

	/// Returns `true` if this ConVar is currently using competitive restrictions.
	pub const fn using_competitive_restrictions(&self) -> bool {
		self.0.using_competitive_restrictions
	}

	/// Clamp `value` in place using the limits set in this ConVar,
	/// returning `true` if it was modified by this function.
	pub fn clamp_value(&self, value: &mut c_float) -> bool {
		if self.0.using_competitive_restrictions {
			if clamp(value, self.comp_min(), self.comp_max()) {
				return true
			}
		
			if let Some(default) = to_str(self.default()).and_then(|s| s.parse::<c_float>().ok())
				&& (*value - default).abs() > 0.0001
			{
					*value = default;
					return true
				}

			false
		} else {
			clamp(value, self.min(), self.max())
		}
	}
}

const fn clamp(value: &mut c_float, min: Option<c_float>, max: Option<c_float>) -> bool {
	if let Some(min) = min {
		if *value < min {
			*value = min;
		}
		true
	} else if let Some(max) = max {
		if *value > max {
			*value = max;
		}
		true
	} else {
		false
	}
}

const fn to_str(s: &CStr) -> Option<&str> {
	match s.to_str() {
		Ok(s) => Some(s),
		Err(..) => None,
	}
}
