#![no_std]

use ::core::ptr::NonNull;

pub use ::cppdvt::*;

mod flag_utils;
mod transparent_wrappers;
mod vtable_macros;

pub mod ptr_compat;

mod vt_object_wrapper;
pub use vt_object_wrapper::*;
mod with_vtable;
pub use with_vtable::*;

/// Type of pointers to immutable C++ objects with v-tables (`const IObject *`).
pub type VtObjectRef<VTable> = VtObjectPtr<VTable>;
/// Type of pointers to mutable C++ objects with v-tables (`IObject *`).
pub type VtObjectMut<VTable> = VtObjectPtr<VTable>;

/// Type of immutable C++ references (`const int &`).
pub type RefConst<T> = NonNull<T>;
/// Type of mutable C++ references (`int &`).
pub type RefMut<T> = NonNull<T>;

pub trait AsObject<VTable> {
	fn as_object(&self) -> &VtObject<VTable>;
}
impl<VTable> AsObject<VTable> for VtObject<VTable> {
	fn as_object(&self) -> &VtObject<VTable> {
		self
	}
}
