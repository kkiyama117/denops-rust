// extern WASM calls are wrapped in unsafe,
// but they don't technically have to be.
#![deny(unused_unsafe)]

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(module = "https://deno.land/x/denops_std@v0.3/mod.ts")]
extern {
    pub type Vim;
    pub type VariableHelper;
    // static vim: Vim;

    #[wasm_bindgen(method, getter)]
    pub fn g(this: &Vim) -> VariableHelper;

    #[wasm_bindgen(method, getter, catch)]
    pub async fn get(this: &VariableHelper, prop: &str) -> Result<JsValue, JsValue>;
}