/// Type for a DLL identifier that's used to mark ConVars and ConCommands.
pub type CvarDllIdentifier = ::core::ffi::c_int;

mod con_command_base;
pub use con_command_base::*;
mod con_command;
pub use con_command::*;
mod con_var;
pub use con_var::*;
mod command;
pub use command::*;

pub mod flags;
