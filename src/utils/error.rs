use derive_more::Display;
use failure::Fail;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Display)]
#[display(fmt = "{}", _0)]
pub struct JsErr(String);

impl From<JsValue> for JsErr {
    fn from(val: JsValue) -> Self {
        match val.dyn_into::<js_sys::Error>() {
            Ok(err) => Self(err.to_string().into()),
            Err(val) => Self(format!("{:?}", val)),
        }
    }
}

#[wasm_bindgen]
pub struct JasonErr {
    error: Box<dyn Fail>,
}

#[wasm_bindgen]
impl JasonErr {
    pub fn get_name(&self) -> String {
        (*self.error).name().map_or("Error".to_string(), Into::into)
    }

    pub fn description(&self) -> String {
        self.error.to_string()
    }

    pub fn cause(&self) -> String {
        let err = self.error.find_root_cause();
        format!("{}", err.name().unwrap_or("Error"))
    }

    pub fn backtrace(&self) -> String {
        unimplemented!()
    }
}

impl<T> From<T> for JasonErr
where
    T: Fail + 'static,
{
    fn from(err: T) -> Self {
        Self {
            error: Box::new(err),
        }
    }
}
