pub use ::rse_convar::{
	cppdef::RawCvarFlags,
	console_base::CvarFlags,
	Registrable as Registered, ConVar as Variable, ConCommand as Command,
};

#[cfg(feature = "cvar-autoregister")]
mod autoregister;
#[cfg(feature = "cvar-autoregister")]
pub use autoregister::*;

pub mod cvar;
pub mod cmd;
pub mod flag;
pub mod var;