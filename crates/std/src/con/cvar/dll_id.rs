use ::atomic::*;
use ::rse_convar::console_base::CvarDllIdentifier;

pub const FIRST_INIT_DLL_ID: CvarDllIdentifier = 0;
pub const UNINIT_DLL_ID: CvarDllIdentifier = FIRST_INIT_DLL_ID - 1;

static DLL_IDENTIFIER: Atomic<CvarDllIdentifier> = Atomic::new(UNINIT_DLL_ID);

pub fn dll_identifier() -> CvarDllIdentifier {
	DLL_IDENTIFIER.load(Ordering::Relaxed)
}

pub fn reset_dll_identifier() {
	unsafe { set_dll_identifier(UNINIT_DLL_ID) }
}

/// # Safety
/// `dll_identifier` must be a valid identifier previously returned by
/// `ICvar::AllocateDLLIdentifier`.
pub unsafe fn set_dll_identifier(dll_id: CvarDllIdentifier) {
	DLL_IDENTIFIER.store(dll_id, Ordering::Relaxed);
}
