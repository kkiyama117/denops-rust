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

#[wasm_bindgen]
#[no_mangle]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: JsValue);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    pub fn log_error(s: JsValue);
}
