use js_sys::{Error, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/error.js")]
extern "C" {
    #[wasm_bindgen(extends = Object, extends = Error)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type JasonError;

    #[wasm_bindgen(constructor)]
    pub fn new(message: &str) -> JasonError;
}

#[wasm_bindgen(module = "/src/error.js")]
extern "C" {
    #[wasm_bindgen(extends = Object, extends = JasonError)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type DetachedStateError;

    #[wasm_bindgen(constructor)]
    pub fn new(message: &str) -> DetachedStateError;
}

#[wasm_bindgen(module = "/src/error.js")]
extern "C" {
    #[wasm_bindgen(extends = Object, extends = JasonError)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type MediaError;

    #[wasm_bindgen(constructor)]
    pub fn new(message: &str) -> MediaError;
}
