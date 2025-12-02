use ::core::{
	marker::PhantomData,
	ptr::{
		null, null_mut,
	},
};

use crate::{
	cppdef::Registrable as CRegistrable,
	Registrable,
};

#[repr(transparent)]
pub struct RegisteredIter<'a> {
	current: *const CRegistrable,
	all: PhantomData<&'a ()>,
}

impl<'a> RegisteredIter<'a> {
	/// # Safety
	/// `first` must either be
	/// a [`Registrable`](CRegistrable)
	/// that is valid for the `'a` lifetime
	/// or null.
	pub const unsafe fn from_ptr(first: *const CRegistrable) -> Self {
		Self {
			current: first,
			all: PhantomData,
		}
	}

	pub const fn new(first: &'a Registrable) -> Self {
		unsafe { Self::from_ptr(first.as_ptr()) }
	}

	pub const fn empty() -> Self {
		unsafe { Self::from_ptr(null()) }
	}
}

impl Default for RegisteredIter<'_> {
	fn default() -> Self {
		Self::empty()
	}
}

impl<'a> Iterator for RegisteredIter<'a> {
	type Item = &'a Registrable;
	fn next(&mut self) -> Option<Self::Item> {
		let current = self.current;
		if !current.is_null() {
			let result = unsafe { Registrable::from_ptr(current) };
			self.current = result.as_inner().data.next;
			Some(result)
		} else {
			None
		}
	}
}

#[repr(transparent)]
pub struct RegisteredIterMut<'a> {
	current: *mut CRegistrable,
	all: PhantomData<&'a mut ()>,
}

impl<'a> RegisteredIterMut<'a> {
	/// # Safety
	/// `first` must either be
	/// a [`Registrable`](CRegistrable)
	/// that is valid for the `'a` lifetime
	/// or null.
	pub const unsafe fn from_ptr(first: *mut CRegistrable) -> Self {
		Self {
			current: first,
			all: PhantomData,
		}
	}

	pub const fn new(first: &'a mut Registrable) -> Self {
		unsafe { Self::from_ptr(first.as_mut_ptr()) }
	}

	pub const fn empty() -> Self {
		unsafe { Self::from_ptr(null_mut()) }
	}
}

impl Default for RegisteredIterMut<'_> {
	fn default() -> Self {
		Self::empty()
	}
}

impl<'a> Iterator for RegisteredIterMut<'a> {
	type Item = &'a mut Registrable;
	fn next(&mut self) -> Option<Self::Item> {
		let current = self.current;
		if !current.is_null() {
			let result = unsafe { Registrable::from_mut_ptr(current) };
			self.current = result.as_inner().data.next;
			Some(result)
		} else {
			None
		}
	}
}
