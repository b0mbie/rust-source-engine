#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SteamId {
	pub bits: u64,
}

impl SteamId {
	const ACCOUNT_ID_BITS: u32 = 32;
	const INSTANCE_ID_BITS: u32 = 20;
	const ACCOUNT_TYPE_BITS: u32 = 4;
	const UNIVERSE_BITS: u32 = 8;

	const fn u64_mask(bits: u32) -> u64 {
		u64::MAX >> (u64::BITS - bits)
	}

	pub const fn account_id(&self) -> u32 {
		(self.bits & Self::u64_mask(Self::ACCOUNT_ID_BITS)) as _
	}

	pub const fn instance_id(&self) -> u32 {
		((self.bits >> Self::ACCOUNT_ID_BITS) & Self::u64_mask(Self::INSTANCE_ID_BITS)) as _
	}

	pub const fn account_type(&self) -> u8 {
		((self.bits >> (Self::ACCOUNT_ID_BITS + Self::INSTANCE_ID_BITS)) & Self::u64_mask(Self::ACCOUNT_TYPE_BITS)) as _
	}

	pub const fn universe(&self) -> u8 {
		(
			(self.bits >> (Self::ACCOUNT_ID_BITS + Self::INSTANCE_ID_BITS + Self::ACCOUNT_TYPE_BITS)) &
			Self::u64_mask(Self::UNIVERSE_BITS)
		) as _
	}
}
