#![no_std]

pub mod cppdef;

mod util;

pub mod console_base;
pub mod command;
pub mod variable;

mod invocation;
pub use invocation::*;
