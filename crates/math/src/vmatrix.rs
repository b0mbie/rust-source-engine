use super::vec_t;

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(C)]
pub struct VMatrix {
	pub m: [[vec_t; 4]; 4],
}
