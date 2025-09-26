use ::core::ptr::NonNull;
use ::rse_cpp::{
	VtObjectPtr, VtObjectMut, new_vtable_self, this_to_self,
	VtObject,
};

use crate::{
	cppdef::{
		GameEventListener2Vt, GameEventVt,
	},
	GameEvent,
};

pub trait EventListener {
	type Event: for<'a> TryFrom<&'a GameEvent>;
	fn on_event(&mut self, event: Self::Event);
}
impl<T: EventListener> RawEventListener for T {
	fn fire_game_event(&mut self, event: &GameEvent) {
		if let Ok(event) = T::Event::try_from(event) {
			self.on_event(event)
		}
	}
}

pub trait RawEventListener {
	fn fire_game_event(&mut self, event: &GameEvent);
}

#[repr(C)]
pub struct EventListenerObject<T> {
	vtable: NonNull<GameEventListener2Vt>,
	inner: T,
}

impl<T> Default for EventListenerObject<T>
where
	T: RawEventListener + Default,
{
	fn default() -> Self {
		Self::new(T::default())
	}
}

impl<T> EventListenerObject<T>
where
	T: RawEventListener,
{
	pub const fn new(inner: T) -> Self {
		Self {
			vtable: unsafe { NonNull::new_unchecked(Self::VTABLE as *const _ as *mut _) },
			inner,
		}
	}

	pub const fn as_inner(&self) -> &T {
		&self.inner
	}

	pub const fn as_inner_mut(&mut self) -> &mut T {
		&mut self.inner
	}

	const VTABLE: &GameEventListener2Vt = &new_vtable_self!(GameEventListener2Vt {
		destructor,
		#[cfg(not(windows))]
		destructor_2,
		fire_game_event
	});

	::rse_cpp::vtable_methods! {
		this: VtObjectPtr<GameEventListener2Vt>;
		fn destructor() {
			unsafe { this.cast::<Self>().drop_in_place() }
		}
		#[cfg(not(windows))]
		fn destructor_2() {
			unsafe { this.cast::<Self>().drop_in_place() }
		}
		fn fire_game_event(event: VtObjectMut<GameEventVt>) {
			let event = unsafe { VtObject::from_ptr_const(event) };
			this_to_self!(mut this).inner.fire_game_event(event.into())
		}
	}
}
