use ::atomic_wait::{
	wait, wake_one,
};
use ::core::{
	fmt,
	hint::spin_loop,
	sync::atomic::Ordering,
};

type AtomicState = ::core::sync::atomic::AtomicU32;
type RawState = u32;

#[derive(Debug)]
enum State {
	Unlocked,
	Locked,
	Contended,
	Undefined = -1,
}

const UNLOCKED: RawState = State::Unlocked as _;
// locked, no other threads waiting
const LOCKED: RawState = State::Locked as _;
// locked, and other threads waiting (contended)
const CONTENDED: RawState = State::Contended as _;

/// Fast user-space mutex.
#[repr(transparent)]
pub struct Futex {
	state: AtomicState,
}

impl Futex {
	pub const fn new() -> Self {
		Self {
			state: AtomicState::new(UNLOCKED),
		}
	}

	/// # Safety
	/// The data that this futex locked must not have an existing reference.
	pub unsafe fn unlock(&self) {
		if self.state.swap(UNLOCKED, Ordering::Release) == CONTENDED {
			// We only wake up one thread.
			// When that thread locks the mutex,
			// it will mark the mutex as `CONTENDED` (see `lock_contended`),
			// which makes sure that any other waiting threads will also be woken up eventually.
			self.wake_one();
		}
	}

	#[cold]
	fn wake_one(&self) {
		wake_one(&self.state)
	}

	pub fn try_lock(&self) -> bool {
        self.state.compare_exchange(
			UNLOCKED, LOCKED,
			Ordering::Acquire, Ordering::Relaxed,
		).is_ok()
	}

	pub fn lock(&self) {
		if self.state.compare_exchange(
			UNLOCKED, LOCKED,
			Ordering::Acquire, Ordering::Relaxed,
		).is_err() {
			self.lock_contended();
		}
	}

	#[cold]
	fn lock_contended(&self) {
		let mut state = self.spin();
		if state == UNLOCKED {
			match self.state.compare_exchange(
				UNLOCKED, LOCKED,
				Ordering::Acquire, Ordering::Relaxed,
			) {
				Ok(..) => return,
				Err(s) => state = s,
			}
		}

		loop {
			if state != CONTENDED
				&& self.state.swap(CONTENDED, Ordering::Acquire) == UNLOCKED
			{
				return
			}
			wait(&self.state, CONTENDED);
			state = self.spin();
		}
	}

	fn spin(&self) -> RawState {
		let mut tries = 100u8;
		loop {
			let state = self.state.load(Ordering::Relaxed);
			if state != LOCKED || tries == 0 {
				return state
			}
			spin_loop();
			tries -= 1;
		}
	}
}

impl fmt::Debug for Futex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let state = match self.state.load(Ordering::Relaxed) {
			UNLOCKED => State::Unlocked,
			LOCKED => State::Locked,
			CONTENDED => State::Contended,
			_ => State::Undefined,
		};

		f.debug_struct("Futex")
			.field("state", &state)
			.finish()
	}
}
