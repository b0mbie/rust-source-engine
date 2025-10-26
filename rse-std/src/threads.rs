//! Source Engine game thread management.

use ::std::{
	cell::{
		RefCell, Ref, RefMut,
	},
	ops::{
		Deref, DerefMut,
	},
	sync::OnceLock,
	thread::{
		current, ThreadId,
	},
};

macro_rules! thread_binding {
	{
		$(#[$attr:meta])*
		$vis:vis struct $name:ident;
		$(#[$binding_attr:meta])*
		$binding_vis:vis static $binding_name:ident;
	} => {
		$(#[$attr])*
		$vis struct $name;
		impl ThreadBindingProvider for $name {
			const INIT: Self = Self;
			fn is_current(&self) -> bool {
				$binding_name.is_current()
			}
		}

		$(#[$binding_attr])*
		$binding_vis static $binding_name: ThreadBinding = ThreadBinding::new();
	};
}

pub type MainThreadBound<T> = ThreadBound<T, MainThread>;
thread_binding! {
	pub struct MainThread;
	pub(crate) static MAIN_THREAD;
}

pub(crate) static MATERIAL_THREAD: ThreadBinding = ThreadBinding::new();

pub struct ThreadBound<T, Thread> {
	binding: Thread,
	value: RefCell<T>,
}

unsafe impl<T, Thread: Sync> Sync for ThreadBound<T, Thread> {}

impl<T, Thread> ThreadBound<T, Thread>
where
	Thread: ThreadBindingProvider,
{
	pub const fn new(value: T) -> Self {
		Self {
			binding: Thread::INIT,
			value: RefCell::new(value),
		}
	}

	pub fn can_be_accessed(&self) -> bool {
		self.binding.is_current()
	}

	pub unsafe fn read_unchecked(&self) -> ThreadBoundRead<'_, T> {
		unsafe {
			let inner = self.value.try_borrow().unwrap_unchecked();
			ThreadBoundRead { inner, }
		}
	}

	pub unsafe fn write_unchecked(&self) -> ThreadBoundWrite<'_, T> {
		unsafe {
			let inner = self.value.try_borrow_mut().unwrap_unchecked();
			ThreadBoundWrite { inner, }
		}
	}

	pub fn read(&self) -> Option<ThreadBoundRead<'_, T>> {
		if self.can_be_accessed() {
			self.value.try_borrow().ok().map(move |inner| ThreadBoundRead { inner, })
		} else {
			None
		}
	}

	pub fn write(&self) -> Option<ThreadBoundWrite<'_, T>> {
		if self.can_be_accessed() {
			self.value.try_borrow_mut().ok().map(move |inner| ThreadBoundWrite { inner, })
		} else {
			None
		}
	}
}#[repr(transparent)]
pub struct ThreadBoundRead<'a, T: ?Sized> {
	inner: Ref<'a, T>,
}
impl<T: ?Sized> Deref for ThreadBoundRead<'_, T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		self.inner.deref()
	}
}

#[repr(transparent)]
pub struct ThreadBoundWrite<'a, T: ?Sized> {
	inner: RefMut<'a, T>,
}
impl<T: ?Sized> Deref for ThreadBoundWrite<'_, T> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		self.inner.deref()
	}
}
impl<T: ?Sized> DerefMut for ThreadBoundWrite<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.inner.deref_mut()
	}
}

pub trait ThreadBindingProvider {
	const INIT: Self;
	fn is_current(&self) -> bool;
}

#[repr(transparent)]
pub struct ThreadBinding {
	id: OnceLock<ThreadId>,
}

impl ThreadBinding {
	pub const fn new() -> Self {
		Self {
			id: OnceLock::new(),
		}
	}

	pub fn init_is_current(&self) -> bool {
		let current = current_id();
		*self.id.get_or_init(move || current) == current
	}

	pub fn bind_to_current(&self) {
		let _ = self.id.set(current_id());
	}

	pub fn is_current(&self) -> bool {
		self.id.get().copied() == Some(current_id())
	}

	pub fn try_run<F: FnOnce() -> R, R>(&self, f: F) -> Option<R> {
		if self.is_current() {
			Some(f())
		} else {
			None
		}
	}
}

fn current_id() -> ThreadId {
	current().id()
}
