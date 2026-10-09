use super::vec_t;

::rse_cpp::class! {
	#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
	pub VMatrix {
		pub m: [[vec_t; 4]; 4];
	}
}
