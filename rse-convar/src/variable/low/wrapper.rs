use ::core::{
	ffi::{
		CStr, c_float, c_int,
	},
	ptr::addr_eq,
};

use crate::console_base::ConCommandBaseExt;

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
	pub const fn as_base(&self) -> &ConCommandBaseExt {
		unsafe { ConCommandBaseExt::from_ref(&self.0.base) }
	}

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

	pub const fn default(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.default_value) }
	}

	pub const fn c_str(&self) -> &CStr {
		unsafe { CStr::from_ptr(self.0.value_string) }
	}

	pub const fn float(&self) -> c_float {
		self.0.value_float
	}

	pub const fn float_mut(&mut self) -> &mut c_float {
		&mut self.0.value_float
	}

	pub const fn int(&self) -> c_int {
		self.0.value_int
	}

	pub const fn int_mut(&mut self) -> &mut c_int {
		&mut self.0.value_int
	}

	limit_funcs! {
		lim_value = min_value;
		has_lim = has_min;
		lim_unwrap = min_unwrap;
		lim = min;
	}

	limit_funcs! {
		lim_value = max_value;
		has_lim = has_max;
		lim_unwrap = max_unwrap;
		lim = max;
	}

	limit_funcs! {
		lim_value = comp_min_value;
		has_lim = has_comp_min;
		lim_unwrap = comp_min_unwrap;
		lim = comp_min;
	}

	limit_funcs! {
		lim_value = comp_max_value;
		has_lim = has_comp_max;
		lim_unwrap = comp_max_unwrap;
		lim = comp_max;
	}

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
