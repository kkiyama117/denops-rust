#![deny(unused_unsafe)]

use wasm_bindgen::prelude::*;
use crate::denops::Denops;
use crate::Vim;

#[wasm_bindgen(module = "https://deno.land/x/denops_std/mod.ts")]
extern {
    #[wasm_bindgen(catch)]
    pub async fn autocmd(denops: &Denops, group: &str, main: fn(AutocmdHelper)) -> Result<(), JsValue>;

    #[wasm_bindgen(method)]
    pub fn define(this: &AutocmdHelper, event: JsValue, pat: JsValue, cmd: &str, options: JsValue);

    #[wasm_bindgen(method)]
    pub fn remove(this: &AutocmdHelper, event: JsValue, pat: JsValue, cmd: &str, options: JsValue);
}