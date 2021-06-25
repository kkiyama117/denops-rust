use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "https://deno.land/x/denops/mod.ts")]
extern "C" {
    // https://deno.land/x/denops/denops.ts
    pub(crate) type Denops;
}
