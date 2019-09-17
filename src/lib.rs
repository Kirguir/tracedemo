pub mod one;
pub mod two;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::two::{InitStreamError, MediaManager};
    use crate::utils::error::JasonErr;
    use wasm_bindgen::JsValue;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn error() {
        let err = InitStreamError::UserMediaFailed(JsValue::null());
        let err: JasonErr = err.into();
        let manager = MediaManager::default();
    }
}
