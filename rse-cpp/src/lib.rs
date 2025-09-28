#![no_std]

use ::core::ptr::NonNull;

pub use ::cppdvt::*;

mod flag_utils;
mod transparent_wrappers;
mod vtable_macros;

/// Type of pointers to immutable C++ objects with v-tables (`const IObject *`).
pub type VtObjectRef<VTable> = VtObjectPtr<VTable>;
/// Type of pointers to mutable C++ objects with v-tables (`IObject *`).
pub type VtObjectMut<VTable> = VtObjectPtr<VTable>;

/// Type of immutable C++ references (`const int &`).
pub type RefConst<T> = NonNull<T>;
/// Type of mutable C++ references (`int &`).
pub type RefMut<T> = NonNull<T>;

/// # Safety
/// The implementing type must be a `repr(transparent)` wrapper around a [`VtObject<Self::VTable>`](VtObject).
pub unsafe trait VtObjectWrapper: Sized {
	type VTable;
	fn from_object_const(object: &VtObject<Self::VTable>) -> &Self {
		unsafe { &*(object as *const VtObject<Self::VTable> as *const Self) }
	}
	fn from_object_mut(object: &mut VtObject<Self::VTable>) -> &mut Self {
		unsafe { &mut *(object as *mut VtObject<Self::VTable> as *mut Self) }
	}
	fn to_object_const(&self) -> &VtObject<Self::VTable> {
		unsafe { &*(self as *const Self as *const VtObject<Self::VTable>) }
	}
	fn to_object_mut(&mut self) -> &mut VtObject<Self::VTable> {
		unsafe { &mut *(self as *mut Self as *mut VtObject<Self::VTable>) }
	}
}
impl<T: VtObjectWrapper> AsObject<T::VTable> for T {
	fn as_object(&self) -> &VtObject<T::VTable> {
		unsafe { &*(self as *const Self as *const VtObject<T::VTable>) }
	}
}

#[macro_export]
macro_rules! vt_object_wrapper {
	{
		$(#[$attr:meta])*
		$vis:vis struct $name:ident for $vtable:ty;
	} => {
		#[repr(transparent)]
		$(#[$attr])*
		$vis struct $name {
			object: $crate::VtObject<$vtable>,
		}

		#[automatically_derived]
		unsafe impl $crate::VtObjectWrapper for $name {
			type VTable = $vtable;
		}

		impl<'a> ::core::convert::From<&'a $crate::VtObject<$vtable>> for &'a $name {
			fn from(value: &'a $crate::VtObject<$vtable>) -> Self {
				$crate::VtObjectWrapper::from_object_const(value)
			}
		}

		impl<'a> ::core::convert::From<&'a mut $crate::VtObject<$vtable>> for &'a mut $name {
			fn from(value: &'a mut $crate::VtObject<$vtable>) -> Self {
				$crate::VtObjectWrapper::from_object_mut(value)
			}
		}

		impl<'a> ::core::convert::From<&'a $name> for &'a $crate::VtObject<$vtable> {
			fn from(value: &'a $name) -> Self {
				$crate::VtObjectWrapper::to_object_const(value)
			}
		}

		impl<'a> ::core::convert::From<&'a mut $name> for &'a mut $crate::VtObject<$vtable> {
			fn from(value: &'a mut $name) -> Self {
				$crate::VtObjectWrapper::to_object_mut(value)
			}
		}
	};
}

pub trait OwnedVtObjectWrapper: Sized {
	type VTable;
	/// # Safety
	/// `ptr` must be a valid pointer to an owned C++ object with the associated [`VTable`](OwnedVtObjectWrapper::VTable).
	unsafe fn from_ptr(ptr: VtObjectPtr<Self::VTable>) -> Self;
}

#[macro_export]
macro_rules! owned_vt_object_wrapper {
	{
		$(#[$attr:meta])*
		$vis:vis struct $name:ident for $vtable:ty;
	} => {
		#[repr(transparent)]
		$(#[$attr])*
		$vis struct $name {
			object: &'static mut $crate::VtObject<$vtable>,
		}

		#[automatically_derived]
		impl $crate::OwnedVtObjectWrapper for $name {
			type VTable = $vtable;
			unsafe fn from_ptr(ptr: $crate::VtObjectPtr<Self::VTable>) -> Self {
				unsafe {
					Self {
						object: $crate::VtObject::from_ptr_mut(ptr),
					}
				}
			}
		}

		#[automatically_derived]
		impl $crate::AsObject<$vtable> for $name {
			fn as_object(&self) -> &$crate::VtObject<$vtable> {
				self.object
			}
		}
	};
}

pub trait AsObject<VTable> {
	fn as_object(&self) -> &VtObject<VTable>;
}
impl<VTable> AsObject<VTable> for VtObject<VTable> {
	fn as_object(&self) -> &VtObject<VTable> {
		self
	}
}
