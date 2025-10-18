use ::rse_game_interfaces::cvar::Cvar;
use ::std::sync::{
	RwLock, RwLockReadGuard, RwLockWriteGuard,
};

static CVAR: RwLock<Option<Cvar>> = RwLock::new(None);
const POISON_EXPECT: &str = "Cvar interface lock shouldn't be poisoned";
pub fn cvar_read() -> RwLockReadGuard<'static, Option<Cvar>> {
	CVAR.read().expect(POISON_EXPECT)
}
pub fn cvar_write() -> RwLockWriteGuard<'static, Option<Cvar>> {
	CVAR.write().expect(POISON_EXPECT)
}

mod dll_id;
pub use dll_id::*;