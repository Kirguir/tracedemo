use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use derive_more::Display;
use failure::Fail;
use futures::future::{self, Either, Future};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::media::{InitStreamError, MediaManager};
use crate::utils::error::JasonErr;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Room closed")]
    RoomClosed,
    #[display(fmt = "Media manager failed: {}", _0)]
    MediaManagerFailed(InitStreamError),
}

impl Fail for ApiError {
    #[inline]
    fn name(&self) -> Option<&str> {
        use ApiError::*;
        Some(match self {
            MediaManagerFailed(_) => "MEDIA_MANAGER_FAILED",
            RoomClosed => "ROOM_CLOSED",
        })
    }

    #[inline]
    fn cause(&self) -> Option<&dyn Fail> {
        use ApiError::*;
        match self {
            MediaManagerFailed(e) => Some(e),
            RoomClosed => None,
        }
    }
}

struct InnerRoom {
    media_manager: Rc<MediaManager>,
    stream: Option<web_sys::MediaStream>,
}

impl InnerRoom {
    /// Creates new [`InnerRoom`].
    #[inline]
    fn new(media_manager: Rc<MediaManager>) -> Self {
        Self {
            media_manager,
            stream: None,
        }
    }
}

pub struct Room(Rc<RefCell<InnerRoom>>);

impl Room {
    pub fn new(media_manager: Rc<MediaManager>) -> Self {
        Self(Rc::new(RefCell::new(InnerRoom::new(media_manager))))
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
    pub fn join(&self) -> Promise {
        let fut = match self.0.upgrade() {
            None => Either::A(future::err(ApiError::RoomClosed)),
            Some(inner) => {
                let room = Rc::clone(&inner);
                let fut = inner
                    .borrow()
                    .media_manager
                    .get_stream()
                    .map_err(|err| ApiError::MediaManagerFailed(err))
                    .and_then(move |stream| {
                        room.borrow_mut().stream.replace(stream);
                        Ok(())
                    });
                Either::B(fut)
            }
        }
        .map(|_| JsValue::NULL)
        .map_err(|err| JasonErr::from(err).into());
        future_to_promise(fut)
    }
}
