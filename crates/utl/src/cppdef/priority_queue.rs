use ::rse_cpp::class;

use super::{
	CUtlMemory, LessFunc_t,
};

class! {
	#[derive(Debug)]
	pub CUtlPriorityQueue<T> {
		pub heap: CUtlMemory<T>;
		pub less_func: LessFunc_t<T>;
	}
}
