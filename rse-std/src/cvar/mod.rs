use ::rse_game_interfaces::{
	cvar::{
		Cvar, CvarImpl,
	},
	InterfaceFactories,
};
use ::rse_interface::InterfaceError;
use ::std::sync::RwLock;

mod dll_id;
pub use dll_id::*;
mod lock;
pub use lock::*;

static CVAR: RwLock<Option<Cvar>> = RwLock::new(None);
const POISON_EXPECT: &str = "Cvar interface lock shouldn't be poisoned";

pub fn init(factories: InterfaceFactories<'_>) -> Result<CvarInit, InitError> {
	let mut inner_cvar = CVAR.write().expect(POISON_EXPECT);
	if inner_cvar.is_none() {
		let mut cvar = factories.create_interface::<Cvar>()?;
		unsafe { set_dll_identifier(cvar.allocate_dll_identifier()) };
		*inner_cvar = Some(cvar);
	}
	unsafe { Ok(CvarInit::forge()) }
}

pub type InitError = InterfaceError<Cvar>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
#[non_exhaustive]
pub struct CvarInit;
impl CvarInit {
	/// # Safety
	/// The [`Cvar`] interface should've been initialized beforehand with [`init`].
	pub const unsafe fn forge() -> Self {
		Self
	}

	pub fn read(self) -> CvarRead {
		unsafe { CvarRead::acquire().unwrap_unchecked() }
	}

	pub fn write(self) -> CvarWrite {
		unsafe { CvarWrite::acquire().unwrap_unchecked() }
	}
}
