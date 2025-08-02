use ::rse_cpp::{
	VtObjectMut, new_vtable_self, this_to_self,
};

use crate::{
	cppdef::{
		GameEventListener2Vt, GameEventVt,
	},
	GameEvent,
};

pub trait EventListener {
	type Event: for<'a> TryFrom<GameEvent<'a>>;
	fn on_event(&mut self, event: Self::Event);
}
impl<T: EventListener> RawEventListener for T {
	fn fire_game_event(&mut self, event: GameEvent<'_>) {
		if let Ok(event) = T::Event::try_from(event) {
			self.on_event(event)
		}
	}
}

pub trait RawEventListener {
	fn fire_game_event(&mut self, event: GameEvent<'_>);
}

#[repr(C)]
pub struct EventListenerObject<T> {
	vtable: *mut GameEventListener2Vt,
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
			vtable: Self::VTABLE as *const _ as *mut _,
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
		#[cfg(not(target_os = "windows"))]
		destructor_2,
		fire_game_event
	});

	::rse_cpp::vtable_methods! {
		this: VtObjectMut<GameEventListener2Vt>;
		fn destructor() {
			unsafe { this.cast::<Self>().drop_in_place() }
		}
		#[cfg(not(target_os = "windows"))]
		fn destructor_2() {
			unsafe { this.cast::<Self>().drop_in_place() }
		}
		fn fire_game_event(event: VtObjectMut<GameEventVt>) {
			let event = unsafe { GameEvent::from_ptr(event) };
			this_to_self!(mut this).inner.fire_game_event(event)
		}
	}
}
