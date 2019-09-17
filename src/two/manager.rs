use derive_more::Display;
use futures::prelude::*;
use js_sys::Promise;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, JsFuture};

use crate::utils::error::{JasonErr, WasmErr};
use crate::utils::window;

#[derive(Default)]
struct InnerMediaManager {}

impl InnerMediaManager {
    fn get_local_stream(
        &self,
        constraints: web_sys::MediaStreamConstraints,
    ) -> impl Future<Item = web_sys::MediaStream, Error = InitStreamError> {
        window()
            .navigator()
            .media_devices()
            .map_err(|err| InitStreamError::MediaDevicesFailed(err))
            .into_future()
            .and_then(move |devices| {
                devices
                    .get_user_media_with_constraints(&constraints)
                    .map_err(|err| InitStreamError::UserMediaFailed(err))
            })
            .and_then(|promise: js_sys::Promise| {
                JsFuture::from(promise).map_err(|err| InitStreamError::MediaStreamFailed(err))
            })
            .map(web_sys::MediaStream::from)
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct MediaManager(Rc<RefCell<InnerMediaManager>>);

impl MediaManager {
    /// Instantiates new [`MediaManagerHandle`] for use on JS side.
    #[inline]
    pub fn new_handle(&self) -> MediaManagerHandle {
        MediaManagerHandle(Rc::downgrade(&self.0))
    }
}

#[derive(Debug, Display)]
pub enum InitStreamError {
    #[display(fmt = "Failed init media devices")]
    MediaDevicesFailed(JsValue),
    #[display(fmt = "Failed gets user media")]
    UserMediaFailed(JsValue),
    #[display(fmt = "Failed gets stream")]
    MediaStreamFailed(JsValue),
    #[display(fmt = "Detached state")]
    DetachedState,
}

impl WasmErr for InitStreamError {
    fn name(&self) -> String {
        match self {
            InitStreamError::MediaDevicesFailed(_) => "MEDIA_DEVICES_FAILED".to_string(),
            InitStreamError::UserMediaFailed(_) => "USER_MEDIA_FAILED".to_string(),
            InitStreamError::MediaStreamFailed(_) => "MEDIA_DEVICES_FAILED".to_string(),
            InitStreamError::DetachedState => "DETACHED_STATE".to_string(),
        }
    }

    fn js_error(&self) -> JsValue {
        match &self {
            InitStreamError::MediaDevicesFailed(val) => val.clone(),
            InitStreamError::UserMediaFailed(val) => val.clone(),
            InitStreamError::MediaStreamFailed(val) => val.clone(),
            InitStreamError::DetachedState => JsValue::null(),
        }
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
            .into_future()
            .and_then(move |inner| inner.borrow().get_local_stream(constraints))
            .map(Into::into)
            .map_err(|err| JasonErr::from(err).into());
        future_to_promise(fut)
    }
}
