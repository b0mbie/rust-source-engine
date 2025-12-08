#![no_std]

pub mod cppdef;

pub mod console_base;
pub mod command;
pub mod variable;
pub mod cvar;

mod wrappers;
pub use wrappers::*;
