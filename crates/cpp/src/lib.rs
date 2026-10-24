#![no_std]

use ::core::ptr::NonNull;

pub use ::bitflags::bitflags;
pub use ::cpp_class::*;

mod flag_utils;
mod opaque_type;
mod transparent_wrappers;
mod vtable_macros;

pub mod c_str;
pub mod ptr_compat;
pub mod util;
pub mod wide_c_str;

mod type_info;
pub use type_info::*;

/// Type of immutable C++ references (`const int &`).
pub type RefConst<T> = NonNull<T>;
/// Type of mutable C++ references (`int &`).
pub type RefMut<T> = NonNull<T>;
