use js_sys::{Error, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/error.js")]
extern "C" {
    #[wasm_bindgen(extends = Object, extends = Error)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    type MyError;

    #[wasm_bindgen(constructor)]
    fn new(message: &str) -> MyError;
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Foo;

#[wasm_bindgen]
impl Foo {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dispose(self) {}

    pub fn run(&self) -> Result<(), JsValue> {
        Err(MyError::new("run service error").into())
    }
}
