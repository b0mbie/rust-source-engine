pub use ::rse_convar::{
	console_base::CvarFlags,
	Registrable as Registered, ConVar as Variable, ConCommand as Command,
};

#[cfg(feature = "cvar-autoregister")]
mod autoregister;
#[cfg(feature = "cvar-autoregister")]
pub use autoregister::*;

pub mod cvar;
pub mod cmd;
pub mod var;