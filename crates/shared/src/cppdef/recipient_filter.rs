use ::core::ffi::c_int;
use ::rse_cpp::class;
use ::rse_utl::cppdef::CUtlVector;

class! {
	pub IRecipientFilter {
		pub virtual ~destructors();
		pub virtual fn is_reliable() -> bool;
		pub virtual fn is_init_message() -> bool;
		pub virtual fn get_recipient_count() -> c_int;
		pub virtual fn get_recipient_index(slot: c_int) -> c_int;
	}
}

class! {
	pub CRecipientFilter: pub virtual IRecipientFilter {
		pub reliable: bool;
		pub init_message: bool;
		pub recipients: CUtlVector<c_int>;
		pub using_prediction_rules: bool;
		pub ignore_prediction_cull: bool;
	}
}
