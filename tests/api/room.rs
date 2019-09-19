#![cfg(target_arch = "wasm32")]
use std::rc::Rc;

use futures::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

use tracedemo::api::room::Room;
use tracedemo::media::MediaManager;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(module = "/tests/check_error.js")]
extern "C" {
    fn get_error_desc(val: &JsValue) -> String;
    fn get_error_name(val: &JsValue) -> String;
    fn get_error_cause(val: &JsValue) -> String;
}

#[wasm_bindgen_test(async)]
fn join_room() -> impl Future<Item = (), Error = JsValue> {
    let media_manager = Rc::new(MediaManager::default());
    let mut constraints = web_sys::MediaStreamConstraints::new();
    constraints.audio(&JsValue::from_bool(true));
    constraints.video(&JsValue::from_bool(true));
    let room = Room::new(media_manager);
    JsFuture::from(room.new_handle().join())
        .then(|res| {
            match res {
                Ok(_) => assert!(false, "cannot be success"),
                Err(e) => {
                    assert_eq!(get_error_name(&e), "MEDIA_MANAGER_FAILED");
                    assert_eq!(get_error_desc(&e), "Media manager failed: Failed gets stream: NotFoundError: Requested device not found");
                    assert_eq!(get_error_cause(&e), "MEDIA_STREAM_FAILED: Failed gets stream: NotFoundError: Requested device not found");
                }
            };
            Ok(())
        })
}
