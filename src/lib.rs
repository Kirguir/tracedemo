use wasm_bindgen::prelude::*;
use web_sys::{MediaStreamConstraints, MediaStream};
use crate::errors::{DetachedStateError, JasonError, MediaError};

mod errors;

// 1. Try-> Promise<MediaStream, JasonError> where JasonError in [DetachedStateError, MediaError]
// 2. Add caused to MediaError

#[wasm_bindgen]
fn get_media_manager(detached:bool) -> Result<MediaManager, DetachedStateError> {
    if detached {
        Err(DetachedStateError::new("DetachedStateError when accessing media_manager"))
    } else {
        Ok(MediaManager{})
    }
}

//struct MediaManager;
//
//impl MediaManager {
//    async fn get_stream() -> Result<MediaStream, MediaError>{
////        web_sys::window().unwrap()()
////            .navigator()
////            .media_devices()
////            .map_err(WasmErr::from)
////            .into_future()
////            .and_then(move |devices| {
////                let mut constraints = MediaStreamConstraints::new();
////                constraints.audio(JsValue::TRUE);
////                constraints.video(JsValue::TRUE);
////
////                JsFuture::from(                devices
////                    .get_user_media_with_constraints(&caps).unwrap())
////            })
////            .map(MediaStream::from)
//        Err(MediaError::new("asd"))
//    }
//}
//
//#[wasm_bindgen]
//impl MediaManager {
//
//}
