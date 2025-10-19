use ::core::{
	marker::PhantomData,
	ptr::{
		null, null_mut,
	},
};
use ::rse_convar::{
	cppdef::ConCommandBase as CConCommandBase,
	ConCommandBase,
};

#[repr(transparent)]
pub struct RegisteredIter<'a> {
	current: *const CConCommandBase,
	all: PhantomData<&'a ()>,
}

impl<'a> RegisteredIter<'a> {
	pub const unsafe fn from_ptr(first: *const CConCommandBase) -> Self {
		Self {
			current: first,
			all: PhantomData,
		}
	}

	pub const fn new(first: &'a ConCommandBase) -> Self {
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
	type Item = &'a ConCommandBase;
	fn next(&mut self) -> Option<Self::Item> {
		let current = self.current;
		if !current.is_null() {
			let result = unsafe { ConCommandBase::from_ptr(current) };
			self.current = result.as_inner().data.next;
			Some(result)
		} else {
			None
		}
	}
}

#[repr(transparent)]
pub struct RegisteredIterMut<'a> {
	current: *mut CConCommandBase,
	all: PhantomData<&'a mut ()>,
}

impl<'a> RegisteredIterMut<'a> {
	pub const unsafe fn from_ptr(first: *mut CConCommandBase) -> Self {
		Self {
			current: first,
			all: PhantomData,
		}
	}

	pub const fn new(first: &'a mut ConCommandBase) -> Self {
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
	type Item = &'a mut ConCommandBase;
	fn next(&mut self) -> Option<Self::Item> {
		let current = self.current;
		if !current.is_null() {
			let result = unsafe { ConCommandBase::from_mut_ptr(current) };
			self.current = result.as_inner().data.next;
			Some(result)
		} else {
			None
		}
	}
}
