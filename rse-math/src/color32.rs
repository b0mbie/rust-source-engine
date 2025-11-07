/// 32-bit color type.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Color32 {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}
