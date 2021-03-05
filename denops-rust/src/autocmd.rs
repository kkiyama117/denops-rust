#![deny(unused_unsafe)]

use wasm_bindgen::prelude::*;
use crate::denops::Denops;

#[wasm_bindgen(module = "https://deno.land/x/denops_std/mod.ts")]
extern {
    #[wasm_bindgen(catch)]
    pub async fn autocmd(denops: &Denops, group: &str, main: fn(AutocmdHelper)) -> Result<(), JsValue>;

    // https://deno.land/x/denops_std/vim/autocmd.ts
    pub type AutocmdHelper;
    #[wasm_bindgen(method)]
    pub fn define(this: &AutocmdHelper, event: JsValue, pat: JsValue, cmd: &str, options: JsValue);

    #[wasm_bindgen(method)]
    pub fn remove(this: &AutocmdHelper, event: JsValue, pat: JsValue, cmd: &str, options: JsValue);
}