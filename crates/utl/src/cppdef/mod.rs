use ::rse_cpp::RefConst;

mod dictionary;
pub use dictionary::*;
mod map;
pub use map::*;
mod memory;
pub use memory::*;
mod priority_queue;
pub use priority_queue::*;
mod rbtree;
pub use rbtree::*;
mod string;
pub use string::*;
mod symbol;
pub use symbol::*;
mod vector;
pub use vector::*;

#[allow(non_camel_case_types)]
pub type LessFunc_t<T> = unsafe extern "C" fn(a: RefConst<T>, b: RefConst<T>) -> bool;
