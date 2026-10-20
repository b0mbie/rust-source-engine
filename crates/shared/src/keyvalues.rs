use ::core::ptr::null_mut;

use crate::cppdef::{
	INVALID_KEY_SYMBOL,
	types_t,
	KeyValues as CKeyValues,
	KeyValuesValue,
};

::rse_cpp::transparent_wrapper! {
	pub struct KeyValues for CKeyValues as "KeyValues";
}

impl Default for KeyValues {
	fn default() -> Self {
		Self::new()
	}
}

impl Drop for KeyValues {
	fn drop(&mut self) {
		self.remove_everything()
	}
}

impl KeyValues {
	pub const fn new() -> Self {
		Self(CKeyValues {
			key_name: INVALID_KEY_SYMBOL,
			data_type: types_t::None as _,
			sub: null_mut(),
			peer: null_mut(),
			chain: null_mut(),
			s_value: null_mut(),
			ws_value: null_mut(),
			value: KeyValuesValue {
				p_value: null_mut(),
			},
			has_escape_sequences: false,
			evaluate_conditionals: true,
			unused: [0; _],
		})
	}

	pub const fn sub_mut(&mut self) -> Option<&mut Self> {
		unsafe {
			if let Some(kv) = self.0.sub.as_mut() {
				Some(Self::from_mut(kv))
			} else {
				None
			}
		}
	}

	pub const fn peer_mut(&mut self) -> Option<&mut Self> {
		unsafe {
			if let Some(kv) = self.0.peer.as_mut() {
				Some(Self::from_mut(kv))
			} else {
				None
			}
		}
	}

	pub fn remove_everything(&mut self) {
		self.0.s_value = null_mut();
		self.0.ws_value = null_mut();
	}
}
