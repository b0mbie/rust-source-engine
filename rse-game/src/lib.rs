#![no_std]

pub mod cppdef;

pub use cppdef::convar::Command;

mod server_edict;
pub use server_edict::*;
