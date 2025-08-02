#![no_std]

use ::core::ptr::NonNull;

pub use ::cppdvt::*;

mod flag_utils;

/// Type of mutable C++ objects with v-tables (`const IObject *`).
pub type VtObjectRef<VTable> = VtObject<VTable>;
/// Type of mutable C++ objects with v-tables (`IObject *`).
pub type VtObjectMut<VTable> = VtObject<VTable>;

/// Type of immutable C++ references (`const int &`).
pub type RefConst<T> = NonNull<T>;
/// Type of mutable C++ references (`int &`).
pub type RefMut<T> = NonNull<T>;

#[macro_export]
macro_rules! vtable_methods {
	{
		$this:ident : $this_ty:ty;
		$(
			$(#[$attr:meta])*
			fn $name:ident($($param:tt)*) $(-> $return:ty)? {
				$($body:tt)*
			}
		)*
	} => {
		$(
			$(#[$attr])*
			unsafe extern "C-unwind" fn $name($this: $this_ty, $($param)*) $(-> $return)? {
				$($body)*
			}
		)*
	};
}
