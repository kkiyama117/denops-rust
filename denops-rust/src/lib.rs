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

#[wasm_bindgen(module = "https://deno.land/x/denops_std/mod.ts")]
extern "C" {
    // https://deno.land/x/denops_std/vim/vim.ts
    pub type Vim;
}
