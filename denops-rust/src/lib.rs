/*!

# Denops Rust library

## About

Read this [article](https://zenn.dev/kkiyama117/articles/2021-03-01-denops-rust) also.

## Todo

- [ ] Add type and trait wrapping `denops-deno` and `denops-deno-std`

## 🛠️ Build

## LICENSE

This program is dual licensed by MIT and apache because of its dependencies.
See each licence also. ([`LICENSE`](https://github.com/kkiyama117/denops-rust/blob/main/LICENSE) and [`LICENSE_APACHE`](https://github.com/kkiyama117/denops-rust/blob/main/LICENSE_APACHE))

## Relations

- https://github.com/vim-denops/denops-deno
- https://github.com/vim-denops/denops-std-deno

## Info

- https://github.com/rustwasm/wasm-pack/issues/672

*/
// extern WASM calls are wrapped in unsafe,
// but they don't technically have to be.
#![deny(unused_unsafe)]

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "console")]
#[macro_use]
pub mod console;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "variable")]
pub mod variable;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "execute")]
pub mod execute;

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "autocmd")]
pub mod autocmd;

#[cfg(target_arch = "wasm32")]
pub(crate) mod denops;

use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::{IntoWasmAbi, WasmAbi};

#[wasm_bindgen(module = "https://deno.land/x/denops_std/mod.ts")]
extern {
    // https://deno.land/x/denops_std/vim/vim.ts
    pub type Vim;

    // https://deno.land/x/denops_std/vim/autocmd.ts
    pub type AutocmdHelper;

    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Vim) -> String;

    #[wasm_bindgen(static_method_of = Vim)]
    pub fn get() -> Vim;

    #[wasm_bindgen(method, catch)]
    pub async fn call(this: &Vim, func: &str, args: Box<[JsValue]>) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn cmd(this: &Vim, context: Context) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn eval(this: &Vim, context: Context) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn execute(this: &Vim, command: StrOrStrArray, context: Context) -> Result<(), JsValue>;

    #[wasm_bindgen(method, catch)]
    pub async fn autocmd(this: &Vim, group: &str, main: fn(AutocmdHelper)) -> Result<(), JsValue>;

    // #[wasm_bindgen(method, catch)]
    // pub fn register(this: &Vim, dispatcher: fn());
}

// can't use `Box<[str]>`, so need to use JsValue
pub type StrArray = Box<[JsValue]>;

pub enum StrOrStrArray {
    StrArray,
    JsValue,
}

impl IntoWasmAbi for StrOrStrArray, WasmDescribe {
    type Abi = u32;

    #[inline]
    fn into_abi(self) -> u32 {
        let ret = self.idx;
        std::mem::forget(self);
        ret
    }
}

pub type Context = JsValue;
