use std::fmt::Display;

use wasm_bindgen::prelude::*;

pub trait WasmErr: Display {
    fn name(&self) -> String;

    fn js_error(&self) -> JsValue {
        JsValue::null()
    }
}

#[wasm_bindgen]
pub struct JasonErr {
    error: Box<dyn WasmErr>, //    name: String,
                             //    description: String,
                             //    cause: Option<JsValue>,
                             //    trace: String
}

#[wasm_bindgen]
impl JasonErr {
    pub fn name(&self) -> String {
        self.error.name()
    }

    pub fn description(&self) -> String {
        self.error.to_string()
    }

    pub fn cause(&self) -> JsValue {
        //        match &self.cause {
        //            Some(val) => val.clone(),
        //            None() => &JsValue::null(),
        //        }
        self.error.js_error()
    }

    pub fn backtrace(&self) -> String {
        unimplemented!()
    }
}

impl<T> From<T> for JasonErr
where
    T: WasmErr + 'static,
{
    fn from(err: T) -> Self {
        Self {
            error: Box::new(err),
        }
    }
}
