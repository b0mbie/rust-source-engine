use ::rse_game_interfaces::cvar::Cvar;
use ::std::{
	ops::{
		Deref, DerefMut,
	},
	sync::{
		RwLockWriteGuard, RwLockReadGuard,
	},
};

use super::{
	CVAR, POISON_EXPECT,
};

#[repr(transparent)]
pub struct CvarWrite {
	inner_some: RwLockWriteGuard<'static, Option<Cvar>>,
}

impl CvarWrite {
	pub fn acquire() -> Option<Self> {
		let inner = CVAR.write().expect(POISON_EXPECT);
		if inner.is_some() {
			Some(Self {
				inner_some: inner,
			})
		} else {
			None
		}
	}
}

impl Deref for CvarWrite {
	type Target = Cvar;
	fn deref(&self) -> &Self::Target {
		unsafe { self.inner_some.as_ref().unwrap_unchecked() }
	}
}

impl DerefMut for CvarWrite {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { self.inner_some.as_mut().unwrap_unchecked() }
	}
}

#[repr(transparent)]
pub struct CvarRead {
	inner_some: RwLockReadGuard<'static, Option<Cvar>>,
}

impl CvarRead {
	pub fn acquire() -> Option<Self> {
		let inner = CVAR.read().expect(POISON_EXPECT);
		if inner.is_some() {
			Some(Self {
				inner_some: inner,
			})
		} else {
			None
		}
	}
}

impl Deref for CvarRead {
	type Target = Cvar;
	fn deref(&self) -> &Self::Target {
		unsafe { self.inner_some.as_ref().unwrap_unchecked() }
	}
}
