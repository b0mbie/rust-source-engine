use ::rse_client::interfaces::{
	OVEngineClient, OVEngineClient013,
};

use crate::{
	plugin::PluginFactories,
	raw::global_interface::GlobalInterface,
};

pub use ::rse_client::interfaces::{
	VEngineClient, VEngineClient013,
};

/// Tries to initialize the `IVEngineClient` functions in this module.
/// 
/// # Safety
/// This function must be called from the main thread.
pub(crate) unsafe fn attach(factories: PluginFactories) -> bool {
	let result = factories
		.create_interface().map(Inner::V14)
		.or_else(move |_| factories.create_interface().map(Inner::V13))
		;
	match result {
		Ok(inner) => {
			unsafe { CLIENT.inspect_unchecked(move |cl| *cl = inner) }
			true
		}
		Err(error) => {
			::rse_tier0::con_warn!("{error}");
			false
		}
	}
}

#[cold]
const fn not_init() -> ! {
	panic!("client interface used without being initialized")
}

/// # Safety
/// This function must not be called
/// in a function that is called by
/// [`inspect`] or [`inspect_mt`].
/// 
/// Additionally, the operations performed on the interface
/// *must* support multi-threading.
pub unsafe fn inspect<R, F: FnOnce(Option<&mut Client>) -> R>(f: F) -> R {
	let f = move |inner: Option<&mut Inner>| {
		let cl = inner.map(move |inner| {
			match Client::from_mut(inner) {
				Some(sv) => sv,
				None => not_init(),
			}
		});
		f(cl)
	};
	unsafe { CLIENT.inspect(f) }
}

/// # Safety
/// This function must not be called
/// in a function that is called by
/// [`inspect`] or [`inspect_mt`].
/// 
/// Additionally, the operations performed on the interface
/// *must* support multi-threading.
pub unsafe fn inspect_mt<R, F: FnOnce(&Client) -> R>(f: F) -> R {
	let f = move |inner: &Inner| {
		match Client::from_ref(inner) {
			Some(sv) => f(sv),
			None => not_init(),
		}
	};
	unsafe { CLIENT.inspect_mt(f) }
}

#[repr(transparent)]
pub struct Client {
	// INVARIANT: This value is never `Inner::None`.
	inner: Inner,
}

impl Client {
	pub fn to_v14(&self) -> Option<&VEngineClient> {
		match self.inner {
			Inner::V14(ref cl) => Some(cl),
			_ => None,
		}
	}

	pub fn as_v13(&self) -> &VEngineClient013 {
		match self.inner {
			Inner::V14(ref cl) => cl.as_v013(),
			Inner::V13(ref cl) => cl,
			_ => unsafe { ::core::hint::unreachable_unchecked() },
		}
	}

	const fn from_mut(inner: &mut Inner) -> Option<&mut Self> {
		match inner {
			Inner::None => None,
			// SAFETY: `Self` is a transparent wrapper for `Inner`.
			inner => unsafe { Some(&mut *(inner as *mut Inner as *mut Self)) }
		}
	}

	const fn from_ref(inner: &Inner) -> Option<&Self> {
		match inner {
			Inner::None => None,
			// SAFETY: `Self` is a transparent wrapper for `Inner`.
			inner => unsafe { Some(&*(inner as *const Inner as *const Self)) }
		}
	}
}

enum Inner {
	None,
	V14(OVEngineClient),
	V13(OVEngineClient013),
}

static CLIENT: GlobalInterface<Inner> = GlobalInterface::new(Inner::None);
