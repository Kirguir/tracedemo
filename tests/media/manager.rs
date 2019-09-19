#![cfg(target_arch = "wasm32")]
use futures::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

use tracedemo::media::MediaManager;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(module = "/tests/check_error.js")]
extern "C" {
    fn get_error_desc(val: &JsValue) -> String;
    fn get_error_name(val: &JsValue) -> String;
    fn get_error_cause(val: &JsValue) -> String;
}

#[wasm_bindgen_test(async)]
fn init_local_stream() -> impl Future<Item = (), Error = JsValue> {
    let media_manager = MediaManager::default();
    let mut constraints = web_sys::MediaStreamConstraints::new();
    constraints.audio(&JsValue::from_bool(true));
    constraints.video(&JsValue::from_bool(true));
    JsFuture::from(media_manager.new_handle().init_local_stream(constraints))
        .then(|res| {
            match res {
                Ok(_) => assert!(false, "should not be success"),
                Err(e) => {
                    assert_eq!(get_error_name(&e), "MEDIA_STREAM_FAILED");
                    assert_eq!(get_error_desc(&e), "Failed gets stream: NotFoundError: Requested device not found\nerror trace:\ntracedemo::media::manager\n  at src\\media\\manager.rs:41");
                    assert_eq!(get_error_cause(&e), "MEDIA_STREAM_FAILED");
                }
            };
            Ok(())
        })
}
