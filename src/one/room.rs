use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use derive_more::Display;
use wasm_bindgen::prelude::*;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Failed init media devices")]
    MediaDevicesFailed(JsValue),
    #[display(fmt = "Failed gets user media")]
    UserMediaFailed(JsValue),
}

#[derive(Default)]
struct InnerRoom {}

pub struct Room(Rc<RefCell<InnerRoom>>);

impl Room {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(InnerRoom::default())))
    }

    #[inline]
    pub fn new_handle(&self) -> RoomHandle {
        RoomHandle(Rc::downgrade(&self.0))
    }
}

#[allow(clippy::module_name_repetitions)]
#[wasm_bindgen]
pub struct RoomHandle(Weak<RefCell<InnerRoom>>);

#[wasm_bindgen]
impl RoomHandle {
    /// Mutes outbound audio in this room.
    pub fn mute_audio(&self) -> Result<(), JsValue> {
        unimplemented!()
        //map_weak!(self, |inner| inner.borrow_mut().toggle_send_audio(false))
    }
}
