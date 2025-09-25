#![no_std]

pub mod cppdef;

pub mod memory;
pub mod vector;

pub use {
	memory::Memory,
	vector::Vector,
};
