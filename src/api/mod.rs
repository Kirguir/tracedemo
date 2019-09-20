pub mod room;

use wasm_bindgen::prelude::*;
use crate::api::room::{Room, RoomHandle};
use crate::media::{MediaManager};
use std::rc::Rc;

#[wasm_bindgen]
pub struct Jason {
    room: Room
}

#[wasm_bindgen]
impl Jason {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {

        Self {
            room: Room::new(Rc::new(MediaManager::new()))
        }
    }

    pub fn dispose(self) {

    }

    pub fn room_handle(&self) -> RoomHandle {
        self.room.new_handle()
    }
}
