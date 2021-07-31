use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
         $crate::console::log(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! console_error {
    ($($arg:tt)*) => {
         $crate::console::log_error(&format!($($arg)*))
    };
}

fn to_js_value(s: impl ToString) -> JsValue {
    JsValue::from_str(s.to_string().as_str())
}

pub fn log(s: impl ToString) {
    _log(to_js_value(s))
}

pub fn log_error(s: impl ToString) {
    _log_error(to_js_value(s))
}

pub fn log_raw(s: JsValue) {
    _log(s)
}

pub fn log_error_raw(s: JsValue) {
    _log_error(s)
}

#[wasm_bindgen]
#[no_mangle]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn _log(s: JsValue);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn _log_error(s: JsValue);
}
