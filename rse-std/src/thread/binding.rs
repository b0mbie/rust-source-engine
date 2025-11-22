use ::rse_tier0::{
	linked::LinkedTier0,
	Tier0Thread,
};
use ::thread_bound::{
	ThreadBound,
	ThreadBinding, InitThreadBinding,
};

/// Container for a `T` that can only be accessed on the main engine thread.
pub type MainThreadBound<T> = ThreadBound<T, MainThread>;

/// Returns `true` if the current thread is the main engine thread.
pub fn on_main_thread() -> bool {
	LinkedTier0.in_main_thread()
}

pub struct MainThread;
unsafe impl ThreadBinding for MainThread {
	fn is_current(&self) -> bool {
		on_main_thread()
	}
}
impl InitThreadBinding for MainThread {
	const INIT: Self = Self;
}
