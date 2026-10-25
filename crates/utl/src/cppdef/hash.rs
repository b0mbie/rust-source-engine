use ::core::ffi::c_uint;
use ::rse_cpp::RefConst;

use super::CUtlVector;

::rse_cpp::class! {
	#[derive(Debug)]
	pub CUtlHash<Data> {
		pub buckets: CUtlVector<HashBucketList_t<Data>>;
		pub compare_func: CompareFunc_t<Data>;
		pub key_func: KeyFunc_t<Data>;
		pub power_of_two: bool;
		pub mod_mask: c_uint;
	}
}

#[allow(non_camel_case_types)]
pub type HashBucketList_t<Data> = CUtlVector<Data>;

#[allow(non_camel_case_types)]
pub type CompareFunc_t<Data> = unsafe extern "C" fn(RefConst<Data>, RefConst<Data>) -> bool;

#[allow(non_camel_case_types)]
pub type KeyFunc_t<Data> = unsafe extern "C" fn(RefConst<Data>) -> c_uint;
