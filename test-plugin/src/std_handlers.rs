use ::rse_tier0::{
	linked::mem::LinkedTier0Allocator,
	mem_alloc::Tier0GlobalAlloc,
};

#[global_allocator]
static ALLOCATOR: Tier0GlobalAlloc<LinkedTier0Allocator> = Tier0GlobalAlloc(LinkedTier0Allocator);
