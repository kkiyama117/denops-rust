#![deny(unused_unsafe)]

use wasm_bindgen::prelude::*;
use crate::denops::Denops;

#[wasm_bindgen(module = "https://deno.land/x/denops_std@v0.3/mod.ts")]
extern {
    #[wasm_bindgen(catch)]
    pub async fn execute(denops: &Denops, command: JsValue, context: JsValue) -> Result<(), JsValue>;
}