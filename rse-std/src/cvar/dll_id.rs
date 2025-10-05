use ::core::sync::atomic::Ordering;
use ::rse_convar::console_base::CvarDllIdentifier;

use crate::atomic::Atomic;

static DLL_IDENTIFIER: Atomic<CvarDllIdentifier> = Atomic::<CvarDllIdentifier>::new(-1);

pub fn dll_identifier() -> CvarDllIdentifier {
	DLL_IDENTIFIER.load(Ordering::Relaxed)
}

pub unsafe fn set_dll_identifier(dll_identifier: CvarDllIdentifier) {
	DLL_IDENTIFIER.store(dll_identifier, Ordering::Relaxed);
}
