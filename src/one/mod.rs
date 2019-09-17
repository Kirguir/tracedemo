use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::prelude::*;

use self::room::RoomHandle;
use crate::one::room::Room;
use crate::two::MediaManager;

pub mod room;

#[wasm_bindgen]
#[derive(Default)]
pub struct Jason(Rc<RefCell<Inner>>);

#[derive(Default)]
struct Inner {
    media_manager: Rc<MediaManager>,
    rooms: Vec<Room>,
}

#[wasm_bindgen]
impl Jason {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init_room(&self) -> RoomHandle {
        let room = Room::new();

        let handle = room.new_handle();

        self.0.borrow_mut().rooms.push(room);

        handle
    }

    /// Drops [`Jason`] API object, so all related objects (rooms, connections,
    /// streams etc.) respectively. All objects related to this [`Jason`] API
    /// object will be detached (you will still hold them, but unable to use).
    pub fn dispose(self) {}
}
