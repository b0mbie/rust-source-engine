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
