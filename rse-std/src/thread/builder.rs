use ::core::{
	panic::AssertUnwindSafe,
	sync::atomic::{
		Ordering, AtomicBool,
	},
};
use ::rust_alloc::{
	boxed::Box,
	sync::Arc,
	vec::Vec,
};
use ::rse_tier0::{
	linked::LinkedTier0,
	ThreadData,
	uintp,
};
use ::std::{
	panic::catch_unwind,
	sync::{
		Mutex, MutexGuard,
	},
};

pub fn spawn<F>(f: F) -> Option<Thread>
where
	F: FnOnce() + Send + 'static,
{
	let f: Box<dyn FnOnce() + Send + 'static> = Box::new(f);
	Thread::spawn_boxed(Box::new(f))
}

static THREADS: Mutex<Vec<State>> = Mutex::new(Vec::new());
fn threads() -> MutexGuard<'static, Vec<State>> {
	THREADS.lock().unwrap_or_else(move |e| {
		THREADS.clear_poison();
		e.into_inner()
	})
}

static RUNNING: AtomicBool = AtomicBool::new(false);

/// Returns `true` if the plugin is currently running.
/// 
/// See the [module-level documentation](crate::thread) for more information.
pub fn running() -> bool {
	RUNNING.load(Ordering::SeqCst)
}

pub(crate) fn attach() {
	RUNNING.store(true, Ordering::SeqCst)
}

/// Join all threads that haven't been joined yet.
pub(crate) fn detach() {
	RUNNING.store(false, Ordering::SeqCst);
	let mut states = threads();
	for state in states.drain(..) {
		if let Some(raw) = state.take_raw() {
			raw.join();
		}
	}
}

#[derive(Default, Debug)]
pub struct Builder {
	stack_size: usize,
}

impl Builder {
	pub const fn new() -> Self {
		Self {
			stack_size: 0,
		}
	}

	pub const fn stack_size(&mut self, stack_size: usize) -> &mut Self {
		self.stack_size = stack_size;
		self
	}

	pub fn spawn_boxed<F>(&self, f: Box<F>) -> Option<Thread>
	where
		F: FnOnce() + Send + 'static,
	{
		extern "C" fn thread_func<F>(data: ThreadData<*mut F>) -> uintp
		where
			F: FnOnce() + Send + 'static,
		{
			let f = unsafe { Box::from_raw(data.get()) };
			let _ = catch_unwind(AssertUnwindSafe(f));
			0
		}

		let raw = RawThread::spawn(
			LinkedTier0,
			thread_func, ThreadData::new(Box::into_raw(f)),
			self.stack_size,
		)?;
		let state = State::new(raw);
		threads().push(state.clone());
		Some(Thread {
			state,
		})
	}
}

#[repr(transparent)]
pub struct Thread {
	state: State,
}

impl Thread {
	/// Spawn a new thread.
	pub fn spawn_boxed<F>(f: Box<F>) -> Option<Self>
	where
		F: FnOnce() + Send + 'static,
	{
		Builder::new().spawn_boxed(f)
	}

	/// Wait for this thread to finish,
	/// returning `false` if it failed for some reason.
	pub fn join(self) -> bool {
		if let Some(raw) = self.state.take_raw() {
			let result = raw.join();

			let ptr = self.state.as_ptr();
			let mut states = threads();
			let index = states.iter().position(move |thread| thread.as_ptr() == ptr);
			if let Some(index) = index {
				states.swap_remove(index);
			}
			result
		} else {
			true
		}
	}

	/// Detach this thread without waiting for it to finish
	/// if it doesn't terminate by the time the plugin is unloading.
	/// 
	/// # Safety
	/// 
	pub unsafe fn detach(self) {
		if let Some(raw) = self.state.take_raw() {
			unsafe { raw.detach() }
		}
	}
}

#[derive(Clone)]
#[repr(transparent)]
struct State(pub Arc<StateInner>);
impl State {
	pub fn new(raw: RawThread) -> Self {
		Self(Arc::new(StateInner {
			raw: Mutex::new(Some(raw)),
		}))
	}
	
	pub fn take_raw(&self) -> Option<RawThread> {
		self.0.raw.lock().unwrap_or_else(|e| e.into_inner()).take()
	}

	pub fn as_ptr(&self) -> *const StateInner {
		Arc::as_ptr(&self.0)
	}
}

struct StateInner {
	pub raw: Mutex<Option<RawThread>>,
}

type RawThread = ::rse_tier0::RawThread<LinkedTier0>;
