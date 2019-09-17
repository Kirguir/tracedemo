#![cfg(target_arch = "wasm32")]
use futures::Future;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

use tracedemo::two::MediaManager;
use tracedemo::utils::error::JasonErr;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test(async)]
fn init_local_stream() -> impl Future<Item = (), Error = JsValue> {
    let media_manager = MediaManager::default();
    let mut constraints = web_sys::MediaStreamConstraints::new();
    constraints.audio(&JsValue::from_bool(true));
    constraints.video(&JsValue::from_bool(true));
    JsFuture::from(media_manager.new_handle().init_local_stream(constraints))
        .map(|stream| match stream.dyn_into::<web_sys::MediaStream>() {
            Ok(stream) => assert_eq!(stream.id().len(), 36),
            Err(_) => assert!(false),
        })
        .map_err(|e| {
            js_sys::Error::new("Failed init local stream").into()
            //JsValue::from_str("Failed init local stream")
        })
}
