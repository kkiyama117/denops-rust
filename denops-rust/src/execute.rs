#![deny(unused_unsafe)]

use crate::denops::Denops;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "https://deno.land/x/denops_std@v0.3/mod.ts")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub async fn execute(
        denops: &Denops,
        command: JsValue,
        context: JsValue,
    ) -> Result<(), JsValue>;
}
