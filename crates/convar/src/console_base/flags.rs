use crate::cppdef::{
	fcvar::*,
	RawCvarFlags,
};

impl CvarFlags {
	pub const fn from_ref(flags: &RawCvarFlags) -> &Self {
		unsafe { &*(flags as *const _ as *const Self) }
	}

	pub const fn from_mut(flags: &mut RawCvarFlags) -> &mut Self {
		unsafe { &mut *(flags as *mut _ as *mut Self) }
	}

	pub const fn is_for_material_thread(&self) -> bool {
		self.contains(
			Self::RELOAD_MATERIALS
				.union(Self::RELOAD_TEXTURES)
				.union(Self::MATERIAL_SYSTEM_THREAD)
		)
	}
}

::rse_cpp::bitflags! {
	#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[repr(transparent)]
	pub struct CvarFlags: RawCvarFlags {
		const UNREGISTERED = UNREGISTERED;
		const DEVELOPMENT_ONLY = DEVELOPMENT_ONLY;
		const GAMEDLL = GAMEDLL;
		const CLIENTDLL = CLIENTDLL;
		const HIDDEN = HIDDEN;

		const PROTECTED = PROTECTED;
		const SP_ONLY = SP_ONLY;
		const ARCHIVE = ARCHIVE;
		const NOTIFY = NOTIFY;
		const USERINFO = USERINFO;
		const CHEAT = CHEAT;

		const PRINTABLE_ONLY = PRINTABLE_ONLY;
		const UNLOGGED = UNLOGGED;
		const NEVER_AS_STRING = NEVER_AS_STRING;

		const REPLICATED = REPLICATED;
		const DEMO = DEMO;
		const DONT_RECORD = DONT_RECORD;
		const RELOAD_MATERIALS = RELOAD_MATERIALS;
		const RELOAD_TEXTURES = RELOAD_TEXTURES;

		const NOT_CONNECTED = NOT_CONNECTED;
		const MATERIAL_SYSTEM_THREAD = MATERIAL_SYSTEM_THREAD;
		const ARCHIVE_XBOX = ARCHIVE_XBOX;

		const ACCESSIBLE_FROM_THREADS = ACCESSIBLE_FROM_THREADS;

		const SERVER_CAN_EXECUTE = SERVER_CAN_EXECUTE;
		const SERVER_CANNOT_QUERY = SERVER_CANNOT_QUERY;
		const CLIENTCMD_CAN_EXECUTE = CLIENTCMD_CAN_EXECUTE;

		const EXEC_DESPITE_DEFAULT = EXEC_DESPITE_DEFAULT;

		const INTERNAL_USE = INTERNAL_USE;
		const ALLOWED_IN_COMPETITIVE = ALLOWED_IN_COMPETITIVE;
	}
}
