use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use derive_more::Display;
use failure::Fail;
use futures::prelude::*;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};

use crate::utils::error::{JasonErr, JsErr};
use crate::utils::window;
use tracerr::Traced;

#[derive(Default)]
struct InnerMediaManager {}

impl InnerMediaManager {
    fn get_local_stream(
        &self,
        constraints: web_sys::MediaStreamConstraints,
    ) -> impl Future<Item = web_sys::MediaStream, Error = Traced<InitStreamError>> {
        use InitStreamError::*;
        window()
            .navigator()
            .media_devices()
            .map_err(|err| MediaDevicesFailed(err.into()))
            .map_err(tracerr::wrap!())
            .into_future()
            .and_then(move |devices| {
                devices
                    .get_user_media_with_constraints(&constraints)
                    .map_err(|err| UserMediaFailed(err.into()))
                    .map_err(tracerr::wrap!())
            })
            .and_then(|promise: js_sys::Promise| {
                JsFuture::from(promise)
                    .map_err(|err| MediaStreamFailed(err.into()))
                    .map_err(tracerr::wrap!())
            })
            .map(web_sys::MediaStream::from)
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct MediaManager(Rc<RefCell<InnerMediaManager>>);

impl MediaManager {
    pub fn new() -> Self {
        Self (Rc::new(RefCell::new(Default::default())))
    }
}

impl MediaManager {
    pub fn get_stream(
        &self,
    ) -> impl Future<Item = web_sys::MediaStream, Error = Traced<InitStreamError>> {
        let mut constraints = web_sys::MediaStreamConstraints::new();
        constraints.audio(&JsValue::from_bool(true));
        constraints.video(&JsValue::from_bool(true));
        self.0
            .borrow()
            .get_local_stream(constraints)
            .map_err(tracerr::wrap!())
    }

    /// Instantiates new [`MediaManagerHandle`] for use on JS side.
    #[inline]
    pub fn new_handle(&self) -> MediaManagerHandle {
        MediaManagerHandle(Rc::downgrade(&self.0))
    }
}

#[derive(Debug, Display)]
pub enum InitStreamError {
    #[display(fmt = "Failed init media devices: {}", _0)]
    MediaDevicesFailed(JsErr),
    #[display(fmt = "Failed gets user media: {}", _0)]
    UserMediaFailed(JsErr),
    #[display(fmt = "Failed gets stream: {}", _0)]
    MediaStreamFailed(JsErr),
    #[display(fmt = "Detached state")]
    DetachedState,
}

impl Fail for InitStreamError {
    #[inline]
    fn name(&self) -> Option<&str> {
        use InitStreamError::*;
        Some(match self {
            MediaDevicesFailed(_) => "MEDIA_DEVICES_FAILED",
            UserMediaFailed(_) => "USER_MEDIA_FAILED",
            MediaStreamFailed(_) => "MEDIA_STREAM_FAILED",
            DetachedState => "DETACHED_STATE",
        })
    }
}

#[wasm_bindgen]
pub struct MediaManagerHandle(Weak<RefCell<InnerMediaManager>>);

#[wasm_bindgen]
impl MediaManagerHandle {
    /// Returns [`SysMediaStream`] object.
    pub fn init_local_stream(&self, constraints: web_sys::MediaStreamConstraints) -> Promise {
        let fut = self
            .0
            .upgrade()
            .ok_or_else(|| InitStreamError::DetachedState)
            .map_err(tracerr::wrap!())
            .into_future()
            .and_then(move |inner| inner.borrow().get_local_stream(constraints))
            .map(Into::into)
            .map_err(|err| JasonErr::from(err).into());
        future_to_promise(fut)
    }
}
