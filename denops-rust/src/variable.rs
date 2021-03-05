#![deny(unused_unsafe)]

use wasm_bindgen::prelude::*;
use crate::Vim;

#[wasm_bindgen(module = "https://deno.land/x/denops_std/mod.ts")]
extern {
    #[wasm_bindgen(method, getter)]
    pub fn g(this: &Vim) -> VariableHelper;
    #[wasm_bindgen(method, getter)]
    pub fn b(this: &Vim) -> VariableHelper;
    #[wasm_bindgen(method, getter)]
    pub fn w(this: &Vim) -> VariableHelper;
    #[wasm_bindgen(method, getter)]
    pub fn t(this: &Vim) -> VariableHelper;
    #[wasm_bindgen(method, getter)]
    pub fn v(this: &Vim) -> VariableHelper;
    #[wasm_bindgen(method, getter)]
    pub fn env(this: &Vim) -> VariableHelper;

    // https://deno.land/x/denops_std@v0.3/vim/variable.ts
    pub type VariableHelper;
    #[wasm_bindgen(method, catch)]
    pub async fn get(this: &VariableHelper, prop: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(method, catch)]
    pub async fn set(this: &VariableHelper, prop: &str,value:JsValue) -> Result<(), JsValue>;
    #[wasm_bindgen(method, catch)]
    pub async fn remove(this: &VariableHelper, prop: &str) -> Result<(), JsValue>;
}