//! Source Engine game thread management.
//! 
//! # Plugin thread lifecycle
//! A plugin can spawn threads, and then either
//! call [`Thread::join`] to wait for them to finish, or
//! drop them, letting them run freely in a *detached* state.
//! 
//! Detached threads that have not finished
//! by the time the plugin is unloading
//! are `join`ed to wait until they finish.
//! 
//! A value of `false` signals to all remaining threads that they should
//! start shutting down, if they're still running.
//! Threads that do not cooperate will forever hang the program,
//! as the plugin will wait for them to finish first.

use ::core::time::Duration;
use ::rse_tier0::{
	linked::LinkedTier0,
	Tier0Thread,
};

mod binding;
pub use binding::*;
mod builder;
pub use builder::*;

/// Pauses execution on the current thread for the specified [`Duration`].
pub fn sleep(duration: Duration) {
	LinkedTier0.sleep(duration.as_millis() as _)
}
